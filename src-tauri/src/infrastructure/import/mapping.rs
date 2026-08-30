use serde::{Deserialize, Serialize};
use time::{Date, PrimitiveDateTime};

use super::{CellValue, Sheet};
use crate::domain::habit::HabitId;

/// Either every row names its own habit in a column (a multi-habit export),
/// or the whole sheet is being imported against one habit the user picked
/// up front (the common single-habit CSV export case).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum HabitMapping {
    Column(usize),
    Fixed(HabitId),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ColumnMapping {
    pub habit: HabitMapping,
    pub occurred_at_column: usize,
    pub quantity_column: Option<usize>,
    pub duration_column: Option<usize>,
    pub note_column: Option<usize>,
    pub has_header_row: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MappedEntry {
    /// Set when `habit` is `Column` — the caller resolves/creates a habit by
    /// this name. `None` (with `habit_id` set) when `habit` is `Fixed`.
    pub habit_name: Option<String>,
    pub habit_id: Option<HabitId>,
    pub occurred_at: PrimitiveDateTime,
    pub quantity: f64,
    pub duration_minutes: Option<f64>,
    pub note: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RowError {
    pub row_index: usize,
    pub message: String,
}

#[derive(Debug, Default)]
pub struct MappingResult {
    pub entries: Vec<MappedEntry>,
    pub errors: Vec<RowError>,
}

/// A malformed row is reported, not skipped silently or allowed to fail the
/// whole import — inline validation, applied at import time.
pub fn apply_mapping(sheet: &Sheet, mapping: &ColumnMapping) -> MappingResult {
    let mut result = MappingResult::default();
    let start = if mapping.has_header_row { 1 } else { 0 };
    for (i, row) in sheet.rows.iter().enumerate().skip(start) {
        match map_row(row, mapping) {
            Ok(entry) => result.entries.push(entry),
            Err(message) => result.errors.push(RowError {
                row_index: i,
                message,
            }),
        }
    }
    result
}

fn map_row(row: &[CellValue], mapping: &ColumnMapping) -> Result<MappedEntry, String> {
    let occurred_at_cell = row
        .get(mapping.occurred_at_column)
        .ok_or_else(|| "missing occurred_at column".to_string())?;
    let occurred_at = cell_to_datetime(occurred_at_cell)
        .ok_or_else(|| format!("could not parse a date/time from {occurred_at_cell:?}"))?;

    let quantity = mapping
        .quantity_column
        .and_then(|c| row.get(c))
        .and_then(CellValue::as_number)
        .unwrap_or(1.0);

    let duration_minutes = mapping
        .duration_column
        .and_then(|c| row.get(c))
        .and_then(CellValue::as_number);

    let note = mapping
        .note_column
        .and_then(|c| row.get(c))
        .and_then(CellValue::as_text)
        .filter(|s| !s.is_empty());

    let (habit_name, habit_id) = match mapping.habit {
        HabitMapping::Fixed(id) => (None, Some(id)),
        HabitMapping::Column(c) => {
            let name = row
                .get(c)
                .and_then(CellValue::as_text)
                .filter(|s| !s.is_empty())
                .ok_or_else(|| "missing habit name".to_string())?;
            (Some(name), None)
        }
    };

    Ok(MappedEntry {
        habit_name,
        habit_id,
        occurred_at,
        quantity,
        duration_minutes,
        note,
    })
}

fn cell_to_datetime(cell: &CellValue) -> Option<PrimitiveDateTime> {
    match cell {
        CellValue::DateTime(dt) => Some(*dt),
        CellValue::Text(s) => parse_flexible_datetime(s),
        _ => None,
    }
}

fn parse_flexible_datetime(s: &str) -> Option<PrimitiveDateTime> {
    use time::macros::format_description;

    let datetime_formats: &[&[time::format_description::FormatItem]] = &[
        format_description!("[year]-[month]-[day]T[hour]:[minute]:[second]"),
        format_description!("[year]-[month]-[day] [hour]:[minute]:[second]"),
    ];
    for fmt in datetime_formats {
        if let Ok(dt) = PrimitiveDateTime::parse(s.trim(), fmt) {
            return Some(dt);
        }
    }

    let date_formats: &[&[time::format_description::FormatItem]] = &[
        format_description!("[year]-[month]-[day]"),
        format_description!("[month padding:none]/[day padding:none]/[year]"),
    ];
    for fmt in date_formats {
        if let Ok(d) = Date::parse(s.trim(), fmt) {
            return Some(PrimitiveDateTime::new(d, time::Time::MIDNIGHT));
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::macros::datetime;

    fn sheet(rows: Vec<Vec<CellValue>>) -> Sheet {
        Sheet {
            name: "Sheet1".to_string(),
            rows,
        }
    }

    #[test]
    fn maps_a_well_formed_row_with_fixed_habit() {
        let s = sheet(vec![vec![
            CellValue::Text("2026-03-14".to_string()),
            CellValue::Number(2.0),
        ]]);
        let mapping = ColumnMapping {
            habit: HabitMapping::Fixed(HabitId(7)),
            occurred_at_column: 0,
            quantity_column: Some(1),
            duration_column: None,
            note_column: None,
            has_header_row: false,
        };
        let result = apply_mapping(&s, &mapping);
        assert_eq!(result.errors.len(), 0);
        assert_eq!(result.entries.len(), 1);
        assert_eq!(result.entries[0].habit_id, Some(HabitId(7)));
        assert_eq!(result.entries[0].occurred_at, datetime!(2026-03-14 0:00));
        assert_eq!(result.entries[0].quantity, 2.0);
    }

    #[test]
    fn skips_the_header_row_when_flagged() {
        let s = sheet(vec![
            vec![CellValue::Text("date".to_string())],
            vec![CellValue::Text("2026-01-01".to_string())],
        ]);
        let mapping = ColumnMapping {
            habit: HabitMapping::Fixed(HabitId(1)),
            occurred_at_column: 0,
            quantity_column: None,
            duration_column: None,
            note_column: None,
            has_header_row: true,
        };
        let result = apply_mapping(&s, &mapping);
        assert_eq!(result.entries.len(), 1);
        assert_eq!(result.errors.len(), 0);
    }

    #[test]
    fn quantity_defaults_to_one_unit_when_unmapped() {
        let s = sheet(vec![vec![CellValue::Text("2026-01-01".to_string())]]);
        let mapping = ColumnMapping {
            habit: HabitMapping::Fixed(HabitId(1)),
            occurred_at_column: 0,
            quantity_column: None,
            duration_column: None,
            note_column: None,
            has_header_row: false,
        };
        let result = apply_mapping(&s, &mapping);
        assert_eq!(result.entries[0].quantity, 1.0);
    }

    #[test]
    fn reports_a_row_error_instead_of_failing_the_whole_import() {
        let s = sheet(vec![
            vec![CellValue::Text("2026-01-01".to_string())],
            vec![CellValue::Text("not a date".to_string())],
            vec![CellValue::Text("2026-01-03".to_string())],
        ]);
        let mapping = ColumnMapping {
            habit: HabitMapping::Fixed(HabitId(1)),
            occurred_at_column: 0,
            quantity_column: None,
            duration_column: None,
            note_column: None,
            has_header_row: false,
        };
        let result = apply_mapping(&s, &mapping);
        assert_eq!(result.entries.len(), 2);
        assert_eq!(result.errors.len(), 1);
        assert_eq!(result.errors[0].row_index, 1);
    }

    #[test]
    fn column_habit_mapping_resolves_by_name() {
        let s = sheet(vec![vec![
            CellValue::Text("Cigarettes".to_string()),
            CellValue::Text("2026-01-01".to_string()),
        ]]);
        let mapping = ColumnMapping {
            habit: HabitMapping::Column(0),
            occurred_at_column: 1,
            quantity_column: None,
            duration_column: None,
            note_column: None,
            has_header_row: false,
        };
        let result = apply_mapping(&s, &mapping);
        assert_eq!(result.entries[0].habit_name.as_deref(), Some("Cigarettes"));
        assert_eq!(result.entries[0].habit_id, None);
    }

    #[test]
    fn parses_slash_dates() {
        let s = sheet(vec![vec![CellValue::Text("3/14/2026".to_string())]]);
        let mapping = ColumnMapping {
            habit: HabitMapping::Fixed(HabitId(1)),
            occurred_at_column: 0,
            quantity_column: None,
            duration_column: None,
            note_column: None,
            has_header_row: false,
        };
        let result = apply_mapping(&s, &mapping);
        assert_eq!(result.entries[0].occurred_at, datetime!(2026-03-14 0:00));
    }
}
