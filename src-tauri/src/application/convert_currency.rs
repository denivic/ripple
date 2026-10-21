use crate::domain::currency::{ExchangeRateSource, RateError};
use crate::domain::is_currency_code;
use crate::domain::repository::CurrencyConversionRepository;

#[derive(Debug, Clone, PartialEq)]
pub struct CurrencyConversion {
    /// How many `to` one `from` was worth — the factor actually applied.
    pub rate: f64,
    pub converted_habits: usize,
}

fn validate(from: &str, to: &str) -> Result<(), String> {
    for code in [from, to] {
        if !is_currency_code(code) {
            return Err(format!("invalid currency code '{code}'"));
        }
    }
    Ok(())
}

/// The preview half: what the Settings dialog shows before the user commits.
pub fn fetch_rate(source: &dyn ExchangeRateSource, from: &str, to: &str) -> Result<f64, String> {
    validate(from, to)?;
    if from == to {
        return Ok(1.0);
    }
    source.rate(from, to).map_err(|e: RateError| e.to_string())
}

/// Fetches today's rate and re-denominates every stored amount with it. The
/// rate is fetched afresh rather than taken from the caller, so a stale or
/// tampered preview can never be what gets applied.
pub fn convert_currency(
    source: &dyn ExchangeRateSource,
    repo: &dyn CurrencyConversionRepository,
    from: &str,
    to: &str,
) -> Result<CurrencyConversion, String> {
    let rate = fetch_rate(source, from, to)?;
    let converted_habits = repo
        .convert_amounts(from, to, rate)
        .map_err(|e| e.to_string())?;
    Ok(CurrencyConversion {
        rate,
        converted_habits,
    })
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;
    use crate::domain::repository::{HabitRepository, ProfileRepository, SettingsRepository};
    use crate::domain::{Habit, Profile};
    use crate::infrastructure::db::{
        Db, SqliteCurrencyConversionRepository, SqliteHabitRepository, SqliteProfileRepository,
        SqliteSettingsRepository,
    };

    /// Fake rate source: a fixed answer (or error) and no network.
    struct FakeRates(Result<f64, RateError>);

    impl ExchangeRateSource for FakeRates {
        fn rate(&self, _from: &str, _to: &str) -> Result<f64, RateError> {
            self.0.clone()
        }
    }

    fn seeded() -> Arc<Db> {
        let db = Arc::new(Db::open_in_memory().unwrap());
        let mut h = Habit::new("Coffee", "cup");
        h.cost_per_unit = 4.0;
        SqliteHabitRepository::new(Arc::clone(&db))
            .insert(&h)
            .unwrap();
        SqliteProfileRepository::new(Arc::clone(&db))
            .save(&Profile {
                net_hourly_income: Some(30.0),
                ..Default::default()
            })
            .unwrap();
        db
    }

    #[test]
    fn applies_the_fetched_rate_to_habits_and_profile() {
        let db = seeded();
        let repo = SqliteCurrencyConversionRepository::new(Arc::clone(&db));

        let result = convert_currency(&FakeRates(Ok(1.25)), &repo, "USD", "EUR").unwrap();

        assert_eq!(
            result,
            CurrencyConversion {
                rate: 1.25,
                converted_habits: 1
            }
        );
        let habits = SqliteHabitRepository::new(Arc::clone(&db))
            .list(true)
            .unwrap();
        assert_eq!(habits[0].cost_per_unit, 5.0);
        let profile = SqliteProfileRepository::new(Arc::clone(&db)).get().unwrap();
        assert_eq!(profile.net_hourly_income, Some(37.5));
        let settings = SqliteSettingsRepository::new(db).get().unwrap();
        assert_eq!(settings.currency, "EUR");
    }

    #[test]
    fn a_rate_error_changes_nothing() {
        let db = seeded();
        let repo = SqliteCurrencyConversionRepository::new(Arc::clone(&db));
        let err = convert_currency(
            &FakeRates(Err(RateError::Unsupported {
                from: "USD".into(),
                to: "ARS".into(),
            })),
            &repo,
            "USD",
            "ARS",
        )
        .unwrap_err();

        assert!(err.contains("USD → ARS"), "{err}");
        let habits = SqliteHabitRepository::new(Arc::clone(&db))
            .list(true)
            .unwrap();
        assert_eq!(habits[0].cost_per_unit, 4.0);
        assert_eq!(
            SqliteSettingsRepository::new(db).get().unwrap().currency,
            "USD"
        );
    }

    #[test]
    fn network_errors_read_clearly() {
        let err = fetch_rate(
            &FakeRates(Err(RateError::Network("timed out".into()))),
            "USD",
            "EUR",
        )
        .unwrap_err();
        assert_eq!(err, "Couldn't reach the exchange-rate service: timed out");
    }

    #[test]
    fn same_currency_is_rate_one_without_asking_the_source() {
        let source = FakeRates(Err(RateError::Network("should not be called".into())));
        assert_eq!(fetch_rate(&source, "EUR", "EUR"), Ok(1.0));
    }

    #[test]
    fn malformed_codes_are_rejected_before_any_fetch() {
        let err = fetch_rate(&FakeRates(Ok(1.0)), "usd", "EUR").unwrap_err();
        assert_eq!(err, "invalid currency code 'usd'");
    }
}
