use rusqlite::Connection;

use super::error::DbError;

const CURRENT_VERSION: i32 = 1;

/// Versioned via `PRAGMA user_version` rather than a migrations table — the
/// whole schema lives in this file for now, so there's nothing a table would
/// track that the version integer doesn't already.
pub fn run(conn: &Connection) -> Result<(), DbError> {
    let version: i32 = conn.query_row("PRAGMA user_version", [], |row| row.get(0))?;
    if version < 1 {
        migrate_v1(conn)?;
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
