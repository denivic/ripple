use rusqlite::Connection;

use super::error::DbError;

const CURRENT_VERSION: i32 = 7;

/// Versioned via `PRAGMA user_version` rather than a migrations table — the
/// whole schema lives in this file for now, so there's nothing a table would
/// track that the version integer doesn't already.
pub fn run(conn: &Connection) -> Result<(), DbError> {
    let version: i32 = conn.query_row("PRAGMA user_version", [], |row| row.get(0))?;
    if version < 1 {
        migrate_v1(conn)?;
    }
    if version < 2 {
        migrate_v2(conn)?;
    }
    if version < 3 {
        migrate_v3(conn)?;
    }
    if version < 4 {
        migrate_v4(conn)?;
    }
    if version < 5 {
        migrate_v5(conn)?;
    }
    if version < 6 {
        migrate_v6(conn)?;
    }
    if version < 7 {
        migrate_v7(conn)?;
    }
    conn.pragma_update(None, "user_version", CURRENT_VERSION)?;
    Ok(())
}

fn migrate_v1(conn: &Connection) -> Result<(), DbError> {
    conn.execute_batch(
        "
        CREATE TABLE habits (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            unit_label TEXT NOT NULL,
            life_minutes_per_unit REAL NOT NULL DEFAULT 0,
            cost_per_unit REAL NOT NULL DEFAULT 0,
            color TEXT,
            archived INTEGER NOT NULL DEFAULT 0
        );

        CREATE TABLE entries (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            habit_id INTEGER NOT NULL REFERENCES habits(id) ON DELETE CASCADE,
            occurred_at TEXT NOT NULL,
            quantity REAL NOT NULL,
            duration_minutes REAL,
            note TEXT
        );
        CREATE INDEX idx_entries_occurred_at ON entries(occurred_at);
        CREATE INDEX idx_entries_habit_id ON entries(habit_id);

        CREATE TABLE profile (
            id INTEGER PRIMARY KEY CHECK (id = 1),
            birth_date TEXT,
            sex TEXT,
            life_expectancy_years REAL,
            typical_sleep_hours REAL,
            net_hourly_income REAL,
            weight_kg REAL
        );
        ",
    )?;
    Ok(())
}

fn migrate_v2(conn: &Connection) -> Result<(), DbError> {
    conn.execute_batch(
        "
        -- Remembers the column mapping chosen for an import, keyed by a
        -- signature of the source file's header row, so re-importing a
        -- similarly-shaped file (e.g. a recurring export) can suggest the
        -- same mapping without asking again.
        CREATE TABLE import_mappings (
            source_signature TEXT PRIMARY KEY,
            mapping_json TEXT NOT NULL,
            updated_at TEXT NOT NULL
        );
        ",
    )?;
    Ok(())
}

fn migrate_v3(conn: &Connection) -> Result<(), DbError> {
    conn.execute_batch(
        "
        -- Single-row, like `profile`. `theme_name` and `custom_theme_json`
        -- are mutually exclusive in practice (see domain::Settings), but
        -- both nullable so a fresh row has an honest default.
        CREATE TABLE settings (
            id INTEGER PRIMARY KEY CHECK (id = 1),
            startup_route TEXT NOT NULL DEFAULT 'today',
            week_start TEXT NOT NULL DEFAULT 'monday',
            weight_unit TEXT NOT NULL DEFAULT 'kg',
            theme_name TEXT,
            custom_theme_json TEXT
        );
        ",
    )?;
    Ok(())
}

fn migrate_v4(conn: &Connection) -> Result<(), DbError> {
    conn.execute_batch(
        "
        -- ISO 4217 code money is displayed in. Existing rows were implicitly
        -- USD (the old hard-coded default), so the column default keeps them
        -- showing what they always showed.
        ALTER TABLE settings ADD COLUMN currency TEXT NOT NULL DEFAULT 'USD';
        ",
    )?;
    Ok(())
}

fn migrate_v5(conn: &Connection) -> Result<(), DbError> {
    conn.execute_batch(
        "
        -- Default time spent per unit, used when an entry has no duration.
        -- 0 means not time-tracked, which is what existing habits were.
        ALTER TABLE habits ADD COLUMN minutes_per_unit REAL NOT NULL DEFAULT 0;
        ",
    )?;
    Ok(())
}

fn migrate_v6(conn: &Connection) -> Result<(), DbError> {
    conn.execute_batch(
        "
        -- Points ledger. Append-only: `award_key` is unique so evaluating
        -- the same history twice inserts nothing new (INSERT OR IGNORE),
        -- and nothing ever deletes a row except a full reset.
        CREATE TABLE point_awards (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            award_key TEXT NOT NULL UNIQUE,
            kind TEXT NOT NULL,
            points INTEGER NOT NULL,
            awarded_for TEXT NOT NULL,
            detail TEXT,
            created_at TEXT NOT NULL
        );
        ",
    )?;
    Ok(())
}

fn migrate_v7(conn: &Connection) -> Result<(), DbError> {
    conn.execute_batch(
        "
        -- Per-category notification switches. Categories are stable ids
        -- (reminders|progress|weekly_summary|streak_pace); a missing row
        -- reads as the defaults below, so a fresh install needs no seeding.
        CREATE TABLE notification_prefs (
            category TEXT PRIMARY KEY,
            enabled INTEGER NOT NULL DEFAULT 1,
            channel_local INTEGER NOT NULL DEFAULT 1,
            channel_email INTEGER NOT NULL DEFAULT 0
        );

        -- Single-row, like `settings`. Times are local \"HH:MM\"; quiet
        -- hours are off when either end is NULL.
        CREATE TABLE notification_schedule (
            id INTEGER PRIMARY KEY CHECK (id = 1),
            reminder_time TEXT NOT NULL DEFAULT '20:00',
            weekly_day TEXT NOT NULL DEFAULT 'sunday',
            weekly_time TEXT NOT NULL DEFAULT '18:00',
            streak_alert_time TEXT NOT NULL DEFAULT '21:00',
            pace_threshold REAL NOT NULL DEFAULT 1.5,
            quiet_start TEXT,
            quiet_end TEXT,
            bundle_progress INTEGER NOT NULL DEFAULT 1,
            launch_at_login INTEGER NOT NULL DEFAULT 0,
            close_to_tray INTEGER NOT NULL DEFAULT 1
        );

        -- Dedup ledger: a notification is sent only if its key is absent,
        -- and the key is written after a successful send.
        CREATE TABLE notification_log (
            key TEXT PRIMARY KEY,
            sent_at TEXT NOT NULL
        );

        -- Single-row. Secrets never live here — only in the OS keychain.
        CREATE TABLE email_settings (
            id INTEGER PRIMARY KEY CHECK (id = 1),
            provider TEXT NOT NULL DEFAULT 'none',
            from_address TEXT,
            to_address TEXT,
            smtp_host TEXT,
            smtp_port INTEGER,
            smtp_username TEXT,
            smtp_security TEXT NOT NULL DEFAULT 'starttls'
        );
        ",
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn user_version(conn: &Connection) -> i32 {
        conn.query_row("PRAGMA user_version", [], |row| row.get(0))
            .unwrap()
    }

    #[test]
    fn fresh_database_reaches_current_version() {
        let conn = Connection::open_in_memory().unwrap();
        run(&conn).unwrap();
        assert_eq!(user_version(&conn), CURRENT_VERSION);
    }

    #[test]
    fn v4_adds_currency_defaulting_to_usd_on_an_existing_v3_row() {
        let conn = Connection::open_in_memory().unwrap();
        migrate_v1(&conn).unwrap();
        migrate_v2(&conn).unwrap();
        migrate_v3(&conn).unwrap();
        conn.pragma_update(None, "user_version", 3).unwrap();
        conn.execute(
            "INSERT INTO settings (id, weight_unit) VALUES (1, 'lb')",
            [],
        )
        .unwrap();

        run(&conn).unwrap();

        assert_eq!(user_version(&conn), CURRENT_VERSION);
        let (currency, weight_unit): (String, String) = conn
            .query_row(
                "SELECT currency, weight_unit FROM settings WHERE id = 1",
                [],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .unwrap();
        assert_eq!(currency, "USD");
        assert_eq!(weight_unit, "lb");
    }

    #[test]
    fn v5_adds_minutes_per_unit_defaulting_to_zero_on_an_existing_v4_row() {
        let conn = Connection::open_in_memory().unwrap();
        migrate_v1(&conn).unwrap();
        migrate_v2(&conn).unwrap();
        migrate_v3(&conn).unwrap();
        migrate_v4(&conn).unwrap();
        conn.pragma_update(None, "user_version", 4).unwrap();
        conn.execute(
            "INSERT INTO habits (name, unit_label, life_minutes_per_unit) VALUES ('Cigarettes', 'cigarette', 20)",
            [],
        )
        .unwrap();

        run(&conn).unwrap();

        assert_eq!(user_version(&conn), CURRENT_VERSION);
        let (minutes_per_unit, life_minutes): (f64, f64) = conn
            .query_row(
                "SELECT minutes_per_unit, life_minutes_per_unit FROM habits WHERE name = 'Cigarettes'",
                [],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .unwrap();
        assert_eq!(minutes_per_unit, 0.0);
        assert_eq!(life_minutes, 20.0);
    }

    #[test]
    fn v6_adds_an_empty_point_awards_ledger_keeping_existing_data() {
        let conn = Connection::open_in_memory().unwrap();
        migrate_v1(&conn).unwrap();
        migrate_v2(&conn).unwrap();
        migrate_v3(&conn).unwrap();
        migrate_v4(&conn).unwrap();
        migrate_v5(&conn).unwrap();
        conn.pragma_update(None, "user_version", 5).unwrap();
        conn.execute(
            "INSERT INTO habits (name, unit_label) VALUES ('Coffee', 'cup')",
            [],
        )
        .unwrap();

        run(&conn).unwrap();

        assert_eq!(user_version(&conn), CURRENT_VERSION);
        let habits: i64 = conn
            .query_row("SELECT COUNT(*) FROM habits", [], |row| row.get(0))
            .unwrap();
        assert_eq!(habits, 1);
        let awards: i64 = conn
            .query_row("SELECT COUNT(*) FROM point_awards", [], |row| row.get(0))
            .unwrap();
        assert_eq!(awards, 0);

        let insert = "INSERT OR IGNORE INTO point_awards
             (award_key, kind, points, awarded_for, created_at)
             VALUES ('clean:2026-10-20', 'clean', 10, '2026-10-20', '2026-10-21T08:00:00')";
        assert_eq!(conn.execute(insert, []).unwrap(), 1);
        assert_eq!(conn.execute(insert, []).unwrap(), 0);
    }

    #[test]
    fn v7_adds_empty_notification_tables_keeping_existing_data() {
        let conn = Connection::open_in_memory().unwrap();
        migrate_v1(&conn).unwrap();
        migrate_v2(&conn).unwrap();
        migrate_v3(&conn).unwrap();
        migrate_v4(&conn).unwrap();
        migrate_v5(&conn).unwrap();
        migrate_v6(&conn).unwrap();
        conn.pragma_update(None, "user_version", 6).unwrap();
        conn.execute(
            "INSERT INTO habits (name, unit_label) VALUES ('Coffee', 'cup')",
            [],
        )
        .unwrap();

        run(&conn).unwrap();

        assert_eq!(user_version(&conn), 7);
        let habits: i64 = conn
            .query_row("SELECT COUNT(*) FROM habits", [], |row| row.get(0))
            .unwrap();
        assert_eq!(habits, 1);
        for table in [
            "notification_prefs",
            "notification_schedule",
            "notification_log",
            "email_settings",
        ] {
            let n: i64 = conn
                .query_row(&format!("SELECT COUNT(*) FROM {table}"), [], |row| row.get(0))
                .unwrap();
            assert_eq!(n, 0, "{table} should start empty");
        }

        // Column defaults match the spec'd defaults.
        conn.execute("INSERT INTO notification_schedule (id) VALUES (1)", [])
            .unwrap();
        let (reminder, day, threshold, quiet): (String, String, f64, Option<String>) = conn
            .query_row(
                "SELECT reminder_time, weekly_day, pace_threshold, quiet_start
                 FROM notification_schedule WHERE id = 1",
                [],
                |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?)),
            )
            .unwrap();
        assert_eq!(reminder, "20:00");
        assert_eq!(day, "sunday");
        assert_eq!(threshold, 1.5);
        assert_eq!(quiet, None);
        conn.execute("INSERT INTO email_settings (id) VALUES (1)", [])
            .unwrap();
        let provider: String = conn
            .query_row("SELECT provider FROM email_settings", [], |row| row.get(0))
            .unwrap();
        assert_eq!(provider, "none");
    }
}
