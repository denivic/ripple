use serde::{Deserialize, Serialize};

use crate::domain::{Entry, EntryId, Habit, HabitId, Profile, Sex};
use crate::infrastructure::db::codec::{format_date, format_datetime, parse_date, parse_datetime};

/// Domain types never cross IPC directly — these DTOs are the serde-visible
/// boundary, so a domain refactor doesn't silently change the frontend's wire
/// format (and vice versa).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HabitDto {
    pub id: Option<i64>,
    pub name: String,
    pub unit_label: String,
    pub life_minutes_per_unit: f64,
    pub cost_per_unit: f64,
    pub color: Option<String>,
    pub archived: bool,
}

impl From<Habit> for HabitDto {
    fn from(h: Habit) -> Self {
        Self {
            id: h.id.map(|id| id.0),
            name: h.name,
            unit_label: h.unit_label,
            life_minutes_per_unit: h.life_minutes_per_unit,
            cost_per_unit: h.cost_per_unit,
            color: h.color,
            archived: h.archived,
        }
    }
}

impl From<HabitDto> for Habit {
    fn from(d: HabitDto) -> Self {
        let mut habit = Habit::new(d.name, d.unit_label);
        habit.id = d.id.map(HabitId);
        habit.life_minutes_per_unit = d.life_minutes_per_unit;
        habit.cost_per_unit = d.cost_per_unit;
        habit.color = d.color;
        habit.archived = d.archived;
        habit
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntryDto {
    pub id: Option<i64>,
    pub habit_id: i64,
    /// Local wall-clock time, `YYYY-MM-DDTHH:MM:SS` — see [`Entry::occurred_at`].
    pub occurred_at: String,
    pub quantity: f64,
    pub duration_minutes: Option<f64>,
    pub note: Option<String>,
}

impl From<Entry> for EntryDto {
    fn from(e: Entry) -> Self {
        Self {
            id: e.id.map(|id| id.0),
            habit_id: e.habit_id.0,
            occurred_at: format_datetime(e.occurred_at),
            quantity: e.quantity,
            duration_minutes: e.duration_minutes,
            note: e.note,
        }
    }
}

impl TryFrom<EntryDto> for Entry {
    type Error = String;

    fn try_from(d: EntryDto) -> Result<Self, Self::Error> {
        let occurred_at = parse_datetime(&d.occurred_at).map_err(|e| e.to_string())?;
        let mut entry = Entry::new(HabitId(d.habit_id), occurred_at, d.quantity);
        entry.id = d.id.map(EntryId);
        entry.duration_minutes = d.duration_minutes;
        entry.note = d.note;
        Ok(entry)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProfileDto {
    pub birth_date: Option<String>,
    pub sex: Option<String>,
    pub life_expectancy_years: Option<f64>,
    pub typical_sleep_hours: Option<f64>,
    pub net_hourly_income: Option<f64>,
    pub weight_kg: Option<f64>,
}

impl From<Profile> for ProfileDto {
    fn from(p: Profile) -> Self {
        Self {
            birth_date: p.birth_date.map(format_date),
            sex: p.sex.map(|s| match s {
                Sex::Male => "male".to_string(),
                Sex::Female => "female".to_string(),
            }),
            life_expectancy_years: p.life_expectancy_years,
            typical_sleep_hours: p.typical_sleep_hours,
            net_hourly_income: p.net_hourly_income,
            weight_kg: p.weight_kg,
        }
    }
}

impl TryFrom<ProfileDto> for Profile {
    type Error = String;

    fn try_from(d: ProfileDto) -> Result<Self, Self::Error> {
        let birth_date = d
            .birth_date
            .map(|s| parse_date(&s))
            .transpose()
            .map_err(|e| e.to_string())?;
        let sex = d
            .sex
            .map(|s| match s.as_str() {
                "male" => Ok(Sex::Male),
                "female" => Ok(Sex::Female),
                other => Err(format!("unknown sex '{other}'")),
            })
            .transpose()?;
        Ok(Self {
            birth_date,
            sex,
            life_expectancy_years: d.life_expectancy_years,
            typical_sleep_hours: d.typical_sleep_hours,
            net_hourly_income: d.net_hourly_income,
            weight_kg: d.weight_kg,
        })
    }
}
