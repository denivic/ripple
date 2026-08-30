use std::path::Path;

use rust_xlsxwriter::{Format, Workbook};

use super::{ExportError, ExportRow};

const HEADERS: [&str; 5] = [
    "Habit",
    "Occurred At",
    "Quantity",
    "Duration (minutes)",
    "Note",
];

pub fn write_xlsx(path: &Path, rows: &[ExportRow]) -> Result<(), ExportError> {
    let mut workbook = Workbook::new();
    let sheet = workbook.add_worksheet().set_name("Entries")?;
    let header_format = Format::new().set_bold();

    for (col, header) in HEADERS.iter().enumerate() {
        sheet.write_string_with_format(0, col as u16, *header, &header_format)?;
    }

    for (i, row) in rows.iter().enumerate() {
        let r = (i + 1) as u32;
        sheet.write_string(r, 0, &row.habit_name)?;
        sheet.write_string(r, 1, &row.occurred_at)?;
        sheet.write_number(r, 2, row.quantity)?;
        if let Some(d) = row.duration_minutes {
            sheet.write_number(r, 3, d)?;
        }
        if let Some(note) = &row.note {
            sheet.write_string(r, 4, note)?;
        }
    }

    workbook.save(path)?;
    Ok(())
}
