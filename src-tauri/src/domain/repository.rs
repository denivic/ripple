use thiserror::Error;
use time::PrimitiveDateTime;

use super::entry::{Entry, EntryId};
use super::habit::{Habit, HabitId};
use super::profile::Profile;

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
