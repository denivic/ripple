use std::sync::Arc;

use serde::Serialize;
use tauri::{AppHandle, Emitter, State};

use crate::application::export_workbook::ExportFormat;
use crate::application::{
    compute_series, convert_currency as currency_conversion, export_workbook,
    import_workbook, today_summary,
};
use crate::domain::presets::habit_presets;
use crate::domain::repository::{
    EntryRepository, HabitRepository, ProfileRepository, SettingsRepository,
};
use crate::infrastructure::db::codec::{parse_date, parse_datetime};
use crate::infrastructure::db::{
    SqliteCurrencyConversionRepository, SqliteEntryRepository, SqliteHabitRepository,
    SqliteMappingRepository, SqlitePointAwardRepository, SqliteProfileRepository,
    SqliteSettingsRepository,
};
use crate::infrastructure::fx::FrankfurterRateSource;
use crate::infrastructure::import::mapping::ColumnMapping;

use super::dto::{
    CurrencyConversionDto, EntryDto, HabitDto, HabitPresetDto, ImportPreviewDto, ImportSummaryDto,
    ProfileDto, ProgressDto, SettingsDto, TimelineDto, TodaySummaryDto,
};
use super::state::AppState;

// Notifications (N1)
use super::dto::NotificationSettingsDto;
use super::scheduler::permission_label;
use crate::application::notifications;
use crate::domain::email::{EmailProvider, EmailSettingsRepository};
use crate::domain::repository::NotificationSettingsRepository;
use crate::infrastructure::db::SqliteNotificationRepository;
use tauri_plugin_autostart::ManagerExt as _;
use tauri_plugin_notification::NotificationExt as _;

// Email settings (N2)
use super::dto::EmailSettingsDto;
use crate::application::email;
use crate::infrastructure::db::SqliteEmailSettingsRepository;
use crate::infrastructure::email::keychain::KeychainSecretStore;
use crate::infrastructure::email::ProviderTransports;

const DATA_CHANGED_EVENT: &str = "ripple://data-changed";

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct DataChangedEvent {
    scope: &'static str,
}

/// Observer: the frontend's stores subscribe to this once and patch their
/// own state, rather than every write command's caller having to know who
/// else needs to refresh. Emission failure is a wiring bug, not something a
/// command's caller can act on, so it's logged rather than surfaced as a
/// command error.
pub(crate) fn notify_changed<R: tauri::Runtime>(app: &AppHandle<R>, scope: &'static str) {
    if let Err(e) = app.emit(DATA_CHANGED_EVENT, DataChangedEvent { scope }) {
        eprintln!("failed to emit {DATA_CHANGED_EVENT}: {e}");
    }
}

#[tauri::command]
pub fn list_habits(
    state: State<AppState>,
    include_archived: bool,
) -> Result<Vec<HabitDto>, String> {
    SqliteHabitRepository::new(Arc::clone(&state.db))
        .list(include_archived)
        .map(|habits| habits.into_iter().map(Into::into).collect())
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_habit(state: State<AppState>, habit_id: i64) -> Result<Option<HabitDto>, String> {
    SqliteHabitRepository::new(Arc::clone(&state.db))
        .get(crate::domain::HabitId(habit_id))
        .map(|habit| habit.map(Into::into))
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn create_habit(
    app: AppHandle,
    state: State<AppState>,
    habit: HabitDto,
) -> Result<HabitDto, String> {
    let repo = SqliteHabitRepository::new(Arc::clone(&state.db));
    let mut domain_habit = crate::domain::Habit::from(habit);
    let id = repo.insert(&domain_habit).map_err(|e| e.to_string())?;
    domain_habit.id = Some(id);
    notify_changed(&app, "habits");
    Ok(domain_habit.into())
}

#[tauri::command]
pub fn update_habit(app: AppHandle, state: State<AppState>, habit: HabitDto) -> Result<(), String> {
    SqliteHabitRepository::new(Arc::clone(&state.db))
        .update(&habit.into())
        .map_err(|e| e.to_string())?;
    notify_changed(&app, "habits");
    Ok(())
}

#[tauri::command]
pub fn archive_habit(app: AppHandle, state: State<AppState>, habit_id: i64) -> Result<(), String> {
    SqliteHabitRepository::new(Arc::clone(&state.db))
        .archive(crate::domain::HabitId(habit_id))
        .map_err(|e| e.to_string())?;
    notify_changed(&app, "habits");
    Ok(())
}

#[tauri::command]
pub fn log_entry(
    app: AppHandle,
    state: State<AppState>,
    entry: EntryDto,
) -> Result<EntryDto, String> {
    let repo = SqliteEntryRepository::new(Arc::clone(&state.db));
    let mut domain_entry: crate::domain::Entry = entry.try_into()?;
    let id = repo.insert(&domain_entry).map_err(|e| e.to_string())?;
    domain_entry.id = Some(id);
    notify_changed(&app, "entries");
    Ok(domain_entry.into())
}

#[tauri::command]
pub fn get_entry(state: State<AppState>, entry_id: i64) -> Result<Option<EntryDto>, String> {
    SqliteEntryRepository::new(Arc::clone(&state.db))
        .get(crate::domain::EntryId(entry_id))
        .map(|entry| entry.map(Into::into))
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_entry(app: AppHandle, state: State<AppState>, entry: EntryDto) -> Result<(), String> {
    let domain_entry: crate::domain::Entry = entry.try_into()?;
    SqliteEntryRepository::new(Arc::clone(&state.db))
        .update(&domain_entry)
        .map_err(|e| e.to_string())?;
    notify_changed(&app, "entries");
    Ok(())
}

#[tauri::command]
pub fn delete_entry(app: AppHandle, state: State<AppState>, entry_id: i64) -> Result<(), String> {
    SqliteEntryRepository::new(Arc::clone(&state.db))
        .delete(crate::domain::EntryId(entry_id))
        .map_err(|e| e.to_string())?;
    notify_changed(&app, "entries");
    Ok(())
}

#[tauri::command]
pub fn list_entries(state: State<AppState>) -> Result<Vec<EntryDto>, String> {
    SqliteEntryRepository::new(Arc::clone(&state.db))
        .list_all()
        .map(|entries| entries.into_iter().map(Into::into).collect())
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn list_entries_between(
    state: State<AppState>,
    start: String,
    end: String,
) -> Result<Vec<EntryDto>, String> {
    let start = parse_datetime(&start).map_err(|e| e.to_string())?;
    let end = parse_datetime(&end).map_err(|e| e.to_string())?;
    SqliteEntryRepository::new(Arc::clone(&state.db))
        .list_between(start, end)
        .map(|entries| entries.into_iter().map(Into::into).collect())
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_profile(state: State<AppState>) -> Result<ProfileDto, String> {
    SqliteProfileRepository::new(Arc::clone(&state.db))
        .get()
        .map(Into::into)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn save_profile(
    app: AppHandle,
    state: State<AppState>,
    profile: ProfileDto,
) -> Result<(), String> {
    let domain_profile: crate::domain::Profile = profile.try_into()?;
    SqliteProfileRepository::new(Arc::clone(&state.db))
        .save(&domain_profile)
        .map_err(|e| e.to_string())?;
    notify_changed(&app, "profile");
    Ok(())
}

#[tauri::command]
pub fn get_settings(state: State<AppState>) -> Result<SettingsDto, String> {
    SqliteSettingsRepository::new(Arc::clone(&state.db))
        .get()
        .map(Into::into)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn save_settings(
    app: AppHandle,
    state: State<AppState>,
    settings: SettingsDto,
) -> Result<(), String> {
    let domain_settings: crate::domain::Settings = settings.try_into()?;
    SqliteSettingsRepository::new(Arc::clone(&state.db))
        .save(&domain_settings)
        .map_err(|e| e.to_string())?;
    notify_changed(&app, "settings");
    Ok(())
}

/// Preview for the Settings conversion dialog. Async + spawn_blocking so the
/// HTTP round trip never stalls the main thread.
#[tauri::command]
pub async fn fetch_exchange_rate(from: String, to: String) -> Result<f64, String> {
    tauri::async_runtime::spawn_blocking(move || {
        currency_conversion::fetch_rate(&FrankfurterRateSource::new(), &from, &to)
    })
    .await
    .map_err(|e| e.to_string())?
}

/// Re-denominates habits' cost per unit and the profile's hourly income at
/// today's rate and switches the display currency, all in one transaction.
#[tauri::command]
pub async fn convert_currency(
    app: AppHandle,
    state: State<'_, AppState>,
    from: String,
    to: String,
) -> Result<CurrencyConversionDto, String> {
    let db = Arc::clone(&state.db);
    let result = tauri::async_runtime::spawn_blocking(move || {
        currency_conversion::convert_currency(
            &FrankfurterRateSource::new(),
            &SqliteCurrencyConversionRepository::new(db),
            &from,
            &to,
        )
    })
    .await
    .map_err(|e| e.to_string())??;
    notify_changed(&app, "habits");
    notify_changed(&app, "profile");
    notify_changed(&app, "settings");
    Ok(result.into())
}

#[tauri::command]
pub fn get_data_dir(state: State<AppState>) -> String {
    state
        .db_path
        .parent()
        .unwrap_or(&state.db_path)
        .display()
        .to_string()
}

#[tauri::command]
pub fn backup_database(state: State<AppState>, target: String) -> Result<(), String> {
    state
        .db
        .backup_to(&state.db_path, std::path::Path::new(&target))
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn restore_database(
    app: AppHandle,
    state: State<AppState>,
    source: String,
) -> Result<(), String> {
    state
        .db
        .restore_from(&state.db_path, std::path::Path::new(&source))
        .map_err(|e| e.to_string())?;
    notify_changed(&app, "habits");
    notify_changed(&app, "entries");
    notify_changed(&app, "profile");
    notify_changed(&app, "settings");
    Ok(())
}

/// Erases all user data. The Settings page asks for confirmation first.
#[tauri::command]
pub fn reset_database(app: AppHandle, state: State<AppState>) -> Result<(), String> {
    state.db.reset().map_err(|e| e.to_string())?;
    // Email secrets live in the OS keychain, outside the database. Absent
    // ones are skipped; any other keychain failure is logged rather than
    // failing a reset that has already happened.
    if let Err(e) = email::delete_all_secrets(&KeychainSecretStore::new()) {
        eprintln!("reset: couldn't remove email secrets from the keychain: {e}");
    }
    notify_changed(&app, "habits");
    notify_changed(&app, "entries");
    notify_changed(&app, "profile");
    notify_changed(&app, "settings");
    Ok(())
}

#[tauri::command]
pub fn export_theme(path: String, theme_json: String) -> Result<(), String> {
    std::fs::write(path, theme_json).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn import_theme(path: String) -> Result<String, String> {
    std::fs::read_to_string(path).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn compute_timeline(
    state: State<AppState>,
    start: String,
    end: String,
) -> Result<TimelineDto, String> {
    let habit_repo = SqliteHabitRepository::new(Arc::clone(&state.db));
    let entry_repo = SqliteEntryRepository::new(Arc::clone(&state.db));
    let start = parse_date(&start).map_err(|e| e.to_string())?;
    let end = parse_date(&end).map_err(|e| e.to_string())?;
    compute_series::compute_timeline(&habit_repo, &entry_repo, start, end)
        .map(Into::into)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn compute_today_summary(
    state: State<AppState>,
    today: String,
) -> Result<TodaySummaryDto, String> {
    let habit_repo = SqliteHabitRepository::new(Arc::clone(&state.db));
    let entry_repo = SqliteEntryRepository::new(Arc::clone(&state.db));
    let profile_repo = SqliteProfileRepository::new(Arc::clone(&state.db));
    let today = parse_date(&today).map_err(|e| e.to_string())?;
    today_summary::compute_today_summary(&habit_repo, &entry_repo, &profile_repo, today)
        .map(Into::into)
        .map_err(|e| e.to_string())
}

/// Also the points evaluation hook: `today` is the frontend's local date
/// (Rust has no reliable local clock here), and the progress store calls this
/// on startup and on every entries/habits/settings change. New awards are
/// announced with a "points" data-changed event.
#[tauri::command]
pub fn get_progress(
    app: AppHandle,
    state: State<AppState>,
    today: String,
) -> Result<ProgressDto, String> {
    let habit_repo = SqliteHabitRepository::new(Arc::clone(&state.db));
    let entry_repo = SqliteEntryRepository::new(Arc::clone(&state.db));
    let settings_repo = SqliteSettingsRepository::new(Arc::clone(&state.db));
    let award_repo = SqlitePointAwardRepository::new(Arc::clone(&state.db));
    let today = parse_date(&today).map_err(|e| e.to_string())?;
    let (progress, new_awards) = notifications::get_progress_with_new_awards(
        &habit_repo,
        &entry_repo,
        &settings_repo,
        &award_repo,
        today,
    )
    .map_err(|e| e.to_string())?;
    if progress.newly_awarded > 0 {
        notify_changed(&app, "points");
    }
    // Progress notifications go out from the scheduler thread, through the
    // same dedup and quiet-hours path as everything else.
    state.notifications.enqueue_awards(new_awards);
    Ok(progress.into())
}

#[tauri::command]
pub fn get_habit_presets() -> Vec<HabitPresetDto> {
    habit_presets().iter().map(Into::into).collect()
}

#[tauri::command]
pub fn preview_import(state: State<AppState>, path: String) -> Result<ImportPreviewDto, String> {
    let mapping_repo = SqliteMappingRepository::new(Arc::clone(&state.db));
    import_workbook::preview_import(std::path::Path::new(&path), &mapping_repo)
        .map(Into::into)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn apply_import(
    app: AppHandle,
    state: State<AppState>,
    path: String,
    sheet_index: usize,
    mapping: ColumnMapping,
) -> Result<ImportSummaryDto, String> {
    let habit_repo = SqliteHabitRepository::new(Arc::clone(&state.db));
    let entry_repo = SqliteEntryRepository::new(Arc::clone(&state.db));
    let mapping_repo = SqliteMappingRepository::new(Arc::clone(&state.db));
    let summary = import_workbook::apply_import(
        std::path::Path::new(&path),
        sheet_index,
        &mapping,
        &habit_repo,
        &entry_repo,
        &mapping_repo,
    )
    .map_err(|e| e.to_string())?;
    notify_changed(&app, "entries");
    notify_changed(&app, "habits");
    Ok(summary.into())
}

#[tauri::command]
pub fn export_entries(
    state: State<AppState>,
    path: String,
    format: String,
) -> Result<usize, String> {
    let habit_repo = SqliteHabitRepository::new(Arc::clone(&state.db));
    let entry_repo = SqliteEntryRepository::new(Arc::clone(&state.db));
    let format = match format.as_str() {
        "xlsx" => ExportFormat::Xlsx,
        "csv" => ExportFormat::Csv,
        "json" => ExportFormat::Json,
        other => return Err(format!("unsupported export format '{other}'")),
    };
    export_workbook::export_entries(
        std::path::Path::new(&path),
        format,
        &habit_repo,
        &entry_repo,
    )
    .map_err(|e| e.to_string())
}

// --- Email settings (N2) ---------------------------------------------------

#[tauri::command]
pub fn get_email_settings(state: State<AppState>) -> Result<EmailSettingsDto, String> {
    email::get_settings(
        &SqliteEmailSettingsRepository::new(Arc::clone(&state.db)),
        &KeychainSecretStore::new(),
    )
    .map(Into::into)
}

/// `new_secret` goes straight to the keychain; omitted or blank keeps the
/// saved one.
#[tauri::command]
pub fn save_email_settings(
    state: State<AppState>,
    settings: EmailSettingsDto,
    new_secret: Option<String>,
) -> Result<(), String> {
    let domain_settings: crate::domain::email::EmailSettings = settings.try_into()?;
    email::save_settings(
        &SqliteEmailSettingsRepository::new(Arc::clone(&state.db)),
        &KeychainSecretStore::new(),
        &domain_settings,
        new_secret.as_deref(),
    )
}

#[tauri::command]
pub fn clear_email_secret(state: State<AppState>) -> Result<(), String> {
    email::clear_secret(
        &SqliteEmailSettingsRepository::new(Arc::clone(&state.db)),
        &KeychainSecretStore::new(),
    )
}

/// Sends a test email with the saved settings. Async + spawn_blocking so
/// the SMTP/HTTPS round trip (up to 15 s) never stalls the main thread.
#[tauri::command]
pub async fn send_test_email(state: State<'_, AppState>) -> Result<(), String> {
    let db = Arc::clone(&state.db);
    tauri::async_runtime::spawn_blocking(move || {
        email::send_test(
            &SqliteEmailSettingsRepository::new(db),
            &KeychainSecretStore::new(),
            &ProviderTransports,
        )
    })
    .await
    .map_err(|e| e.to_string())?
}

// --- Notifications (N1) ----------------------------------------------------

fn notification_settings_dto(
    app: &AppHandle,
    db: &Arc<crate::infrastructure::db::Db>,
) -> Result<NotificationSettingsDto, String> {
    let repo = SqliteNotificationRepository::new(Arc::clone(db));
    let prefs = repo.get_prefs().map_err(|e| e.to_string())?;
    let schedule = repo.get_schedule().map_err(|e| e.to_string())?;
    // Configured = a provider is chosen; whether its secret works is what
    // the email card's own test button is for.
    let email_configured = SqliteEmailSettingsRepository::new(Arc::clone(db))
        .get()
        .map(|s| s.provider != EmailProvider::None)
        .unwrap_or(false);
    let permission = app
        .notification()
        .permission_state()
        .map(permission_label)
        .unwrap_or("prompt");
    Ok(NotificationSettingsDto::from_domain(
        &prefs,
        &schedule,
        email_configured,
        permission,
    ))
}

#[tauri::command]
pub fn get_notification_settings(
    app: AppHandle,
    state: State<AppState>,
) -> Result<NotificationSettingsDto, String> {
    notification_settings_dto(&app, &state.db)
}

/// Persists the notification settings and applies their side effects:
/// launch-at-login is registered with the OS (before saving, so a refusal
/// leaves the stored value matching reality), and enabling any local
/// channel asks for notification permission if it hasn't been granted.
/// Returns the saved settings with fresh status fields.
#[tauri::command]
pub fn save_notification_settings(
    app: AppHandle,
    state: State<AppState>,
    settings: NotificationSettingsDto,
) -> Result<NotificationSettingsDto, String> {
    let (prefs, schedule) = settings.to_domain()?;
    let repo = SqliteNotificationRepository::new(Arc::clone(&state.db));
    let previous = repo.get_schedule().map_err(|e| e.to_string())?;
    if previous.launch_at_login != schedule.launch_at_login {
        let autolaunch = app.autolaunch();
        let result = if schedule.launch_at_login {
            autolaunch.enable()
        } else {
            autolaunch.disable()
        };
        result.map_err(|e| format!("couldn't change launch at login: {e}"))?;
    }
    repo.save_prefs(&prefs).map_err(|e| e.to_string())?;
    repo.save_schedule(&schedule).map_err(|e| e.to_string())?;
    if prefs.any_local() {
        let notification = app.notification();
        if !matches!(
            notification.permission_state(),
            Ok(tauri::plugin::PermissionState::Granted)
        ) {
            if let Err(e) = notification.request_permission() {
                eprintln!("notification permission request failed: {e}");
            }
        }
    }
    // Re-evaluate straight away with the new schedule.
    state.notifications.wake();
    notification_settings_dto(&app, &state.db)
}

#[tauri::command]
pub fn send_test_notification(app: AppHandle) -> Result<(), String> {
    app.notification()
        .builder()
        .title("Ripple")
        .body("Notifications are working.")
        .show()
        .map_err(|e| e.to_string())
}
