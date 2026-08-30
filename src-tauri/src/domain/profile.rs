use time::Date;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sex {
    Male,
    Female,
}

/// Every field here must feed a metric shown elsewhere, or it doesn't belong —
/// see plan-v1.md "Making the profile load-bearing". All are optional because
/// the app must be useful before onboarding is complete; metrics that need a
/// missing field simply don't render rather than guessing.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Profile {
    pub birth_date: Option<Date>,
    pub sex: Option<Sex>,
    pub life_expectancy_years: Option<f64>,
    pub typical_sleep_hours: Option<f64>,
    pub net_hourly_income: Option<f64>,
    pub weight_kg: Option<f64>,
}

impl Profile {
    // Consumed by cost_model (Phase 4); fully unit-tested already.
    #[allow(dead_code)]
    pub fn waking_hours_per_day(&self) -> f64 {
        (24.0 - self.typical_sleep_hours.unwrap_or(8.0)).max(0.0)
    }
}
