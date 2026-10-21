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
    /// ISO 4217 code (e.g. "USD", "EUR") money amounts are displayed in.
    /// Kept as a plain code rather than an enum: the frontend offers every
    /// currency `Intl` knows, and conversion between them is a separate step.
    pub currency: String,
}

pub const DEFAULT_CURRENCY: &str = "USD";

/// Shape check only (three ASCII uppercase letters) — whether the code is a
/// currency `Intl` can format is the frontend's concern.
pub fn is_currency_code(s: &str) -> bool {
    s.len() == 3 && s.bytes().all(|b| b.is_ascii_uppercase())
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            startup_route: StartupRoute::Today,
            week_start: WeekStart::Monday,
            weight_unit: WeightUnit::Kg,
            theme_name: None,
            custom_theme_json: None,
            currency: DEFAULT_CURRENCY.to_string(),
        }
    }
}
