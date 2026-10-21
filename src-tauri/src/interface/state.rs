use std::path::PathBuf;
use std::sync::Arc;

use crate::application::notifications::NotificationHub;
use crate::infrastructure::db::Db;

pub struct AppState {
    pub db: Arc<Db>,
    pub db_path: PathBuf,
    /// Shared with the notification scheduler thread.
    pub notifications: Arc<NotificationHub>,
}
