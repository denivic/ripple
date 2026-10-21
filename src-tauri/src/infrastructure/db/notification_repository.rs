use std::sync::Arc;

use rusqlite::{params, OptionalExtension};
use time::OffsetDateTime;

use crate::domain::notifications::{
    format_clock, parse_clock, weekday_from_str, weekday_to_str, Category, CategoryPrefs,
    NotificationPrefs, NotificationSchedule, QuietHours,
};
use crate::domain::repository::{
    NotificationLogRepository, NotificationSettingsRepository, RepoResult,
};

use super::codec::format_datetime;
use super::Db;

/// Prefs, schedule and the dedup log in one adapter: they're one feature's
/// tables and always travel together.
pub struct SqliteNotificationRepository {
    db: Arc<Db>,
}

impl SqliteNotificationRepository {
    pub fn new(db: Arc<Db>) -> Self {
        Self { db }
    }
}

impl NotificationSettingsRepository for SqliteNotificationRepository {
    fn get_prefs(&self) -> RepoResult<NotificationPrefs> {
        let conn = self.db.lock()?;
        let mut stmt = conn.prepare(
            "SELECT category, enabled, channel_local, channel_email FROM notification_prefs",
        )?;
        let rows = stmt
            .query_map([], |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    CategoryPrefs {
                        enabled: row.get(1)?,
                        local: row.get(2)?,
                        email: row.get(3)?,
                    },
                ))
            })?
            .collect::<rusqlite::Result<Vec<_>>>()?;
        // Categories without a row keep the default; unknown ids are ignored.
        let mut prefs = NotificationPrefs::default();
        for (id, p) in rows {
            if let Some(category) = Category::from_id(&id) {
                prefs.set(category, p);
            }
        }
        Ok(prefs)
    }

    fn save_prefs(&self, prefs: &NotificationPrefs) -> RepoResult<()> {
        let mut conn = self.db.lock()?;
        let tx = conn.transaction()?;
        for category in Category::ALL {
            let p = prefs.get(category);
            tx.execute(
                "INSERT INTO notification_prefs (category, enabled, channel_local, channel_email)
                 VALUES (?1, ?2, ?3, ?4)
                 ON CONFLICT(category) DO UPDATE SET
                   enabled = excluded.enabled,
                   channel_local = excluded.channel_local,
                   channel_email = excluded.channel_email",
                params![category.id(), p.enabled, p.local, p.email],
            )?;
        }
        tx.commit()?;
        Ok(())
    }

    fn get_schedule(&self) -> RepoResult<NotificationSchedule> {
        let conn = self.db.lock()?;
        let schedule = conn
            .query_row(
                "SELECT reminder_time, weekly_day, weekly_time, streak_alert_time, pace_threshold,
                        quiet_start, quiet_end, bundle_progress, launch_at_login, close_to_tray
                 FROM notification_schedule WHERE id = 1",
                [],
                |row| {
                    let defaults = NotificationSchedule::default();
                    let clock = |i: usize, fallback| -> rusqlite::Result<_> {
                        Ok(parse_clock(&row.get::<_, String>(i)?).unwrap_or(fallback))
                    };
                    let quiet_start: Option<String> = row.get(5)?;
                    let quiet_end: Option<String> = row.get(6)?;
                    let threshold: f64 = row.get(4)?;
                    Ok(NotificationSchedule {
                        reminder_time: clock(0, defaults.reminder_time)?,
                        weekly_day: weekday_from_str(&row.get::<_, String>(1)?)
                            .unwrap_or(defaults.weekly_day),
                        weekly_time: clock(2, defaults.weekly_time)?,
                        streak_alert_time: clock(3, defaults.streak_alert_time)?,
                        pace_threshold: if threshold.is_finite() && threshold > 0.0 {
                            threshold
                        } else {
                            defaults.pace_threshold
                        },
                        // Off unless both ends parse.
                        quiet_hours: match (
                            quiet_start.as_deref().and_then(parse_clock),
                            quiet_end.as_deref().and_then(parse_clock),
                        ) {
                            (Some(start), Some(end)) => Some(QuietHours { start, end }),
                            _ => None,
                        },
                        bundle_progress: row.get(7)?,
                        launch_at_login: row.get(8)?,
                        close_to_tray: row.get(9)?,
                    })
                },
            )
            .optional()?;
        Ok(schedule.unwrap_or_default())
    }

    fn save_schedule(&self, s: &NotificationSchedule) -> RepoResult<()> {
        let conn = self.db.lock()?;
        conn.execute(
            "INSERT INTO notification_schedule
               (id, reminder_time, weekly_day, weekly_time, streak_alert_time, pace_threshold,
                quiet_start, quiet_end, bundle_progress, launch_at_login, close_to_tray)
             VALUES (1, ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)
             ON CONFLICT(id) DO UPDATE SET
               reminder_time = excluded.reminder_time,
               weekly_day = excluded.weekly_day,
               weekly_time = excluded.weekly_time,
               streak_alert_time = excluded.streak_alert_time,
               pace_threshold = excluded.pace_threshold,
               quiet_start = excluded.quiet_start,
               quiet_end = excluded.quiet_end,
               bundle_progress = excluded.bundle_progress,
               launch_at_login = excluded.launch_at_login,
               close_to_tray = excluded.close_to_tray",
            params![
                format_clock(s.reminder_time),
                weekday_to_str(s.weekly_day),
                format_clock(s.weekly_time),
                format_clock(s.streak_alert_time),
                s.pace_threshold,
                s.quiet_hours.map(|q| format_clock(q.start)),
                s.quiet_hours.map(|q| format_clock(q.end)),
                s.bundle_progress,
                s.launch_at_login,
                s.close_to_tray,
            ],
        )?;
        Ok(())
    }
}

impl NotificationLogRepository for SqliteNotificationRepository {
    fn is_sent(&self, key: &str) -> RepoResult<bool> {
        let conn = self.db.lock()?;
        let found: Option<i64> = conn
            .query_row(
                "SELECT 1 FROM notification_log WHERE key = ?1",
                params![key],
                |row| row.get(0),
            )
            .optional()?;
        Ok(found.is_some())
    }

    /// `sent_at` is UTC bookkeeping, like `point_awards.created_at`.
    fn mark_sent(&self, key: &str) -> RepoResult<()> {
        let now = OffsetDateTime::now_utc();
        let sent_at = format_datetime(time::PrimitiveDateTime::new(now.date(), now.time()));
        let conn = self.db.lock()?;
        conn.execute(
            "INSERT OR IGNORE INTO notification_log (key, sent_at) VALUES (?1, ?2)",
            params![key, sent_at],
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::macros::time;
    use time::Weekday;

    fn repo() -> SqliteNotificationRepository {
        SqliteNotificationRepository::new(Arc::new(Db::open_in_memory().unwrap()))
    }

    #[test]
    fn get_before_any_save_returns_defaults() {
        let repo = repo();
        assert_eq!(repo.get_prefs().unwrap(), NotificationPrefs::default());
        assert_eq!(repo.get_schedule().unwrap(), NotificationSchedule::default());
    }

    #[test]
    fn prefs_round_trip() {
        let repo = repo();
        let mut prefs = NotificationPrefs::default();
        prefs.weekly_summary = CategoryPrefs {
            enabled: true,
            local: false,
            email: true,
        };
        prefs.progress.enabled = false;
        repo.save_prefs(&prefs).unwrap();
        assert_eq!(repo.get_prefs().unwrap(), prefs);
        // Saving again upserts rather than duplicating.
        repo.save_prefs(&prefs).unwrap();
        assert_eq!(repo.get_prefs().unwrap(), prefs);
    }

    #[test]
    fn schedule_round_trips_including_quiet_hours() {
        let repo = repo();
        let schedule = NotificationSchedule {
            reminder_time: time!(07:45),
            weekly_day: Weekday::Friday,
            weekly_time: time!(17:30),
            streak_alert_time: time!(22:10),
            pace_threshold: 2.25,
            quiet_hours: Some(QuietHours {
                start: time!(22:00),
                end: time!(06:30),
            }),
            bundle_progress: false,
            launch_at_login: true,
            close_to_tray: false,
        };
        repo.save_schedule(&schedule).unwrap();
        assert_eq!(repo.get_schedule().unwrap(), schedule);

        let off = NotificationSchedule {
            quiet_hours: None,
            ..schedule
        };
        repo.save_schedule(&off).unwrap();
        assert_eq!(repo.get_schedule().unwrap(), off);
    }

    #[test]
    fn log_records_each_key_once() {
        let repo = repo();
        assert!(!repo.is_sent("reminders:2026-10-21:local").unwrap());
        repo.mark_sent("reminders:2026-10-21:local").unwrap();
        repo.mark_sent("reminders:2026-10-21:local").unwrap();
        assert!(repo.is_sent("reminders:2026-10-21:local").unwrap());
        assert!(!repo.is_sent("reminders:2026-10-21:email").unwrap());
    }
}
