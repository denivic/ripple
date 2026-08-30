use time::PrimitiveDateTime;

use super::habit::HabitId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EntryId(pub i64);

/// `occurred_at` is wall-clock local time with no offset: habit logging is
/// inherently local ("what time did you do this"), and the hour-of-day /
/// day-of-week trigger chart depends on the user's own clock, not UTC.
#[derive(Debug, Clone, PartialEq)]
pub struct Entry {
    pub id: Option<EntryId>,
    pub habit_id: HabitId,
    pub occurred_at: PrimitiveDateTime,
    pub quantity: f64,
    pub duration_minutes: Option<f64>,
    pub note: Option<String>,
}

impl Entry {
    pub fn new(habit_id: HabitId, occurred_at: PrimitiveDateTime, quantity: f64) -> Self {
        Self {
            id: None,
            habit_id,
            occurred_at,
            quantity,
            duration_minutes: None,
            note: None,
        }
    }
}
