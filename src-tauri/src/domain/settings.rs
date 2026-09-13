/// Every field here mirrors a real Settings-page control (plan-v1.md's
/// Settings section) — no speculative fields.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StartupRoute {
    Today,
    Timeline,
    Ledger,
    Profile,
    Settings,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WeekStart {
    Monday,
    Sunday,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WeightUnit {
    Kg,
    Lb,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Settings {
    pub startup_route: StartupRoute,
    pub week_start: WeekStart,
    pub weight_unit: WeightUnit,
    /// Name of the active built-in theme, when the user hasn't diverged from
    /// a preset. Cleared whenever `custom_theme_json` is set, and vice versa.
    pub theme_name: Option<String>,
    pub custom_theme_json: Option<String>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            startup_route: StartupRoute::Today,
            week_start: WeekStart::Monday,
            weight_unit: WeightUnit::Kg,
            theme_name: None,
            custom_theme_json: None,
        }
    }
}
