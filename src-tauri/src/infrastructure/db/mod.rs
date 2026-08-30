pub(crate) mod codec;
mod entry_repository;
mod error;
mod habit_repository;
mod mapping_repository;
mod migrations;
mod profile_repository;

use std::path::Path;
use std::sync::{Mutex, MutexGuard};

use rusqlite::Connection;

pub use entry_repository::SqliteEntryRepository;
pub use error::DbError;
pub use habit_repository::SqliteHabitRepository;
pub use mapping_repository::SqliteMappingRepository;
pub use profile_repository::SqliteProfileRepository;

use crate::domain::repository::{RepoResult, RepositoryError};

pub struct Db {
    conn: Mutex<Connection>,
}

impl Db {
    pub fn open(path: impl AsRef<Path>) -> Result<Self, DbError> {
        let conn = Connection::open(path)?;
        Self::from_connection(conn)
    }

    /// Dev/test-only: no on-disk file, so nothing to point `Db::open` at.
    #[allow(dead_code)]
    pub fn open_in_memory() -> Result<Self, DbError> {
        let conn = Connection::open_in_memory()?;
        Self::from_connection(conn)
    }

    fn from_connection(conn: Connection) -> Result<Self, DbError> {
        conn.pragma_update(None, "foreign_keys", true)?;
        migrations::run(&conn)?;
        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    fn lock(&self) -> RepoResult<MutexGuard<'_, Connection>> {
        self.conn
            .lock()
            .map_err(|_| RepositoryError("database mutex poisoned".to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn migrations_are_idempotent() {
        let db = Db::open_in_memory().unwrap();
        // Reopening a fresh connection against the same file would replay
        // `run`; here we just call it again directly on the same connection.
        let conn = db.conn.lock().unwrap();
        assert!(migrations::run(&conn).is_ok());
    }

    #[test]
    fn open_creates_a_file_backed_database() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("ripple.sqlite");
        let db = Db::open(&path).unwrap();
        drop(db);
        assert!(path.exists());
    }
}
