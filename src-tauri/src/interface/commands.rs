use std::sync::Arc;

use serde::Serialize;
use tauri::{AppHandle, Emitter, State};

use crate::application::export_workbook::ExportFormat;
use crate::application::{
    award_points, compute_series, convert_currency as currency_conversion, export_workbook,
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
fn notify_changed(app: &AppHandle, scope: &'static str) {
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
    let progress =
        award_points::get_progress(&habit_repo, &entry_repo, &settings_repo, &award_repo, today)
            .map_err(|e| e.to_string())?;
    if progress.newly_awarded > 0 {
        notify_changed(&app, "points");
    }
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
