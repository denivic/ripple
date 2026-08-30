use thiserror::Error;

use crate::domain::repository::RepositoryError;

#[derive(Debug, Error)]
pub enum DbError {
    #[error(transparent)]
    Sqlite(#[from] rusqlite::Error),
}

impl From<rusqlite::Error> for RepositoryError {
    fn from(e: rusqlite::Error) -> Self {
        RepositoryError(e.to_string())
    }
}

impl From<time::error::Parse> for RepositoryError {
    fn from(e: time::error::Parse) -> Self {
        RepositoryError(format!("stored date/time could not be parsed: {e}"))
    }
}
