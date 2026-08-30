use std::path::{Path, PathBuf};

use calamine::{open_workbook_auto, Data, Reader};
use chrono::{Datelike, Timelike};
use time::{Date, Month, PrimitiveDateTime};

use super::{CellValue, ImportError, Sheet, TabularSource};

pub struct XlsxSource {
    path: PathBuf,
}

impl XlsxSource {
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
        }
    }
}

impl TabularSource for XlsxSource {
    fn sheets(&self) -> Result<Vec<Sheet>, ImportError> {
        let mut workbook =
            open_workbook_auto(&self.path).map_err(|e| ImportError::Format(e.to_string()))?;
        let names = workbook.sheet_names().to_owned();
        let mut sheets = Vec::with_capacity(names.len());
        for name in names {
            let range = workbook
                .worksheet_range(&name)
                .map_err(|e| ImportError::Format(format!("{name}: {e}")))?;
            let rows = range
                .rows()
                .map(|row| row.iter().map(cell_from_calamine).collect())
                .collect();
            sheets.push(Sheet { name, rows });
        }
        Ok(sheets)
    }
}

fn cell_from_calamine(data: &Data) -> CellValue {
    match data {
        Data::Empty => CellValue::Empty,
        Data::String(s) => CellValue::Text(s.clone()),
        Data::Float(f) => CellValue::Number(*f),
        Data::Int(i) => CellValue::Number(*i as f64),
        Data::Bool(b) => CellValue::Bool(*b),
        Data::Error(e) => CellValue::Text(format!("#ERROR: {e:?}")),
        Data::DateTime(excel_dt) => match excel_dt.as_datetime() {
            Some(dt) => {
                let date = match Date::from_calendar_date(
                    dt.year(),
                    Month::try_from(dt.month() as u8).unwrap_or(Month::January),
                    dt.day() as u8,
                ) {
                    Ok(d) => d,
                    Err(_) => return CellValue::Text(dt.to_string()),
                };
                let time =
                    time::Time::from_hms(dt.hour() as u8, dt.minute() as u8, dt.second() as u8)
                        .unwrap_or(time::Time::MIDNIGHT);
                CellValue::DateTime(PrimitiveDateTime::new(date, time))
            }
            None => CellValue::Number(excel_dt.as_f64()),
        },
        Data::DateTimeIso(s) => CellValue::Text(s.clone()),
        Data::DurationIso(s) => CellValue::Text(s.clone()),
    }
}
