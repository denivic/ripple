use std::sync::Arc;

use rusqlite::{params, OptionalExtension};

use crate::domain::profile::{Profile, Sex};
use crate::domain::repository::{ProfileRepository, RepoResult};

use super::codec::{format_date, parse_date};
use super::Db;

pub struct SqliteProfileRepository {
    db: Arc<Db>,
}

impl SqliteProfileRepository {
    pub fn new(db: Arc<Db>) -> Self {
        Self { db }
    }
}

fn sex_to_str(sex: Sex) -> &'static str {
    match sex {
        Sex::Male => "male",
        Sex::Female => "female",
    }
}

fn sex_from_str(s: &str) -> Option<Sex> {
    match s {
        "male" => Some(Sex::Male),
        "female" => Some(Sex::Female),
        _ => None,
    }
}

impl ProfileRepository for SqliteProfileRepository {
    fn get(&self) -> RepoResult<Profile> {
        let conn = self.db.lock()?;
        let profile = conn
            .query_row(
                "SELECT birth_date, sex, life_expectancy_years, typical_sleep_hours, net_hourly_income, weight_kg
                 FROM profile WHERE id = 1",
                [],
                |row| {
                    let birth_date: Option<String> = row.get(0)?;
                    let sex: Option<String> = row.get(1)?;
                    Ok(Profile {
                        birth_date: birth_date.and_then(|s| parse_date(&s).ok()),
                        sex: sex.and_then(|s| sex_from_str(&s)),
                        life_expectancy_years: row.get(2)?,
                        typical_sleep_hours: row.get(3)?,
                        net_hourly_income: row.get(4)?,
                        weight_kg: row.get(5)?,
                    })
                },
            )
            .optional()?;
        Ok(profile.unwrap_or_default())
    }

    fn save(&self, profile: &Profile) -> RepoResult<()> {
        let conn = self.db.lock()?;
        conn.execute(
            "INSERT INTO profile (id, birth_date, sex, life_expectancy_years, typical_sleep_hours, net_hourly_income, weight_kg)
             VALUES (1, ?1, ?2, ?3, ?4, ?5, ?6)
             ON CONFLICT(id) DO UPDATE SET
               birth_date = excluded.birth_date,
               sex = excluded.sex,
               life_expectancy_years = excluded.life_expectancy_years,
               typical_sleep_hours = excluded.typical_sleep_hours,
               net_hourly_income = excluded.net_hourly_income,
               weight_kg = excluded.weight_kg",
            params![
                profile.birth_date.map(format_date),
                profile.sex.map(sex_to_str),
                profile.life_expectancy_years,
                profile.typical_sleep_hours,
                profile.net_hourly_income,
                profile.weight_kg,
            ],
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::macros::date;

    fn repo() -> SqliteProfileRepository {
        SqliteProfileRepository::new(Arc::new(Db::open_in_memory().unwrap()))
    }

    #[test]
    fn get_before_any_save_returns_default() {
        let repo = repo();
        assert_eq!(repo.get().unwrap(), Profile::default());
    }

    #[test]
    fn save_then_get_round_trips() {
        let repo = repo();
        let profile = Profile {
            birth_date: Some(date!(1994 - 06 - 12)),
            sex: Some(Sex::Female),
            life_expectancy_years: Some(83.0),
            typical_sleep_hours: Some(7.5),
            net_hourly_income: Some(28.0),
            weight_kg: Some(63.0),
        };
        repo.save(&profile).unwrap();
        assert_eq!(repo.get().unwrap(), profile);
    }

    #[test]
    fn save_twice_upserts_the_single_row() {
        let repo = repo();
        repo.save(&Profile {
            weight_kg: Some(70.0),
            ..Default::default()
        })
        .unwrap();
        repo.save(&Profile {
            weight_kg: Some(72.0),
            ..Default::default()
        })
        .unwrap();
        assert_eq!(repo.get().unwrap().weight_kg, Some(72.0));
    }
}
