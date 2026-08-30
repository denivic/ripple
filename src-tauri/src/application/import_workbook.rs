use std::collections::HashMap;
use std::path::Path;

use crate::domain::entry::Entry;
use crate::domain::habit::{Habit, HabitId};
use crate::domain::repository::{EntryRepository, HabitRepository};
use crate::infrastructure::db::SqliteMappingRepository;
use crate::infrastructure::import::csv::CsvSource;
use crate::infrastructure::import::mapping::{
    apply_mapping, ColumnMapping, HabitMapping, RowError,
};
use crate::infrastructure::import::numbers::NumbersSource;
use crate::infrastructure::import::xlsx::XlsxSource;
use crate::infrastructure::import::{CellValue, ImportError, Sheet, TabularSource};

fn detect_source(path: &Path) -> Result<Box<dyn TabularSource>, ImportError> {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();
    match ext.as_str() {
        "xlsx" | "xls" => Ok(Box::new(XlsxSource::new(path))),
        "csv" => Ok(Box::new(CsvSource::new(path))),
        "numbers" => Ok(Box::new(NumbersSource::new(path))),
        other => Err(ImportError::Unsupported(format!(
            "unsupported file extension '.{other}'"
        ))),
    }
}

/// A signature of the source file's shape (its header row), not its
/// filename — a recurring export from another app keeps the same columns
/// across files, so this is what "remember the mapping per source file"
/// (plan-v1.md) actually needs to key on to be useful on the next import.
pub fn source_signature(sheet: &Sheet) -> String {
    sheet
        .rows
        .first()
        .map(|row| {
            row.iter()
                .filter_map(CellValue::as_text)
                .collect::<Vec<_>>()
                .join("|")
        })
        .unwrap_or_default()
}

pub struct ImportPreview {
    pub sheets: Vec<Sheet>,
    pub source_signature: String,
    pub remembered_mapping: Option<ColumnMapping>,
}

pub fn preview_import(
    path: &Path,
    mapping_repo: &SqliteMappingRepository,
) -> Result<ImportPreview, ImportError> {
    let source = detect_source(path)?;
    let sheets = source.sheets()?;
    let signature = sheets.first().map(source_signature).unwrap_or_default();
    let remembered_mapping = mapping_repo.find(&signature).unwrap_or(None);
    Ok(ImportPreview {
        sheets,
        source_signature: signature,
        remembered_mapping,
    })
}

pub struct ImportSummary {
    pub entries_created: usize,
    pub row_errors: Vec<RowError>,
}

/// Applies a chosen mapping, creating any habit a `Column` mapping names
/// for the first time, then remembers the mapping for next time.
pub fn apply_import(
    path: &Path,
    sheet_index: usize,
    mapping: &ColumnMapping,
    habit_repo: &dyn HabitRepository,
    entry_repo: &dyn EntryRepository,
    mapping_repo: &SqliteMappingRepository,
) -> Result<ImportSummary, ImportError> {
    let source = detect_source(path)?;
    let sheets = source.sheets()?;
    let sheet = sheets
        .get(sheet_index)
        .ok_or_else(|| ImportError::Format("sheet index out of range".to_string()))?;

    let result = apply_mapping(sheet, mapping);

    let mut habit_ids_by_name: HashMap<String, HabitId> = HashMap::new();
    if matches!(mapping.habit, HabitMapping::Column(_)) {
        let existing = habit_repo
            .list(true)
            .map_err(|e| ImportError::Format(e.to_string()))?;
        habit_ids_by_name.extend(
            existing
                .into_iter()
                .filter_map(|h| h.id.map(|id| (h.name, id))),
        );
    }

    let mut entries_created = 0usize;
    for mapped in result.entries {
        let habit_id = match mapped.habit_id {
            Some(id) => id,
            None => {
                let name = mapped.habit_name.unwrap_or_default();
                if let Some(&id) = habit_ids_by_name.get(&name) {
                    id
                } else {
                    let id = habit_repo
                        .insert(&Habit::new(name.clone(), "unit"))
                        .map_err(|e| ImportError::Format(e.to_string()))?;
                    habit_ids_by_name.insert(name, id);
                    id
                }
            }
        };
        let entry = Entry {
            id: None,
            habit_id,
            occurred_at: mapped.occurred_at,
            quantity: mapped.quantity,
            duration_minutes: mapped.duration_minutes,
            note: mapped.note,
        };
        entry_repo
            .insert(&entry)
            .map_err(|e| ImportError::Format(e.to_string()))?;
        entries_created += 1;
    }

    let signature = source_signature(sheet);
    // Remembering the mapping is a convenience, not correctness-critical:
    // a failure here shouldn't undo a successful import.
    let _ = mapping_repo.save(&signature, mapping);

    Ok(ImportSummary {
        entries_created,
        row_errors: result.errors,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::db::{Db, SqliteEntryRepository, SqliteHabitRepository};
    use crate::infrastructure::import::mapping::HabitMapping;
    use std::sync::Arc;

    fn write_csv(dir: &std::path::Path, name: &str, content: &str) -> std::path::PathBuf {
        let path = dir.join(name);
        std::fs::write(&path, content).unwrap();
        path
    }

    #[test]
    fn imports_a_csv_with_a_fixed_habit_and_remembers_the_mapping() {
        let dir = tempfile::tempdir().unwrap();
        let db = Arc::new(Db::open_in_memory().unwrap());
        let habit_repo = SqliteHabitRepository::new(Arc::clone(&db));
        let entry_repo = SqliteEntryRepository::new(Arc::clone(&db));
        let mapping_repo = SqliteMappingRepository::new(Arc::clone(&db));

        let habit_id = habit_repo
            .insert(&Habit::new("Cigarettes", "cigarette"))
            .unwrap();
        let path = write_csv(
            dir.path(),
            "log.csv",
            "date,qty\n2026-03-10,2\n2026-03-11,3\n",
        );

        let mapping = ColumnMapping {
            habit: HabitMapping::Fixed(habit_id),
            occurred_at_column: 0,
            quantity_column: Some(1),
            duration_column: None,
            note_column: None,
            has_header_row: true,
        };

        let summary =
            apply_import(&path, 0, &mapping, &habit_repo, &entry_repo, &mapping_repo).unwrap();
        assert_eq!(summary.entries_created, 2);
        assert_eq!(summary.row_errors.len(), 0);
        assert_eq!(entry_repo.list_all().unwrap().len(), 2);

        let signature = source_signature(&detect_source(&path).unwrap().sheets().unwrap()[0]);
        assert!(mapping_repo.find(&signature).unwrap().is_some());
    }

    #[test]
    fn column_habit_mapping_creates_habits_by_name() {
        let dir = tempfile::tempdir().unwrap();
        let db = Arc::new(Db::open_in_memory().unwrap());
        let habit_repo = SqliteHabitRepository::new(Arc::clone(&db));
        let entry_repo = SqliteEntryRepository::new(Arc::clone(&db));
        let mapping_repo = SqliteMappingRepository::new(Arc::clone(&db));

        let path = write_csv(
            dir.path(),
            "log.csv",
            "habit,date\nCigarettes,2026-03-10\nCigarettes,2026-03-11\nAlcohol,2026-03-11\n",
        );
        let mapping = ColumnMapping {
            habit: HabitMapping::Column(0),
            occurred_at_column: 1,
            quantity_column: None,
            duration_column: None,
            note_column: None,
            has_header_row: true,
        };

        let summary =
            apply_import(&path, 0, &mapping, &habit_repo, &entry_repo, &mapping_repo).unwrap();
        assert_eq!(summary.entries_created, 3);
        assert_eq!(habit_repo.list(true).unwrap().len(), 2);
    }
}
