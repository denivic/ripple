pub(crate) mod codec;
mod currency_repository;
mod entry_repository;
mod error;
mod habit_repository;
mod mapping_repository;
mod migrations;
mod profile_repository;
mod settings_repository;

use std::path::Path;
use std::sync::{Mutex, MutexGuard};

use rusqlite::Connection;

pub use currency_repository::SqliteCurrencyConversionRepository;
pub use entry_repository::SqliteEntryRepository;
pub use error::DbError;
pub use habit_repository::SqliteHabitRepository;
pub use mapping_repository::SqliteMappingRepository;
pub use profile_repository::SqliteProfileRepository;
pub use settings_repository::SqliteSettingsRepository;

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

    /// Copies the live database file to `target`. Holds the connection lock
    /// for the duration so no write lands mid-copy.
    pub fn backup_to(&self, db_path: &Path, target: &Path) -> RepoResult<()> {
        let _guard = self.lock()?;
        std::fs::copy(db_path, target)?;
        Ok(())
    }

    /// Replaces the live database with `source`. The current connection is
    /// closed before the file is overwritten (Windows locks an open file;
    /// even where the OS wouldn't complain, writing under an open connection
    /// risks corrupting its page cache) and a fresh one is opened and
    /// migrated in its place — holding `lock()` for the whole swap is what
    /// makes this atomic from every other command's point of view.
    pub fn restore_from(&self, db_path: &Path, source: &Path) -> RepoResult<()> {
        let mut guard = self.lock()?;
        let placeholder = Connection::open_in_memory()?;
        let old = std::mem::replace(&mut *guard, placeholder);
        old.close()
            .map_err(|(_, e)| RepositoryError(e.to_string()))?;
        std::fs::copy(source, db_path)?;
        let new_conn = Connection::open(db_path)?;
        new_conn.pragma_update(None, "foreign_keys", true)?;
        migrations::run(&new_conn).map_err(|e| RepositoryError(e.to_string()))?;
        *guard = new_conn;
        Ok(())
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

    #[test]
    fn backup_then_restore_round_trips_data() {
        use crate::domain::habit::Habit;
        use crate::domain::repository::HabitRepository;
        use habit_repository::SqliteHabitRepository;
        use std::sync::Arc;

        let dir = tempfile::tempdir().unwrap();
        let live_path = dir.path().join("live.sqlite");
        let backup_path = dir.path().join("backup.sqlite");

        // Same Arc<Db> shared between the repository and the backup/restore
        // calls, matching how AppState hands the one Db out to every command.
        let db = Arc::new(Db::open(&live_path).unwrap());
        let habits = SqliteHabitRepository::new(Arc::clone(&db));
        habits
            .insert(&Habit::new(
                "Cigarettes".to_string(),
                "cigarette".to_string(),
            ))
            .unwrap();
        db.backup_to(&live_path, &backup_path).unwrap();

        // Diverge the live database from the backup.
        habits
            .insert(&Habit::new("Coffee".to_string(), "cup".to_string()))
            .unwrap();
        assert_eq!(habits.list(false).unwrap().len(), 2);

        db.restore_from(&live_path, &backup_path).unwrap();

        let list = habits.list(false).unwrap();
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].name, "Cigarettes");
    }
}
