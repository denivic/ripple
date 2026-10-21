mod application;
mod domain;
mod infrastructure;
mod interface;

use std::sync::Arc;

use tauri::Manager;

use application::notifications::NotificationHub;
use infrastructure::db::Db;
use interface::state::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            None,
        ))
        .setup(|app| {
            let app_data_dir = app.path().app_data_dir()?;
            std::fs::create_dir_all(&app_data_dir)?;
            let db_path = app_data_dir.join("ripple.sqlite");
            let db = Arc::new(Db::open(&db_path)?);
            let notifications = Arc::new(NotificationHub::default());
            app.manage(AppState {
                db: Arc::clone(&db),
                db_path,
                notifications: Arc::clone(&notifications),
            });
            interface::tray::build(app)?;
            interface::scheduler::spawn(app.handle().clone(), db, notifications);
            Ok(())
        })
        .on_window_event(interface::tray::on_window_event)
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
            interface::commands::get_progress,
            interface::commands::get_habit_presets,
            interface::commands::preview_import,
            interface::commands::apply_import,
            interface::commands::export_entries,
            interface::commands::get_email_settings,
            interface::commands::save_email_settings,
            interface::commands::clear_email_secret,
            interface::commands::send_test_email,
            interface::commands::get_notification_settings,
            interface::commands::save_notification_settings,
            interface::commands::send_test_notification,
        ])
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|_app, _event| {
            // macOS: clicking the dock icon brings back a window hidden to
            // the tray.
            #[cfg(target_os = "macos")]
            if let tauri::RunEvent::Reopen { .. } = _event {
                interface::tray::show_main_window(_app);
            }
        });
}
