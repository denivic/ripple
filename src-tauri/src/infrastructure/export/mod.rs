pub mod csv;
pub mod json;
pub mod xlsx;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ExportError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Csv(#[from] ::csv::Error),
    #[error(transparent)]
    Xlsx(#[from] rust_xlsxwriter::XlsxError),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error("{0}")]
    Repository(String),
}

/// A denormalized, ready-to-write row — every exporter wants the same
/// shape (habit name resolved, dates formatted), so repositories/domain
/// entities are flattened into this once rather than per-format.
pub struct ExportRow {
    pub habit_name: String,
    pub occurred_at: String,
    pub quantity: f64,
    pub duration_minutes: Option<f64>,
    pub note: Option<String>,
}
