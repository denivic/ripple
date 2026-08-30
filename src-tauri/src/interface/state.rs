use std::sync::Arc;

use crate::infrastructure::db::Db;

pub struct AppState {
    pub db: Arc<Db>,
}
