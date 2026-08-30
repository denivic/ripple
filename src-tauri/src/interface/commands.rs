use std::sync::Arc;

use tauri::State;

use crate::domain::repository::{EntryRepository, HabitRepository, ProfileRepository};
use crate::infrastructure::db::codec::parse_datetime;
use crate::infrastructure::db::{
    SqliteEntryRepository, SqliteHabitRepository, SqliteProfileRepository,
};

use super::dto::{EntryDto, HabitDto, ProfileDto};
use super::state::AppState;

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
pub fn create_habit(state: State<AppState>, habit: HabitDto) -> Result<HabitDto, String> {
    let repo = SqliteHabitRepository::new(Arc::clone(&state.db));
    let mut domain_habit = crate::domain::Habit::from(habit);
    let id = repo.insert(&domain_habit).map_err(|e| e.to_string())?;
    domain_habit.id = Some(id);
    Ok(domain_habit.into())
}

#[tauri::command]
pub fn update_habit(state: State<AppState>, habit: HabitDto) -> Result<(), String> {
    SqliteHabitRepository::new(Arc::clone(&state.db))
        .update(&habit.into())
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn archive_habit(state: State<AppState>, habit_id: i64) -> Result<(), String> {
    SqliteHabitRepository::new(Arc::clone(&state.db))
        .archive(crate::domain::HabitId(habit_id))
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn log_entry(state: State<AppState>, entry: EntryDto) -> Result<EntryDto, String> {
    let repo = SqliteEntryRepository::new(Arc::clone(&state.db));
    let mut domain_entry: crate::domain::Entry = entry.try_into()?;
    let id = repo.insert(&domain_entry).map_err(|e| e.to_string())?;
    domain_entry.id = Some(id);
    Ok(domain_entry.into())
}

#[tauri::command]
pub fn update_entry(state: State<AppState>, entry: EntryDto) -> Result<(), String> {
    let domain_entry: crate::domain::Entry = entry.try_into()?;
    SqliteEntryRepository::new(Arc::clone(&state.db))
        .update(&domain_entry)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_entry(state: State<AppState>, entry_id: i64) -> Result<(), String> {
    SqliteEntryRepository::new(Arc::clone(&state.db))
        .delete(crate::domain::EntryId(entry_id))
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_entry(state: State<AppState>, entry_id: i64) -> Result<Option<EntryDto>, String> {
    SqliteEntryRepository::new(Arc::clone(&state.db))
        .get(crate::domain::EntryId(entry_id))
        .map(|entry| entry.map(Into::into))
        .map_err(|e| e.to_string())
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
pub fn save_profile(state: State<AppState>, profile: ProfileDto) -> Result<(), String> {
    let domain_profile: crate::domain::Profile = profile.try_into()?;
    SqliteProfileRepository::new(Arc::clone(&state.db))
        .save(&domain_profile)
        .map_err(|e| e.to_string())
}
