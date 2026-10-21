//! The notification scheduler thread and the Tauri-side adapters for the
//! application layer's notification ports.

use std::sync::Arc;

use chrono::{Datelike, Timelike};
use tauri::plugin::PermissionState;
use tauri::{AppHandle, Runtime};
use tauri_plugin_notification::NotificationExt;
use time::{Date, Month, PrimitiveDateTime, Time};

use crate::application::email::{send_notification_email, EmailMessage};
use crate::application::notifications::{
    tick, Channels, EmailSender, LocalNotifier, NotificationHub, NotificationRepos,
    SchedulerState,
};
use crate::infrastructure::db::{
    Db, SqliteEntryRepository, SqliteHabitRepository, SqliteNotificationRepository,
    SqlitePointAwardRepository, SqliteProfileRepository, SqliteSettingsRepository,
};

use super::commands::notify_changed;

const TICK: std::time::Duration = std::time::Duration::from_secs(60);

/// OS notifications through `tauri-plugin-notification`.
pub struct TauriNotifier<R: Runtime>(pub AppHandle<R>);

impl<R: Runtime> LocalNotifier for TauriNotifier<R> {
    fn show(&self, title: &str, body: &str) -> Result<(), String> {
        self.0
            .notification()
            .builder()
            .title(title)
            .body(body)
            .show()
            .map_err(|e| e.to_string())
    }
}

/// Email through N2's `send_notification_email`.
pub struct DbEmailSender(pub Arc<Db>);

impl EmailSender for DbEmailSender {
    fn send(&self, msg: &EmailMessage) -> Result<bool, String> {
        send_notification_email(&self.0, msg)
    }
}

pub fn permission_label(state: PermissionState) -> &'static str {
    match state {
        PermissionState::Granted => "granted",
        PermissionState::Denied => "denied",
        _ => "prompt",
    }
}

/// Local wall-clock time via chrono: `time`'s local offset lookup refuses
/// to work once the process has more than one thread on macOS.
pub fn local_now() -> PrimitiveDateTime {
    let now = chrono::Local::now().naive_local();
    let date = Month::try_from(now.month() as u8)
        .ok()
        .and_then(|m| Date::from_calendar_date(now.year(), m, now.day() as u8).ok());
    let time = Time::from_hms(now.hour() as u8, now.minute() as u8, now.second() as u8).ok();
    match (date, time) {
        (Some(d), Some(t)) => PrimitiveDateTime::new(d, t),
        // chrono's fields are always in range; this is unreachable.
        _ => PrimitiveDateTime::MIN,
    }
}

/// Runs `tick` every minute (sooner when the hub is woken) for the life of
/// the process. Errors are logged and the loop carries on: a bad tick must
/// never take the scheduler down.
pub fn spawn<R: Runtime>(app: AppHandle<R>, db: Arc<Db>, hub: Arc<NotificationHub>) {
    std::thread::Builder::new()
        .name("ripple-notifications".to_string())
        .spawn(move || {
            let habits = SqliteHabitRepository::new(Arc::clone(&db));
            let entries = SqliteEntryRepository::new(Arc::clone(&db));
            let profile = SqliteProfileRepository::new(Arc::clone(&db));
            let settings = SqliteSettingsRepository::new(Arc::clone(&db));
            let awards = SqlitePointAwardRepository::new(Arc::clone(&db));
            let notifications = SqliteNotificationRepository::new(Arc::clone(&db));
            let repos = NotificationRepos {
                habits: &habits,
                entries: &entries,
                profile: &profile,
                settings: &settings,
                awards: &awards,
                notification_settings: &notifications,
                log: &notifications,
            };
            let local = TauriNotifier(app.clone());
            let email = DbEmailSender(Arc::clone(&db));
            let channels = Channels {
                local: &local,
                email: &email,
            };
            let mut state = SchedulerState::default();
            loop {
                match tick(&repos, &channels, &hub, &mut state, local_now()) {
                    Ok(outcome) if outcome.new_awards > 0 => notify_changed(&app, "points"),
                    Ok(_) => {}
                    Err(e) => eprintln!("notification scheduler tick failed: {e}"),
                }
                hub.wait(TICK);
            }
        })
        .expect("spawn notification scheduler thread");
}
