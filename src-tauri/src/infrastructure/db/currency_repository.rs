use std::sync::Arc;

use rusqlite::{params, OptionalExtension};

use crate::domain::repository::{CurrencyConversionRepository, RepoResult, RepositoryError};
use crate::domain::settings::DEFAULT_CURRENCY;

use super::Db;

pub struct SqliteCurrencyConversionRepository {
    db: Arc<Db>,
}

impl SqliteCurrencyConversionRepository {
    pub fn new(db: Arc<Db>) -> Self {
        Self { db }
    }
}

impl CurrencyConversionRepository for SqliteCurrencyConversionRepository {
    fn convert_amounts(&self, from: &str, to: &str, rate: f64) -> RepoResult<usize> {
        let mut conn = self.db.lock()?;
        // Dropping `tx` without commit() rolls back, so every early return
        // below — including a failed statement — leaves the data untouched.
        let tx = conn.transaction()?;

        // No settings row yet means the default currency is in effect.
        let current: String = tx
            .query_row("SELECT currency FROM settings WHERE id = 1", [], |row| {
                row.get(0)
            })
            .optional()?
            .unwrap_or_else(|| DEFAULT_CURRENCY.to_string());
        if current != from {
            return Err(RepositoryError(format!(
                "amounts are stored in {current}, not {from}; reload Settings and try again"
            )));
        }

        // Rounded to cents: money is displayed to two decimals, and an
        // unrounded product (3.3645000001) would surface in the Ledger grid.
        let converted_habits = tx.execute(
            "UPDATE habits SET cost_per_unit = ROUND(cost_per_unit * ?1, 2)",
            params![rate],
        )?;
        // NULL stays NULL: an unset income isn't a zero income.
        tx.execute(
            "UPDATE profile SET net_hourly_income = ROUND(net_hourly_income * ?1, 2)",
            params![rate],
        )?;
        tx.execute(
            "INSERT INTO settings (id, currency) VALUES (1, ?1)
             ON CONFLICT(id) DO UPDATE SET currency = excluded.currency",
            params![to],
        )?;

        tx.commit()?;
        Ok(converted_habits)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::repository::{HabitRepository, ProfileRepository, SettingsRepository};
    use crate::domain::{Habit, Profile, Settings};
    use crate::infrastructure::db::{
        SqliteHabitRepository, SqliteProfileRepository, SqliteSettingsRepository,
    };

    struct Fixture {
        db: Arc<Db>,
        habits: SqliteHabitRepository,
        profile: SqliteProfileRepository,
        settings: SqliteSettingsRepository,
        repo: SqliteCurrencyConversionRepository,
    }

    fn fixture() -> Fixture {
        let db = Arc::new(Db::open_in_memory().unwrap());
        Fixture {
            habits: SqliteHabitRepository::new(Arc::clone(&db)),
            profile: SqliteProfileRepository::new(Arc::clone(&db)),
            settings: SqliteSettingsRepository::new(Arc::clone(&db)),
            repo: SqliteCurrencyConversionRepository::new(Arc::clone(&db)),
            db,
        }
    }

    fn habit(name: &str, cost: f64) -> Habit {
        let mut h = Habit::new(name.to_string(), "unit".to_string());
        h.cost_per_unit = cost;
        h
    }

    fn costs(f: &Fixture) -> Vec<f64> {
        f.habits
            .list(true)
            .unwrap()
            .into_iter()
            .map(|h| h.cost_per_unit)
            .collect()
    }

    #[test]
    fn converts_habits_profile_and_currency_together() {
        let f = fixture();
        f.habits.insert(&habit("Cigarettes", 0.5)).unwrap();
        f.habits.insert(&habit("Coffee", 3.2)).unwrap();
        f.profile
            .save(&Profile {
                net_hourly_income: Some(20.0),
                ..Default::default()
            })
            .unwrap();

        let n = f.repo.convert_amounts("USD", "EUR", 0.9).unwrap();

        assert_eq!(n, 2);
        assert_eq!(costs(&f), vec![0.45, 2.88]);
        assert_eq!(f.profile.get().unwrap().net_hourly_income, Some(18.0));
        assert_eq!(f.settings.get().unwrap().currency, "EUR");
    }

    #[test]
    fn archived_habits_are_converted_too() {
        let f = fixture();
        let mut archived = habit("Old", 10.0);
        archived.archived = true;
        f.habits.insert(&archived).unwrap();

        assert_eq!(f.repo.convert_amounts("USD", "GBP", 0.5).unwrap(), 1);
        assert_eq!(costs(&f), vec![5.0]);
    }

    #[test]
    fn unset_income_stays_unset() {
        let f = fixture();
        f.profile.save(&Profile::default()).unwrap();
        f.repo.convert_amounts("USD", "EUR", 0.9).unwrap();
        assert_eq!(f.profile.get().unwrap().net_hourly_income, None);
    }

    #[test]
    fn keeps_other_settings_when_creating_the_currency() {
        let f = fixture();
        f.settings
            .save(&Settings {
                theme_name: Some("Ember".to_string()),
                ..Default::default()
            })
            .unwrap();
        f.repo.convert_amounts("USD", "JPY", 150.0).unwrap();
        let s = f.settings.get().unwrap();
        assert_eq!(s.currency, "JPY");
        assert_eq!(s.theme_name, Some("Ember".to_string()));
    }

    #[test]
    fn refuses_when_stored_currency_is_not_from() {
        let f = fixture();
        f.habits.insert(&habit("Coffee", 3.0)).unwrap();
        let err = f.repo.convert_amounts("EUR", "USD", 1.1).unwrap_err();
        assert!(err.0.contains("stored in USD"), "{}", err.0);
        assert_eq!(costs(&f), vec![3.0]);
        assert_eq!(f.settings.get().unwrap().currency, "USD");
    }

    #[test]
    fn a_failure_mid_transaction_rolls_everything_back() {
        let f = fixture();
        f.habits.insert(&habit("Coffee", 3.0)).unwrap();
        f.profile
            .save(&Profile {
                net_hourly_income: Some(20.0),
                ..Default::default()
            })
            .unwrap();
        // Fails the profile UPDATE, which runs after the habits UPDATE has
        // already succeeded inside the transaction.
        f.db.lock()
            .unwrap()
            .execute_batch(
                "CREATE TRIGGER fail_profile BEFORE UPDATE ON profile
                 BEGIN SELECT RAISE(ABORT, 'boom'); END;",
            )
            .unwrap();

        assert!(f.repo.convert_amounts("USD", "EUR", 0.9).is_err());

        assert_eq!(costs(&f), vec![3.0]);
        assert_eq!(f.profile.get().unwrap().net_hourly_income, Some(20.0));
        assert_eq!(f.settings.get().unwrap().currency, "USD");
    }
}
