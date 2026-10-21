use rusqlite::Connection;

use super::error::DbError;

const CURRENT_VERSION: i32 = 4;

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

        assert_eq!(user_version(&conn), 4);
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
}
