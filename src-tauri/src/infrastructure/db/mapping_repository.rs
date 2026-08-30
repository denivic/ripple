use std::sync::Arc;

use rusqlite::{params, OptionalExtension};

use crate::domain::repository::{RepoResult, RepositoryError};
use crate::infrastructure::import::mapping::ColumnMapping;

use super::codec::format_datetime;
use super::Db;

/// Not a domain repository (`ColumnMapping` is an import-wizard convenience,
/// not a business entity) — kept in infrastructure alongside the other
/// SQLite adapters rather than promoted to a `domain::repository` port.
pub struct SqliteMappingRepository {
    db: Arc<Db>,
}

impl SqliteMappingRepository {
    pub fn new(db: Arc<Db>) -> Self {
        Self { db }
    }

    pub fn find(&self, source_signature: &str) -> RepoResult<Option<ColumnMapping>> {
        let conn = self.db.lock()?;
        let json: Option<String> = conn
            .query_row(
                "SELECT mapping_json FROM import_mappings WHERE source_signature = ?1",
                params![source_signature],
                |row| row.get(0),
            )
            .optional()?;
        json.map(|j| {
            serde_json::from_str(&j)
                .map_err(|e| RepositoryError(format!("stored mapping was malformed: {e}")))
        })
        .transpose()
    }

    pub fn save(&self, source_signature: &str, mapping: &ColumnMapping) -> RepoResult<()> {
        let json = serde_json::to_string(mapping).map_err(|e| RepositoryError(e.to_string()))?;
        let conn = self.db.lock()?;
        conn.execute(
            "INSERT INTO import_mappings (source_signature, mapping_json, updated_at)
             VALUES (?1, ?2, ?3)
             ON CONFLICT(source_signature) DO UPDATE SET
               mapping_json = excluded.mapping_json,
               updated_at = excluded.updated_at",
            params![source_signature, json, format_datetime(now())],
        )?;
        Ok(())
    }
}

fn now() -> time::PrimitiveDateTime {
    let offset_now = time::OffsetDateTime::now_utc();
    time::PrimitiveDateTime::new(offset_now.date(), offset_now.time())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::habit::HabitId;
    use crate::infrastructure::import::mapping::HabitMapping;

    fn repo() -> SqliteMappingRepository {
        SqliteMappingRepository::new(Arc::new(Db::open_in_memory().unwrap()))
    }

    fn sample_mapping() -> ColumnMapping {
        ColumnMapping {
            habit: HabitMapping::Fixed(HabitId(3)),
            occurred_at_column: 0,
            quantity_column: Some(1),
            duration_column: None,
            note_column: Some(2),
            has_header_row: true,
        }
    }

    #[test]
    fn find_before_any_save_returns_none() {
        assert_eq!(repo().find("date,qty,note").unwrap(), None);
    }

    #[test]
    fn save_then_find_round_trips() {
        let repo = repo();
        repo.save("date,qty,note", &sample_mapping()).unwrap();
        let found = repo.find("date,qty,note").unwrap().unwrap();
        assert_eq!(found.occurred_at_column, 0);
        assert_eq!(found.quantity_column, Some(1));
        assert_eq!(found.habit, HabitMapping::Fixed(HabitId(3)));
    }

    #[test]
    fn save_twice_upserts_by_signature() {
        let repo = repo();
        repo.save("date,qty,note", &sample_mapping()).unwrap();
        let mut updated = sample_mapping();
        updated.note_column = None;
        repo.save("date,qty,note", &updated).unwrap();
        assert_eq!(
            repo.find("date,qty,note").unwrap().unwrap().note_column,
            None
        );
    }
}
