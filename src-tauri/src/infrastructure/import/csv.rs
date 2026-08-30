use std::path::{Path, PathBuf};

use csv::ReaderBuilder;

use super::{CellValue, ImportError, Sheet, TabularSource};

pub struct CsvSource {
    path: PathBuf,
}

impl CsvSource {
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
        }
    }
}

impl TabularSource for CsvSource {
    fn sheets(&self) -> Result<Vec<Sheet>, ImportError> {
        let mut reader = ReaderBuilder::new()
            .has_headers(false)
            .flexible(true)
            .from_path(&self.path)?;
        let mut rows = Vec::new();
        for record in reader.records() {
            let record = record?;
            rows.push(record.iter().map(infer_cell).collect());
        }
        let name = self
            .path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("Sheet1")
            .to_string();
        Ok(vec![Sheet { name, rows }])
    }
}

/// CSV has no type system — every field arrives as a string — so a plain
/// numeric parse is the only inference worth doing; dates are left as text
/// for the mapping step to parse against a chosen format.
fn infer_cell(s: &str) -> CellValue {
    if s.is_empty() {
        CellValue::Empty
    } else if let Ok(n) = s.parse::<f64>() {
        CellValue::Number(n)
    } else {
        CellValue::Text(s.to_string())
    }
}
