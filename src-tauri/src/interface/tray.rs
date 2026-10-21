//! Tray icon and close-to-tray: with `close_to_tray` on, closing the main
//! window hides it and Ripple keeps running (and notifying) in the tray.

use tauri::menu::{Menu, MenuItem};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{AppHandle, Manager, Runtime, Window, WindowEvent};

use crate::domain::repository::NotificationSettingsRepository;
use crate::infrastructure::db::SqliteNotificationRepository;

use super::state::AppState;

const MAIN_WINDOW: &str = "main";
const MENU_OPEN: &str = "open";
const MENU_QUIT: &str = "quit";

pub fn show_main_window<R: Runtime>(app: &AppHandle<R>) {
    if let Some(window) = app.get_webview_window(MAIN_WINDOW) {
        let _ = window.unminimize();
        let _ = window.show();
        let _ = window.set_focus();
    }
}

pub fn build<R: Runtime>(app: &tauri::App<R>) -> tauri::Result<()> {
    let open = MenuItem::with_id(app, MENU_OPEN, "Open Ripple", true, None::<&str>)?;
    let quit = MenuItem::with_id(app, MENU_QUIT, "Quit Ripple", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&open, &quit])?;
    let mut builder = TrayIconBuilder::with_id("ripple-tray")
        .tooltip("Ripple")
        .menu(&menu)
        // macOS convention: any click opens the menu. Elsewhere a left
        // click opens the window and the menu lives on the right button.
        .show_menu_on_left_click(cfg!(target_os = "macos"))
        .on_menu_event(|app, event| match event.id.as_ref() {
            MENU_OPEN => show_main_window(app),
            MENU_QUIT => app.exit(0),
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                if !cfg!(target_os = "macos") {
                    show_main_window(tray.app_handle());
                }
            }
        });
    if let Some(icon) = app.default_window_icon() {
        builder = builder.icon(icon.clone());
    }
    builder.build(app)?;
    Ok(())
}

/// Window-event hook: hides instead of closing while close-to-tray is on.
/// A read failure falls back to closing normally rather than trapping the
/// user with a window that won't close.
pub fn on_window_event<R: Runtime>(window: &Window<R>, event: &WindowEvent) {
    let WindowEvent::CloseRequested { api, .. } = event else {
        return;
    };
    if window.label() != MAIN_WINDOW {
        return;
    }
    let state = window.state::<AppState>();
    let close_to_tray = SqliteNotificationRepository::new(std::sync::Arc::clone(&state.db))
        .get_schedule()
        .map(|s| s.close_to_tray)
        .unwrap_or(false);
    if close_to_tray {
        api.prevent_close();
        let _ = window.hide();
    }
}
