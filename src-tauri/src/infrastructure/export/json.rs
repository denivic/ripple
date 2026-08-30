use std::path::Path;

use serde::Serialize;

use super::{ExportError, ExportRow};

#[derive(Serialize)]
struct JsonRow<'a> {
    habit: &'a str,
    occurred_at: &'a str,
    quantity: f64,
    duration_minutes: Option<f64>,
    note: Option<&'a str>,
}

pub fn write_json(path: &Path, rows: &[ExportRow]) -> Result<(), ExportError> {
    let json_rows: Vec<JsonRow> = rows
        .iter()
        .map(|r| JsonRow {
            habit: &r.habit_name,
            occurred_at: &r.occurred_at,
            quantity: r.quantity,
            duration_minutes: r.duration_minutes,
            note: r.note.as_deref(),
        })
        .collect();
    let file = std::fs::File::create(path)?;
    serde_json::to_writer_pretty(file, &json_rows)?;
    Ok(())
}
