use std::sync::Arc;

use rusqlite::{params, OptionalExtension, Row};
use time::PrimitiveDateTime;

use crate::domain::entry::{Entry, EntryId};
use crate::domain::habit::HabitId;
use crate::domain::repository::{EntryRepository, RepoResult, RepositoryError};

use super::codec::{format_datetime, parse_datetime};
use super::Db;

pub struct SqliteEntryRepository {
    db: Arc<Db>,
}

impl SqliteEntryRepository {
    pub fn new(db: Arc<Db>) -> Self {
        Self { db }
    }
}

const COLUMNS: &str = "id, habit_id, occurred_at, quantity, duration_minutes, note";

fn row_to_entry(row: &Row) -> rusqlite::Result<Entry> {
    let occurred_at_str: String = row.get(2)?;
    let occurred_at = parse_datetime(&occurred_at_str).map_err(|e| {
        rusqlite::Error::FromSqlConversionFailure(2, rusqlite::types::Type::Text, Box::new(e))
    })?;
    Ok(Entry {
        id: Some(EntryId(row.get(0)?)),
        habit_id: HabitId(row.get(1)?),
        occurred_at,
        quantity: row.get(3)?,
        duration_minutes: row.get(4)?,
        note: row.get(5)?,
    })
}

impl EntryRepository for SqliteEntryRepository {
    fn insert(&self, entry: &Entry) -> RepoResult<EntryId> {
        let conn = self.db.lock()?;
        conn.execute(
            "INSERT INTO entries (habit_id, occurred_at, quantity, duration_minutes, note) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                entry.habit_id.0,
                format_datetime(entry.occurred_at),
                entry.quantity,
                entry.duration_minutes,
                entry.note,
            ],
        )?;
        Ok(EntryId(conn.last_insert_rowid()))
    }

    fn update(&self, entry: &Entry) -> RepoResult<()> {
        let id = entry
            .id
            .ok_or_else(|| RepositoryError("cannot update an entry without an id".into()))?;
        let conn = self.db.lock()?;
        conn.execute(
            "UPDATE entries SET habit_id = ?1, occurred_at = ?2, quantity = ?3, duration_minutes = ?4, note = ?5 WHERE id = ?6",
            params![
                entry.habit_id.0,
                format_datetime(entry.occurred_at),
                entry.quantity,
                entry.duration_minutes,
                entry.note,
                id.0,
            ],
        )?;
        Ok(())
    }

    fn delete(&self, id: EntryId) -> RepoResult<()> {
        let conn = self.db.lock()?;
        conn.execute("DELETE FROM entries WHERE id = ?1", params![id.0])?;
        Ok(())
    }

    fn get(&self, id: EntryId) -> RepoResult<Option<Entry>> {
        let conn = self.db.lock()?;
        conn.query_row(
            &format!("SELECT {COLUMNS} FROM entries WHERE id = ?1"),
            params![id.0],
            row_to_entry,
        )
        .optional()
        .map_err(Into::into)
    }

    fn list_between(
        &self,
        start: PrimitiveDateTime,
        end: PrimitiveDateTime,
    ) -> RepoResult<Vec<Entry>> {
        let conn = self.db.lock()?;
        // ISO-8601 with zero-padded, fixed-width fields sorts lexicographically
        // identically to chronologically, so a plain TEXT range scan is correct.
        let mut stmt = conn.prepare(&format!(
            "SELECT {COLUMNS} FROM entries WHERE occurred_at >= ?1 AND occurred_at <= ?2 ORDER BY occurred_at"
        ))?;
        let rows = stmt.query_map(
            params![format_datetime(start), format_datetime(end)],
            row_to_entry,
        )?;
        rows.collect::<rusqlite::Result<Vec<_>>>()
            .map_err(Into::into)
    }

    fn list_all(&self) -> RepoResult<Vec<Entry>> {
        let conn = self.db.lock()?;
        let mut stmt = conn.prepare(&format!(
            "SELECT {COLUMNS} FROM entries ORDER BY occurred_at"
        ))?;
        let rows = stmt.query_map([], row_to_entry)?;
        rows.collect::<rusqlite::Result<Vec<_>>>()
            .map_err(Into::into)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::habit::Habit;
    use crate::domain::repository::HabitRepository;
    use time::macros::datetime;

    fn repo_with_habit() -> (SqliteEntryRepository, HabitId) {
        let db = Arc::new(Db::open_in_memory().unwrap());
        let habit_repo = super::super::SqliteHabitRepository::new(Arc::clone(&db));
        let habit_id = habit_repo
            .insert(&Habit::new("Cigarettes", "cigarette"))
            .unwrap();
        (SqliteEntryRepository::new(db), habit_id)
    }

    #[test]
    fn insert_then_get_round_trips_including_datetime() {
        let (repo, habit_id) = repo_with_habit();
        let mut entry = Entry::new(habit_id, datetime!(2026 - 03 - 14 08:30:00), 2.0);
        entry.duration_minutes = Some(10.0);
        entry.note = Some("after lunch".to_string());
        let id = repo.insert(&entry).unwrap();

        let fetched = repo.get(id).unwrap().unwrap();
        assert_eq!(fetched.occurred_at, datetime!(2026 - 03 - 14 08:30:00));
        assert_eq!(fetched.quantity, 2.0);
        assert_eq!(fetched.duration_minutes, Some(10.0));
        assert_eq!(fetched.note.as_deref(), Some("after lunch"));
    }

    #[test]
    fn list_between_filters_by_range() {
        let (repo, habit_id) = repo_with_habit();
        repo.insert(&Entry::new(
            habit_id,
            datetime!(2026 - 01 - 01 08:00:00),
            1.0,
        ))
        .unwrap();
        repo.insert(&Entry::new(
            habit_id,
            datetime!(2026 - 01 - 05 08:00:00),
            1.0,
        ))
        .unwrap();
        repo.insert(&Entry::new(
            habit_id,
            datetime!(2026 - 01 - 10 08:00:00),
            1.0,
        ))
        .unwrap();

        let results = repo
            .list_between(
                datetime!(2026 - 01 - 02 00:00:00),
                datetime!(2026 - 01 - 09 00:00:00),
            )
            .unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].occurred_at, datetime!(2026 - 01 - 05 08:00:00));
    }

    #[test]
    fn deleting_a_habit_cascades_to_its_entries() {
        let (repo, habit_id) = repo_with_habit();
        repo.insert(&Entry::new(
            habit_id,
            datetime!(2026 - 01 - 01 08:00:00),
            1.0,
        ))
        .unwrap();
        repo.db
            .lock()
            .unwrap()
            .execute("DELETE FROM habits WHERE id = ?1", params![habit_id.0])
            .unwrap();

        assert_eq!(repo.list_all().unwrap().len(), 0);
    }
}
