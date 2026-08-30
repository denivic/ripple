#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
pub struct HabitId(pub i64);

#[derive(Debug, Clone, PartialEq)]
pub struct Habit {
    pub id: Option<HabitId>,
    pub name: String,
    pub unit_label: String,
    pub life_minutes_per_unit: f64,
    pub cost_per_unit: f64,
    pub color: Option<String>,
    pub archived: bool,
}

impl Habit {
    pub fn new(name: impl Into<String>, unit_label: impl Into<String>) -> Self {
        Self {
            id: None,
            name: name.into(),
            unit_label: unit_label.into(),
            life_minutes_per_unit: 0.0,
            cost_per_unit: 0.0,
            color: None,
            archived: false,
        }
    }
}
