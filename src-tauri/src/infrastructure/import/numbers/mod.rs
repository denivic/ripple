mod archive;
mod bundle;
mod cell_storage;
mod protobuf;
mod registry;
mod snappy_frames;
mod source;

pub use source::NumbersSource;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum IwaError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Zip(#[from] zip::result::ZipError),
    #[error("{0}")]
    Format(String),
    /// The fallback contract: a specific revision this reader doesn't
    /// (yet) understand, surfaced as an actionable message rather than a
    /// silent failure or wrong data. See cell_storage.rs and registry.rs.
    #[error("{0}")]
    UnsupportedRevision(String),
}
