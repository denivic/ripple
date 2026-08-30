use std::collections::HashMap;
use std::path::Path;

use crate::domain::habit::{Habit, HabitId};
use crate::domain::repository::{EntryRepository, HabitRepository};
use crate::infrastructure::db::codec::format_datetime;
use crate::infrastructure::export::{csv, json, xlsx, ExportError, ExportRow};

pub enum ExportFormat {
    Xlsx,
    Csv,
    Json,
}

pub fn export_entries(
    path: &Path,
    format: ExportFormat,
    habit_repo: &dyn HabitRepository,
    entry_repo: &dyn EntryRepository,
) -> Result<usize, ExportError> {
    let habits: HashMap<HabitId, Habit> = habit_repo
        .list(true)
        .map_err(|e| ExportError::Repository(e.to_string()))?
        .into_iter()
        .filter_map(|h| h.id.map(|id| (id, h)))
        .collect();
    let entries = entry_repo
        .list_all()
        .map_err(|e| ExportError::Repository(e.to_string()))?;

    let rows: Vec<ExportRow> = entries
        .iter()
        .map(|e| ExportRow {
            habit_name: habits
                .get(&e.habit_id)
                .map(|h| h.name.clone())
                .unwrap_or_else(|| "Unknown".to_string()),
            occurred_at: format_datetime(e.occurred_at),
            quantity: e.quantity,
            duration_minutes: e.duration_minutes,
            note: e.note.clone(),
        })
        .collect();

    let count = rows.len();
    match format {
        ExportFormat::Xlsx => xlsx::write_xlsx(path, &rows)?,
        ExportFormat::Csv => csv::write_csv(path, &rows)?,
        ExportFormat::Json => json::write_json(path, &rows)?,
    }
    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entry::Entry;
    use crate::infrastructure::db::{Db, SqliteEntryRepository, SqliteHabitRepository};
    use std::sync::Arc;
    use time::macros::datetime;

    #[test]
    fn export_resolves_habit_names_and_counts_rows() {
        let db = Arc::new(Db::open_in_memory().unwrap());
        let habit_repo = SqliteHabitRepository::new(Arc::clone(&db));
        let entry_repo = SqliteEntryRepository::new(Arc::clone(&db));

        let habit_id = habit_repo
            .insert(&Habit::new("Cigarettes", "cigarette"))
            .unwrap();
        entry_repo
            .insert(&Entry::new(
                habit_id,
                datetime!(2026 - 03 - 10 08:00:00),
                2.0,
            ))
            .unwrap();

        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("export.csv");
        let count = export_entries(&path, ExportFormat::Csv, &habit_repo, &entry_repo).unwrap();

        assert_eq!(count, 1);
        let content = std::fs::read_to_string(&path).unwrap();
        assert!(content.contains("Cigarettes"));
    }
}
