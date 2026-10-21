use thiserror::Error;
use time::PrimitiveDateTime;

use super::entry::{Entry, EntryId};
use super::habit::{Habit, HabitId};
use super::profile::Profile;
use super::settings::Settings;

/// Repository ports are infrastructure-agnostic on purpose: domain code (and
/// its tests) never link against rusqlite, only this trait.
#[derive(Debug, Error)]
#[error("{0}")]
pub struct RepositoryError(pub String);

pub type RepoResult<T> = Result<T, RepositoryError>;

pub trait HabitRepository {
    fn insert(&self, habit: &Habit) -> RepoResult<HabitId>;
    fn update(&self, habit: &Habit) -> RepoResult<()>;
    fn get(&self, id: HabitId) -> RepoResult<Option<Habit>>;
    fn list(&self, include_archived: bool) -> RepoResult<Vec<Habit>>;
    fn archive(&self, id: HabitId) -> RepoResult<()>;
}

pub trait EntryRepository {
    fn insert(&self, entry: &Entry) -> RepoResult<EntryId>;
    fn update(&self, entry: &Entry) -> RepoResult<()>;
    fn delete(&self, id: EntryId) -> RepoResult<()>;
    fn get(&self, id: EntryId) -> RepoResult<Option<Entry>>;
    fn list_between(
        &self,
        start: PrimitiveDateTime,
        end: PrimitiveDateTime,
    ) -> RepoResult<Vec<Entry>>;
    fn list_all(&self) -> RepoResult<Vec<Entry>>;
}

pub trait ProfileRepository {
    fn get(&self) -> RepoResult<Profile>;
    fn save(&self, profile: &Profile) -> RepoResult<()>;
}

pub trait SettingsRepository {
    fn get(&self) -> RepoResult<Settings>;
    fn save(&self, settings: &Settings) -> RepoResult<()>;
}

/// Re-denominates every stored money amount in one go. A port of its own
/// rather than a method per repository, because the whole point is that the
/// habit, profile and settings writes commit or roll back together.
pub trait CurrencyConversionRepository {
    /// Multiplies each habit's cost per unit and the profile's net hourly
    /// income by `rate` and records `to` as the display currency — but only
    /// if the stored currency is still `from`, so a stale caller can't
    /// convert the same amounts twice. Returns how many habits were updated.
    fn convert_amounts(&self, from: &str, to: &str, rate: f64) -> RepoResult<usize>;
}
