use std::sync::Arc;

use serde::Serialize;
use tauri::{AppHandle, Emitter, State};

use crate::application::export_workbook::ExportFormat;
use crate::application::{compute_series, export_workbook, import_workbook, today_summary};
use crate::domain::cost_model::default_habit_presets;
use crate::domain::repository::{EntryRepository, HabitRepository, ProfileRepository};
use crate::infrastructure::db::codec::{parse_date, parse_datetime};
use crate::infrastructure::db::{
    SqliteEntryRepository, SqliteHabitRepository, SqliteMappingRepository, SqliteProfileRepository,
};
use crate::infrastructure::import::mapping::ColumnMapping;

use super::dto::{
    EntryDto, HabitDto, HabitPresetDto, ImportPreviewDto, ImportSummaryDto, ProfileDto,
    TimelineDto, TodaySummaryDto,
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

#[tauri::command]
pub fn get_habit_presets() -> Vec<HabitPresetDto> {
    default_habit_presets()
        .into_iter()
        .map(Into::into)
        .collect()
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
