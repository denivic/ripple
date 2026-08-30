use std::path::Path;

use csv::WriterBuilder;

use super::{ExportError, ExportRow};

pub fn write_csv(path: &Path, rows: &[ExportRow]) -> Result<(), ExportError> {
    let mut writer = WriterBuilder::new().from_path(path)?;
    writer.write_record([
        "Habit",
        "Occurred At",
        "Quantity",
        "Duration (minutes)",
        "Note",
    ])?;
    for row in rows {
        writer.write_record([
            row.habit_name.as_str(),
            row.occurred_at.as_str(),
            &row.quantity.to_string(),
            &row.duration_minutes
                .map(|d| d.to_string())
                .unwrap_or_default(),
            row.note.as_deref().unwrap_or(""),
        ])?;
    }
    writer.flush()?;
    Ok(())
}
