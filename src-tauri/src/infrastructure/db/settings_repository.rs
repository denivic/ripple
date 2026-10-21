use std::sync::Arc;

use rusqlite::{params, OptionalExtension};

use crate::domain::currency::default_currency;
use crate::domain::repository::{RepoResult, SettingsRepository};
use crate::domain::settings::{
    is_currency_code, Settings, StartupRoute, TimeFormat, WeekStart, WeightUnit,
};

use super::Db;

pub struct SqliteSettingsRepository {
    db: Arc<Db>,
}

impl SqliteSettingsRepository {
    pub fn new(db: Arc<Db>) -> Self {
        Self { db }
    }
}

fn startup_route_to_str(r: StartupRoute) -> &'static str {
    match r {
        StartupRoute::Today => "today",
        StartupRoute::Timeline => "timeline",
        StartupRoute::Ledger => "ledger",
        StartupRoute::Profile => "profile",
        StartupRoute::Settings => "settings",
    }
}

fn startup_route_from_str(s: &str) -> Option<StartupRoute> {
    match s {
        "today" => Some(StartupRoute::Today),
        "timeline" => Some(StartupRoute::Timeline),
        "ledger" => Some(StartupRoute::Ledger),
        "profile" => Some(StartupRoute::Profile),
        "settings" => Some(StartupRoute::Settings),
        _ => None,
    }
}

fn week_start_to_str(w: WeekStart) -> &'static str {
    match w {
        WeekStart::Monday => "monday",
        WeekStart::Sunday => "sunday",
    }
}

fn week_start_from_str(s: &str) -> Option<WeekStart> {
    match s {
        "monday" => Some(WeekStart::Monday),
        "sunday" => Some(WeekStart::Sunday),
        _ => None,
    }
}

fn weight_unit_to_str(u: WeightUnit) -> &'static str {
    match u {
        WeightUnit::Kg => "kg",
        WeightUnit::Lb => "lb",
    }
}

fn weight_unit_from_str(s: &str) -> Option<WeightUnit> {
    match s {
        "kg" => Some(WeightUnit::Kg),
        "lb" => Some(WeightUnit::Lb),
        _ => None,
    }
}

fn time_format_to_str(t: TimeFormat) -> &'static str {
    match t {
        TimeFormat::H24 => "24h",
        TimeFormat::H12 => "12h",
    }
}

fn time_format_from_str(s: &str) -> Option<TimeFormat> {
    match s {
        "24h" => Some(TimeFormat::H24),
        "12h" => Some(TimeFormat::H12),
        _ => None,
    }
}

impl SettingsRepository for SqliteSettingsRepository {
    fn get(&self) -> RepoResult<Settings> {
        let conn = self.db.lock()?;
        let settings = conn
            .query_row(
                "SELECT startup_route, week_start, weight_unit, theme_name, custom_theme_json, currency, time_format
                 FROM settings WHERE id = 1",
                [],
                |row| {
                    let startup_route: String = row.get(0)?;
                    let week_start: String = row.get(1)?;
                    let weight_unit: String = row.get(2)?;
                    let currency: String = row.get(5)?;
                    let time_format: String = row.get(6)?;
                    Ok(Settings {
                        startup_route: startup_route_from_str(&startup_route)
                            .unwrap_or(StartupRoute::Today),
                        week_start: week_start_from_str(&week_start).unwrap_or(WeekStart::Monday),
                        weight_unit: weight_unit_from_str(&weight_unit).unwrap_or(WeightUnit::Kg),
                        time_format: time_format_from_str(&time_format).unwrap_or(TimeFormat::H24),
                        theme_name: row.get(3)?,
                        custom_theme_json: row.get(4)?,
                        currency: if is_currency_code(&currency) {
                            currency
                        } else {
                            default_currency().to_string()
                        },
                    })
                },
            )
            .optional()?;
        Ok(settings.unwrap_or_default())
    }

    fn save(&self, settings: &Settings) -> RepoResult<()> {
        let conn = self.db.lock()?;
        conn.execute(
            "INSERT INTO settings (id, startup_route, week_start, weight_unit, theme_name, custom_theme_json, currency, time_format)
             VALUES (1, ?1, ?2, ?3, ?4, ?5, ?6, ?7)
             ON CONFLICT(id) DO UPDATE SET
               startup_route = excluded.startup_route,
               week_start = excluded.week_start,
               weight_unit = excluded.weight_unit,
               theme_name = excluded.theme_name,
               custom_theme_json = excluded.custom_theme_json,
               currency = excluded.currency,
               time_format = excluded.time_format",
            params![
                startup_route_to_str(settings.startup_route),
                week_start_to_str(settings.week_start),
                weight_unit_to_str(settings.weight_unit),
                settings.theme_name,
                settings.custom_theme_json,
                settings.currency,
                time_format_to_str(settings.time_format),
            ],
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn repo() -> SqliteSettingsRepository {
        SqliteSettingsRepository::new(Arc::new(Db::open_in_memory().unwrap()))
    }

    #[test]
    fn get_before_any_save_returns_default() {
        let repo = repo();
        assert_eq!(repo.get().unwrap(), Settings::default());
    }

    #[test]
    fn save_then_get_round_trips() {
        let repo = repo();
        let settings = Settings {
            startup_route: StartupRoute::Ledger,
            week_start: WeekStart::Sunday,
            weight_unit: WeightUnit::Lb,
            time_format: TimeFormat::H12,
            theme_name: None,
            custom_theme_json: Some(r#"{"schemaVersion":1}"#.to_string()),
            currency: "EUR".to_string(),
        };
        repo.save(&settings).unwrap();
        assert_eq!(repo.get().unwrap(), settings);
    }

    #[test]
    fn save_twice_upserts_the_single_row() {
        let repo = repo();
        repo.save(&Settings {
            theme_name: Some("Ember".to_string()),
            ..Default::default()
        })
        .unwrap();
        repo.save(&Settings {
            theme_name: Some("Deep Current".to_string()),
            ..Default::default()
        })
        .unwrap();
        assert_eq!(
            repo.get().unwrap().theme_name,
            Some("Deep Current".to_string())
        );
    }

    #[test]
    fn malformed_stored_currency_falls_back_to_default() {
        let repo = repo();
        repo.save(&Settings::default()).unwrap();
        repo.db
            .lock()
            .unwrap()
            .execute("UPDATE settings SET currency = 'nope' WHERE id = 1", [])
            .unwrap();
        assert_eq!(repo.get().unwrap().currency, default_currency());
    }

    #[test]
    fn time_format_round_trips_both_ways() {
        let repo = repo();
        for time_format in [TimeFormat::H12, TimeFormat::H24] {
            repo.save(&Settings {
                time_format,
                ..Default::default()
            })
            .unwrap();
            assert_eq!(repo.get().unwrap().time_format, time_format);
        }
    }

    #[test]
    fn malformed_stored_time_format_falls_back_to_24h() {
        let repo = repo();
        repo.save(&Settings::default()).unwrap();
        repo.db
            .lock()
            .unwrap()
            .execute("UPDATE settings SET time_format = 'nope' WHERE id = 1", [])
            .unwrap();
        assert_eq!(repo.get().unwrap().time_format, TimeFormat::H24);
    }
}
