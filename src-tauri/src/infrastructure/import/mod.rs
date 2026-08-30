pub mod csv;
pub mod mapping;
pub mod numbers;
pub mod xlsx;

use thiserror::Error;
use time::PrimitiveDateTime;

/// The common shape every source format normalizes into — xlsx, csv and
/// `.numbers` are interchangeable behind [`TabularSource`], so adding a
/// fourth format later is one new file, not a change to anything that
/// already works. No bare `Date` variant: every source that can produce a
/// date attaches a time component (midnight, if the source has none), so
/// `DateTime` alone covers it without a redundant case to keep in sync.
#[derive(Debug, Clone, PartialEq)]
pub enum CellValue {
    Empty,
    Text(String),
    Number(f64),
    DateTime(PrimitiveDateTime),
    Bool(bool),
}

impl CellValue {
    pub fn as_text(&self) -> Option<String> {
        match self {
            CellValue::Empty => None,
            CellValue::Text(s) => Some(s.clone()),
            CellValue::Number(n) => Some(n.to_string()),
            CellValue::DateTime(dt) => Some(dt.to_string()),
            CellValue::Bool(b) => Some(b.to_string()),
        }
    }

    pub fn as_number(&self) -> Option<f64> {
        match self {
            CellValue::Number(n) => Some(*n),
            CellValue::Text(s) => s.trim().parse().ok(),
            CellValue::Bool(b) => Some(if *b { 1.0 } else { 0.0 }),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Sheet {
    pub name: String,
    /// Row-major; every row is padded/truncated by nothing — callers handle
    /// ragged rows by indexing with bounds checks.
    pub rows: Vec<Vec<CellValue>>,
}

#[derive(Debug, Error)]
pub enum ImportError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Csv(#[from] ::csv::Error),
    #[error("{0}")]
    Format(String),
    /// The fallback contract: a specific, actionable message rather than a
    /// silent failure or a generic parse error.
    #[error("{0}")]
    Unsupported(String),
}

pub trait TabularSource {
    fn sheets(&self) -> Result<Vec<Sheet>, ImportError>;
}
