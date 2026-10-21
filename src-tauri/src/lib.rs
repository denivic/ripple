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
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let app_data_dir = app.path().app_data_dir()?;
            std::fs::create_dir_all(&app_data_dir)?;
            let db_path = app_data_dir.join("ripple.sqlite");
            let db = Arc::new(Db::open(&db_path)?);
            app.manage(AppState { db, db_path });
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
            interface::commands::get_settings,
            interface::commands::save_settings,
            interface::commands::fetch_exchange_rate,
            interface::commands::convert_currency,
            interface::commands::get_data_dir,
            interface::commands::backup_database,
            interface::commands::restore_database,
            interface::commands::reset_database,
            interface::commands::export_theme,
            interface::commands::import_theme,
            interface::commands::compute_timeline,
            interface::commands::compute_today_summary,
            interface::commands::get_habit_presets,
            interface::commands::preview_import,
            interface::commands::apply_import,
            interface::commands::export_entries,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
