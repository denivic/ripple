mod application;
mod domain;
mod infrastructure;
mod interface;

use std::sync::Arc;

use tauri::Manager;

use infrastructure::db::Db;
use interface::state::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let app_data_dir = app.path().app_data_dir()?;
            std::fs::create_dir_all(&app_data_dir)?;
            let db = Arc::new(Db::open(app_data_dir.join("ripple.sqlite"))?);
            app.manage(AppState { db });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            interface::commands::list_habits,
            interface::commands::get_habit,
            interface::commands::create_habit,
            interface::commands::update_habit,
            interface::commands::archive_habit,
            interface::commands::log_entry,
            interface::commands::get_entry,
            interface::commands::update_entry,
            interface::commands::delete_entry,
            interface::commands::list_entries,
            interface::commands::list_entries_between,
            interface::commands::get_profile,
            interface::commands::save_profile,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
