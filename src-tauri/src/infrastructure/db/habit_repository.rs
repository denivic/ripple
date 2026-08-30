use std::sync::Arc;

use rusqlite::{params, OptionalExtension, Row};

use crate::domain::habit::{Habit, HabitId};
use crate::domain::repository::{HabitRepository, RepoResult, RepositoryError};

use super::Db;

pub struct SqliteHabitRepository {
    db: Arc<Db>,
}

impl SqliteHabitRepository {
    pub fn new(db: Arc<Db>) -> Self {
        Self { db }
    }
}

const COLUMNS: &str = "id, name, unit_label, life_minutes_per_unit, cost_per_unit, color, archived";

fn row_to_habit(row: &Row) -> rusqlite::Result<Habit> {
    Ok(Habit {
        id: Some(HabitId(row.get(0)?)),
        name: row.get(1)?,
        unit_label: row.get(2)?,
        life_minutes_per_unit: row.get(3)?,
        cost_per_unit: row.get(4)?,
        color: row.get(5)?,
        archived: row.get::<_, i64>(6)? != 0,
    })
}

impl HabitRepository for SqliteHabitRepository {
    fn insert(&self, habit: &Habit) -> RepoResult<HabitId> {
        let conn = self.db.lock()?;
        conn.execute(
            "INSERT INTO habits (name, unit_label, life_minutes_per_unit, cost_per_unit, color, archived)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                habit.name,
                habit.unit_label,
                habit.life_minutes_per_unit,
                habit.cost_per_unit,
                habit.color,
                habit.archived as i64,
            ],
        )?;
        Ok(HabitId(conn.last_insert_rowid()))
    }

    fn update(&self, habit: &Habit) -> RepoResult<()> {
        let id = habit
            .id
            .ok_or_else(|| RepositoryError("cannot update a habit without an id".into()))?;
        let conn = self.db.lock()?;
        conn.execute(
            "UPDATE habits SET name = ?1, unit_label = ?2, life_minutes_per_unit = ?3,
             cost_per_unit = ?4, color = ?5, archived = ?6 WHERE id = ?7",
            params![
                habit.name,
                habit.unit_label,
                habit.life_minutes_per_unit,
                habit.cost_per_unit,
                habit.color,
                habit.archived as i64,
                id.0,
            ],
        )?;
        Ok(())
    }

    fn get(&self, id: HabitId) -> RepoResult<Option<Habit>> {
        let conn = self.db.lock()?;
        conn.query_row(
            &format!("SELECT {COLUMNS} FROM habits WHERE id = ?1"),
            params![id.0],
            row_to_habit,
        )
        .optional()
        .map_err(Into::into)
    }

    fn list(&self, include_archived: bool) -> RepoResult<Vec<Habit>> {
        let conn = self.db.lock()?;
        let sql = if include_archived {
            format!("SELECT {COLUMNS} FROM habits ORDER BY name")
        } else {
            format!("SELECT {COLUMNS} FROM habits WHERE archived = 0 ORDER BY name")
        };
        let mut stmt = conn.prepare(&sql)?;
        let rows = stmt.query_map([], row_to_habit)?;
        rows.collect::<rusqlite::Result<Vec<_>>>()
            .map_err(Into::into)
    }

    fn archive(&self, id: HabitId) -> RepoResult<()> {
        let conn = self.db.lock()?;
        conn.execute(
            "UPDATE habits SET archived = 1 WHERE id = ?1",
            params![id.0],
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn repo() -> SqliteHabitRepository {
        SqliteHabitRepository::new(Arc::new(Db::open_in_memory().unwrap()))
    }

    #[test]
    fn insert_then_get_round_trips() {
        let repo = repo();
        let mut habit = Habit::new("Cigarettes", "cigarette");
        habit.life_minutes_per_unit = 11.0;
        habit.cost_per_unit = 0.6;
        habit.color = Some("#2DD4BF".to_string());
        let id = repo.insert(&habit).unwrap();

        let fetched = repo.get(id).unwrap().unwrap();
        assert_eq!(fetched.name, "Cigarettes");
        assert_eq!(fetched.life_minutes_per_unit, 11.0);
        assert_eq!(fetched.color.as_deref(), Some("#2DD4BF"));
        assert!(!fetched.archived);
    }

    #[test]
    fn list_excludes_archived_by_default() {
        let repo = repo();
        let id = repo.insert(&Habit::new("Cigarettes", "cigarette")).unwrap();
        repo.insert(&Habit::new("Alcohol", "drink")).unwrap();
        repo.archive(id).unwrap();

        assert_eq!(repo.list(false).unwrap().len(), 1);
        assert_eq!(repo.list(true).unwrap().len(), 2);
    }

    #[test]
    fn update_persists_changes() {
        let repo = repo();
        let id = repo.insert(&Habit::new("Cigarettes", "cigarette")).unwrap();
        let mut habit = repo.get(id).unwrap().unwrap();
        habit.cost_per_unit = 0.75;
        repo.update(&habit).unwrap();

        assert_eq!(repo.get(id).unwrap().unwrap().cost_per_unit, 0.75);
    }
}
