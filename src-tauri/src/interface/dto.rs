use serde::{Deserialize, Serialize};

use crate::application::award_points::{MilestoneStatus, Progress};
use crate::application::compute_series::{
    DailyPoint, HabitBreakdownItem, PeriodCompareResult, TimelineResult,
};
use crate::application::convert_currency::CurrencyConversion;
use crate::application::import_workbook::{ImportPreview, ImportSummary};
use crate::application::today_summary::{HabitWeekTrend, TodaySummary};
use crate::domain::notifications::{
    format_clock, parse_clock, weekday_from_str, weekday_to_str, Category as NotificationCategory,
    CategoryPrefs, NotificationPrefs, NotificationSchedule, QuietHours,
};
use crate::domain::points::{AwardKind, PointAward};
use crate::domain::presets::{HabitPreset, PresetCategory, PresetConfidence};
use crate::domain::{
    is_currency_code, Entry, EntryId, Habit, HabitId, Profile, Settings, Sex, StartupRoute,
    TimeFormat, WeekStart, WeightUnit,
};
use crate::infrastructure::db::codec::{format_date, format_datetime, parse_date, parse_datetime};
use crate::infrastructure::import::Sheet;

/// Domain types never cross IPC directly — these DTOs are the serde-visible
/// boundary, so a domain refactor doesn't silently change the frontend's wire
/// format (and vice versa).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HabitDto {
    pub id: Option<i64>,
    pub name: String,
    pub unit_label: String,
    pub life_minutes_per_unit: f64,
    pub minutes_per_unit: f64,
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
            minutes_per_unit: h.minutes_per_unit,
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
        habit.minutes_per_unit = d.minutes_per_unit;
        habit.cost_per_unit = d.cost_per_unit;
        habit.color = d.color;
        habit.archived = d.archived;
        habit
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
#[serde(rename_all = "camelCase")]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SettingsDto {
    pub startup_route: String,
    pub week_start: String,
    pub weight_unit: String,
    pub time_format: String,
    pub theme_name: Option<String>,
    pub custom_theme_json: Option<String>,
    pub currency: String,
}

impl From<Settings> for SettingsDto {
    fn from(s: Settings) -> Self {
        Self {
            startup_route: match s.startup_route {
                StartupRoute::Today => "today",
                StartupRoute::Timeline => "timeline",
                StartupRoute::Ledger => "ledger",
                StartupRoute::Profile => "profile",
                StartupRoute::Settings => "settings",
            }
            .to_string(),
            week_start: match s.week_start {
                WeekStart::Monday => "monday",
                WeekStart::Sunday => "sunday",
            }
            .to_string(),
            weight_unit: match s.weight_unit {
                WeightUnit::Kg => "kg",
                WeightUnit::Lb => "lb",
            }
            .to_string(),
            time_format: match s.time_format {
                TimeFormat::H24 => "24h",
                TimeFormat::H12 => "12h",
            }
            .to_string(),
            theme_name: s.theme_name,
            custom_theme_json: s.custom_theme_json,
            currency: s.currency,
        }
    }
}

impl TryFrom<SettingsDto> for Settings {
    type Error = String;

    fn try_from(d: SettingsDto) -> Result<Self, Self::Error> {
        let startup_route = match d.startup_route.as_str() {
            "today" => StartupRoute::Today,
            "timeline" => StartupRoute::Timeline,
            "ledger" => StartupRoute::Ledger,
            "profile" => StartupRoute::Profile,
            "settings" => StartupRoute::Settings,
            other => return Err(format!("unknown startup route '{other}'")),
        };
        let week_start = match d.week_start.as_str() {
            "monday" => WeekStart::Monday,
            "sunday" => WeekStart::Sunday,
            other => return Err(format!("unknown week start '{other}'")),
        };
        let weight_unit = match d.weight_unit.as_str() {
            "kg" => WeightUnit::Kg,
            "lb" => WeightUnit::Lb,
            other => return Err(format!("unknown weight unit '{other}'")),
        };
        let time_format = match d.time_format.as_str() {
            "24h" => TimeFormat::H24,
            "12h" => TimeFormat::H12,
            other => return Err(format!("unknown time format '{other}'")),
        };
        if !is_currency_code(&d.currency) {
            return Err(format!("invalid currency code '{}'", d.currency));
        }
        Ok(Self {
            startup_route,
            week_start,
            weight_unit,
            time_format,
            theme_name: d.theme_name,
            custom_theme_json: d.custom_theme_json,
            currency: d.currency,
        })
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrencyConversionDto {
    pub rate: f64,
    pub converted_habits: usize,
}

impl From<CurrencyConversion> for CurrencyConversionDto {
    fn from(c: CurrencyConversion) -> Self {
        Self {
            rate: c.rate,
            converted_habits: c.converted_habits,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DailyPointDto {
    pub date: String,
    pub time_spent_minutes: f64,
    pub life_shortened_minutes: f64,
    pub money: f64,
}

impl From<DailyPoint> for DailyPointDto {
    fn from(d: DailyPoint) -> Self {
        Self {
            date: format_date(d.date),
            time_spent_minutes: d.time_spent_minutes,
            life_shortened_minutes: d.life_shortened_minutes,
            money: d.money,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HabitBreakdownItemDto {
    pub habit_id: i64,
    pub time_spent_minutes: f64,
    pub life_shortened_minutes: f64,
    pub money: f64,
}

impl From<HabitBreakdownItem> for HabitBreakdownItemDto {
    fn from(h: HabitBreakdownItem) -> Self {
        Self {
            habit_id: h.habit_id.0,
            time_spent_minutes: h.time_spent_minutes,
            life_shortened_minutes: h.life_shortened_minutes,
            money: h.money,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PeriodCompareDto {
    pub current_total_minutes: f64,
    pub previous_total_minutes: f64,
    pub delta_minutes: f64,
    pub percent_change: Option<f64>,
}

impl From<PeriodCompareResult> for PeriodCompareDto {
    fn from(p: PeriodCompareResult) -> Self {
        Self {
            current_total_minutes: p.current_total_minutes,
            previous_total_minutes: p.previous_total_minutes,
            delta_minutes: p.delta_minutes,
            percent_change: p.percent_change,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TimelineDto {
    pub daily: Vec<DailyPointDto>,
    pub habit_breakdown: Vec<HabitBreakdownItemDto>,
    /// `[weekday 0=Mon..6=Sun][hour 0..23]`
    pub hour_weekday_matrix: Vec<Vec<f64>>,
    pub current_streak_days: i64,
    pub longest_streak_days: i64,
    pub period_compare: PeriodCompareDto,
}

impl From<TimelineResult> for TimelineDto {
    fn from(t: TimelineResult) -> Self {
        Self {
            daily: t.daily.into_iter().map(Into::into).collect(),
            habit_breakdown: t.habit_breakdown.into_iter().map(Into::into).collect(),
            hour_weekday_matrix: t
                .hour_weekday_matrix
                .into_iter()
                .map(|row| row.to_vec())
                .collect(),
            current_streak_days: t.current_streak_days,
            longest_streak_days: t.longest_streak_days,
            period_compare: t.period_compare.into(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TodaySummaryDto {
    pub time_spent_minutes: f64,
    pub life_shortened_minutes: f64,
    pub money_spent: f64,
    pub opportunity_cost: Option<f64>,
    pub waking_life_share_today: f64,
    pub projected_waking_months_lost: Option<f64>,
    pub projection_window_days: u32,
    pub average_daily_life_minutes: f64,
    pub life_expectancy_age: Option<f64>,
    pub current_streak_days: i64,
    pub previous_week_life_minutes: f64,
    pub current_week_life_minutes: f64,
    pub week_change_fraction: Option<f64>,
    pub months_regained: Option<f64>,
    pub habit_week_trends: Vec<HabitWeekTrendDto>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HabitWeekTrendDto {
    pub habit_id: i64,
    pub current_week_quantity: f64,
    pub previous_week_quantity: f64,
}

impl From<HabitWeekTrend> for HabitWeekTrendDto {
    fn from(t: HabitWeekTrend) -> Self {
        Self {
            habit_id: t.habit_id.0,
            current_week_quantity: t.current_week_quantity,
            previous_week_quantity: t.previous_week_quantity,
        }
    }
}

impl From<TodaySummary> for TodaySummaryDto {
    fn from(s: TodaySummary) -> Self {
        Self {
            time_spent_minutes: s.time_spent_minutes,
            life_shortened_minutes: s.life_shortened_minutes,
            money_spent: s.money_spent,
            opportunity_cost: s.opportunity_cost,
            waking_life_share_today: s.waking_life_share_today,
            projected_waking_months_lost: s.projected_waking_months_lost,
            projection_window_days: s.projection_window_days,
            average_daily_life_minutes: s.average_daily_life_minutes,
            life_expectancy_age: s.life_expectancy_age,
            current_streak_days: s.current_streak_days,
            previous_week_life_minutes: s.previous_week_life_minutes,
            current_week_life_minutes: s.current_week_life_minutes,
            week_change_fraction: s.week_change_fraction,
            months_regained: s.months_regained,
            habit_week_trends: s.habit_week_trends.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HabitPresetDto {
    pub id: String,
    pub name: String,
    pub unit_label: String,
    /// Imperial variant of `unit_label`; null when it reads the same.
    pub unit_label_imperial: Option<String>,
    /// Positive = life lost per unit, negative = life gained.
    pub life_minutes_per_unit: f64,
    /// Editable default for time spent per unit, not a research figure.
    pub minutes_per_unit: f64,
    pub category: String,
    pub source_title: Option<String>,
    pub source_url: Option<String>,
    pub year: Option<u16>,
    pub confidence: String,
    pub derived: bool,
    pub derivation: Option<String>,
    pub note: String,
}

impl From<&HabitPreset> for HabitPresetDto {
    fn from(p: &HabitPreset) -> Self {
        Self {
            id: p.id.to_string(),
            name: p.name.to_string(),
            unit_label: p.unit_label.to_string(),
            unit_label_imperial: p.unit_label_imperial.map(str::to_string),
            life_minutes_per_unit: p.life_minutes_per_unit,
            minutes_per_unit: p.minutes_per_unit,
            category: match p.category {
                PresetCategory::Tobacco => "tobacco",
                PresetCategory::Alcohol => "alcohol",
                PresetCategory::Cannabis => "cannabis",
                PresetCategory::Food => "food",
                PresetCategory::Drink => "drink",
                PresetCategory::Sedentary => "sedentary",
                PresetCategory::Gain => "gain",
            }
            .to_string(),
            source_title: p.source_title.map(str::to_string),
            source_url: p.source_url.map(str::to_string),
            year: p.year,
            confidence: match p.confidence {
                PresetConfidence::High => "high",
                PresetConfidence::Medium => "medium",
                PresetConfidence::Low => "low",
                PresetConfidence::Insufficient => "insufficient",
            }
            .to_string(),
            derived: p.derived,
            derivation: p.derivation.map(str::to_string),
            note: p.note.to_string(),
        }
    }
}

/// A preview-only, display-string rendering of a sheet's cells — the
/// mapping wizard just needs something to show the user, not typed values.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SheetPreviewDto {
    pub name: String,
    pub rows: Vec<Vec<String>>,
}

impl From<Sheet> for SheetPreviewDto {
    fn from(s: Sheet) -> Self {
        Self {
            name: s.name,
            rows: s
                .rows
                .into_iter()
                .map(|row| {
                    row.iter()
                        .map(|c| c.as_text().unwrap_or_default())
                        .collect()
                })
                .collect(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportPreviewDto {
    pub sheets: Vec<SheetPreviewDto>,
    pub source_signature: String,
    pub remembered_mapping: Option<crate::infrastructure::import::mapping::ColumnMapping>,
}

impl From<ImportPreview> for ImportPreviewDto {
    fn from(p: ImportPreview) -> Self {
        Self {
            sheets: p.sheets.into_iter().map(Into::into).collect(),
            source_signature: p.source_signature,
            remembered_mapping: p.remembered_mapping,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportSummaryDto {
    pub entries_created: usize,
    pub row_errors: Vec<crate::infrastructure::import::mapping::RowError>,
}

impl From<ImportSummary> for ImportSummaryDto {
    fn from(s: ImportSummary) -> Self {
        Self {
            entries_created: s.entries_created,
            row_errors: s.row_errors,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PointAwardDto {
    pub id: i64,
    pub kind: &'static str,
    pub points: i64,
    pub awarded_for: String,
    /// Display text, e.g. "Clean day" or "Cigarettes improved".
    pub label: String,
}

impl From<PointAward> for PointAwardDto {
    fn from(a: PointAward) -> Self {
        Self {
            id: a.id.unwrap_or_default(),
            kind: match a.kind {
                AwardKind::LoggedDay => "logged",
                AwardKind::CleanDay => "clean",
                AwardKind::HabitImproved => "habit-improved",
                AwardKind::StreakBonus => "streak",
                AwardKind::Milestone => "milestone",
            },
            points: a.points,
            awarded_for: format_date(a.awarded_for),
            label: a.label(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MilestoneDto {
    pub key: &'static str,
    pub title: &'static str,
    pub description: &'static str,
    pub points: i64,
    /// `None` while still locked.
    pub achieved_on: Option<String>,
}

impl From<MilestoneStatus> for MilestoneDto {
    fn from(m: MilestoneStatus) -> Self {
        Self {
            key: m.milestone.key,
            title: m.milestone.title,
            description: m.milestone.description,
            points: m.milestone.points,
            achieved_on: m.achieved_on.map(format_date),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgressDto {
    pub total_points: i64,
    pub level: &'static str,
    pub level_index: usize,
    /// `None` at the top level.
    pub next_level: Option<&'static str>,
    pub points_to_next: Option<i64>,
    pub level_progress_fraction: f64,
    pub recent_awards: Vec<PointAwardDto>,
    pub milestones: Vec<MilestoneDto>,
}

impl From<Progress> for ProgressDto {
    fn from(p: Progress) -> Self {
        Self {
            total_points: p.total_points,
            level: p.level.name,
            level_index: p.level.index,
            next_level: p.level.next_name,
            points_to_next: p.level.points_to_next,
            level_progress_fraction: p.level.fraction,
            recent_awards: p.recent_awards.into_iter().map(Into::into).collect(),
            milestones: p.milestones.into_iter().map(Into::into).collect(),
        }
    }
}

// --- Email settings (N2) ---------------------------------------------------

/// The email section of Settings. Deliberately has no secret field in
/// either direction: the frontend learns only `has_secret`, and a new secret
/// travels as its own `save_email_settings` argument straight to the keychain.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmailSettingsDto {
    pub provider: String,
    pub from_address: Option<String>,
    pub to_address: Option<String>,
    pub smtp_host: Option<String>,
    /// Wider than u16 so an out-of-range port is a validation message
    /// rather than a deserialisation error.
    pub smtp_port: Option<i64>,
    pub smtp_username: Option<String>,
    pub smtp_security: String,
    /// Output only; ignored on save.
    #[serde(default)]
    pub has_secret: bool,
}

impl From<crate::application::email::EmailSettingsView> for EmailSettingsDto {
    fn from(view: crate::application::email::EmailSettingsView) -> Self {
        let s = view.settings;
        Self {
            provider: s.provider.as_str().to_string(),
            from_address: s.from_address,
            to_address: s.to_address,
            smtp_host: s.smtp_host,
            smtp_port: s.smtp_port.map(i64::from),
            smtp_username: s.smtp_username,
            smtp_security: s.smtp_security.as_str().to_string(),
            has_secret: view.has_secret,
        }
    }
}

impl TryFrom<EmailSettingsDto> for crate::domain::email::EmailSettings {
    type Error = String;

    fn try_from(d: EmailSettingsDto) -> Result<Self, Self::Error> {
        use crate::domain::email::{EmailProvider, SmtpSecurity};
        // Blank form fields arrive as "" — store them as absent.
        let clean = |v: Option<String>| {
            v.map(|s| s.trim().to_string()).filter(|s| !s.is_empty())
        };
        let provider = EmailProvider::parse(&d.provider)
            .ok_or_else(|| format!("unknown email provider '{}'", d.provider))?;
        let smtp_security = SmtpSecurity::parse(&d.smtp_security)
            .ok_or_else(|| format!("unknown SMTP security '{}'", d.smtp_security))?;
        let smtp_port = match d.smtp_port {
            None => None,
            Some(p) => Some(
                u16::try_from(p)
                    .ok()
                    .filter(|p| *p > 0)
                    .ok_or("SMTP port must be between 1 and 65535")?,
            ),
        };
        Ok(Self {
            provider,
            from_address: clean(d.from_address),
            to_address: clean(d.to_address),
            smtp_host: clean(d.smtp_host),
            smtp_port,
            smtp_username: clean(d.smtp_username),
            smtp_security,
        })
    }
}

#[cfg(test)]
mod email_dto_tests {
    use super::*;
    use crate::application::email::{get_settings, save_settings};
    use crate::domain::email::{EmailSettings, EmailSettingsRepository, SecretStore};
    use crate::domain::repository::RepoResult;
    use crate::infrastructure::email::keychain::MemorySecretStore;

    struct OneRow(std::sync::Mutex<EmailSettings>);

    impl EmailSettingsRepository for OneRow {
        fn get(&self) -> RepoResult<EmailSettings> {
            Ok(self.0.lock().unwrap().clone())
        }
        fn save(&self, s: &EmailSettings) -> RepoResult<()> {
            *self.0.lock().unwrap() = s.clone();
            Ok(())
        }
    }

    fn dto() -> EmailSettingsDto {
        EmailSettingsDto {
            provider: "smtp".to_string(),
            from_address: Some(" me@example.com ".to_string()),
            to_address: Some("me@example.com".to_string()),
            smtp_host: Some("smtp.example.com".to_string()),
            smtp_port: Some(465),
            smtp_username: Some("".to_string()),
            smtp_security: "tls".to_string(),
            has_secret: true,
        }
    }

    #[test]
    fn secret_never_appears_in_the_serialized_dto() {
        let repo = OneRow(std::sync::Mutex::new(EmailSettings::default()));
        let secrets = MemorySecretStore::default();
        let settings = EmailSettings {
            smtp_username: Some("me".to_string()),
            ..EmailSettings::try_from(dto()).unwrap()
        };
        save_settings(&repo, &secrets, &settings, Some("sup3r-secret-pw")).unwrap();
        assert_eq!(
            secrets.get("smtp-password").unwrap().as_deref(),
            Some("sup3r-secret-pw")
        );
        let json = serde_json::to_string(&EmailSettingsDto::from(
            get_settings(&repo, &secrets).unwrap(),
        ))
        .unwrap();
        assert!(!json.contains("sup3r-secret-pw"));
        assert!(json.contains("\"hasSecret\":true"));
        assert!(json.contains("\"smtpSecurity\":\"tls\""));
    }

    #[test]
    fn blank_fields_become_absent_and_bad_values_are_rejected() {
        let s = EmailSettings::try_from(dto()).unwrap();
        assert_eq!(s.from_address.as_deref(), Some("me@example.com"));
        assert_eq!(s.smtp_username, None);
        for port in [0, -1, 70000] {
            let err = EmailSettings::try_from(EmailSettingsDto { smtp_port: Some(port), ..dto() });
            assert_eq!(err.unwrap_err(), "SMTP port must be between 1 and 65535");
        }
        assert!(EmailSettings::try_from(EmailSettingsDto { provider: "fax".to_string(), ..dto() }).is_err());
        assert!(EmailSettings::try_from(EmailSettingsDto { smtp_security: "none".to_string(), ..dto() }).is_err());
    }

    #[test]
    fn has_secret_is_optional_on_input() {
        let parsed: EmailSettingsDto = serde_json::from_str(
            r#"{"provider":"none","fromAddress":null,"toAddress":null,"smtpHost":null,"smtpPort":null,"smtpUsername":null,"smtpSecurity":"starttls"}"#,
        )
        .unwrap();
        assert!(!parsed.has_secret);
    }
}

// --- end email settings (N2) ------------------------------------------------

// --- Notifications (N1) -----------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotificationCategoryDto {
    /// reminders|progress|weekly_summary|streak_pace
    pub category: String,
    pub enabled: bool,
    pub local: bool,
    pub email: bool,
}

/// Times are "HH:MM"; quiet hours are off when either end is null.
/// `email_configured` and `permission` are read-only status for the page and
/// ignored on save.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotificationSettingsDto {
    pub categories: Vec<NotificationCategoryDto>,
    pub reminder_time: String,
    pub weekly_day: String,
    pub weekly_time: String,
    pub streak_alert_time: String,
    pub pace_threshold: f64,
    pub quiet_start: Option<String>,
    pub quiet_end: Option<String>,
    pub bundle_progress: bool,
    pub launch_at_login: bool,
    pub close_to_tray: bool,
    #[serde(default)]
    pub email_configured: bool,
    /// "granted" | "denied" | "prompt"
    #[serde(default)]
    pub permission: String,
}

impl NotificationSettingsDto {
    pub fn from_domain(
        prefs: &NotificationPrefs,
        s: &NotificationSchedule,
        email_configured: bool,
        permission: &str,
    ) -> Self {
        Self {
            categories: NotificationCategory::ALL
                .iter()
                .map(|c| {
                    let p = prefs.get(*c);
                    NotificationCategoryDto {
                        category: c.id().to_string(),
                        enabled: p.enabled,
                        local: p.local,
                        email: p.email,
                    }
                })
                .collect(),
            reminder_time: format_clock(s.reminder_time),
            weekly_day: weekday_to_str(s.weekly_day).to_string(),
            weekly_time: format_clock(s.weekly_time),
            streak_alert_time: format_clock(s.streak_alert_time),
            pace_threshold: s.pace_threshold,
            quiet_start: s.quiet_hours.map(|q| format_clock(q.start)),
            quiet_end: s.quiet_hours.map(|q| format_clock(q.end)),
            bundle_progress: s.bundle_progress,
            launch_at_login: s.launch_at_login,
            close_to_tray: s.close_to_tray,
            email_configured,
            permission: permission.to_string(),
        }
    }

    pub fn to_domain(&self) -> Result<(NotificationPrefs, NotificationSchedule), String> {
        let mut prefs = NotificationPrefs::default();
        for c in &self.categories {
            let category = NotificationCategory::from_id(&c.category)
                .ok_or_else(|| format!("unknown notification category '{}'", c.category))?;
            prefs.set(
                category,
                CategoryPrefs {
                    enabled: c.enabled,
                    local: c.local,
                    email: c.email,
                },
            );
        }
        let clock = |label: &str, v: &str| {
            parse_clock(v).ok_or_else(|| format!("invalid {label} '{v}' (expected HH:MM)"))
        };
        let quiet_hours = match (self.quiet_start.as_deref(), self.quiet_end.as_deref()) {
            (Some(start), Some(end)) => Some(QuietHours {
                start: clock("quiet hours start", start)?,
                end: clock("quiet hours end", end)?,
            }),
            _ => None,
        };
        if !(self.pace_threshold.is_finite() && self.pace_threshold > 0.0) {
            return Err("pace threshold must be a positive number".to_string());
        }
        Ok((
            prefs,
            NotificationSchedule {
                reminder_time: clock("reminder time", &self.reminder_time)?,
                weekly_day: weekday_from_str(&self.weekly_day)
                    .ok_or_else(|| format!("unknown weekday '{}'", self.weekly_day))?,
                weekly_time: clock("weekly time", &self.weekly_time)?,
                streak_alert_time: clock("streak alert time", &self.streak_alert_time)?,
                pace_threshold: self.pace_threshold,
                quiet_hours,
                bundle_progress: self.bundle_progress,
                launch_at_login: self.launch_at_login,
                close_to_tray: self.close_to_tray,
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::presets::habit_presets;

    #[test]
    fn habit_preset_dto_carries_source_and_confidence() {
        let cigarette = habit_presets()
            .iter()
            .find(|p| p.id == "cigarette")
            .unwrap();
        let dto = HabitPresetDto::from(cigarette);
        assert_eq!(dto.id, "cigarette");
        assert_eq!(dto.unit_label, cigarette.unit_label);
        assert_eq!(dto.life_minutes_per_unit, cigarette.life_minutes_per_unit);
        assert_eq!(dto.category, "tobacco");
        assert_eq!(dto.confidence, "high");
        assert_eq!(dto.source_url.as_deref(), cigarette.source_url);
        assert_eq!(dto.year, cigarette.year);
    }

    #[test]
    fn habit_preset_dto_serializes_camel_case_with_nulls() {
        let cigar = habit_presets().iter().find(|p| p.id == "cigar").unwrap();
        let json = serde_json::to_value(HabitPresetDto::from(cigar)).unwrap();
        assert_eq!(json["unitLabel"], "cigar");
        assert_eq!(json["lifeMinutesPerUnit"], 0.0);
        assert_eq!(json["minutesPerUnit"], 45.0);
        assert_eq!(json["confidence"], "insufficient");
        assert!(json["sourceUrl"].is_null());
        assert!(json["year"].is_null());
        assert!(json["unitLabelImperial"].is_null());
    }

    #[test]
    fn notification_settings_dto_round_trips_and_validates() {
        use crate::domain::notifications::{NotificationPrefs, NotificationSchedule};
        let mut prefs = NotificationPrefs::default();
        prefs.progress.email = true;
        let schedule = NotificationSchedule {
            quiet_hours: Some(QuietHours {
                start: time::macros::time!(22:00),
                end: time::macros::time!(07:00),
            }),
            ..Default::default()
        };
        let dto = NotificationSettingsDto::from_domain(&prefs, &schedule, true, "granted");
        let json = serde_json::to_value(&dto).unwrap();
        assert_eq!(json["reminderTime"], "20:00");
        assert_eq!(json["quietStart"], "22:00");
        assert_eq!(json["categories"][1]["category"], "progress");
        assert_eq!(dto.to_domain().unwrap(), (prefs, schedule));

        let bad = NotificationSettingsDto {
            reminder_time: "8pm".to_string(),
            ..dto.clone()
        };
        assert!(bad.to_domain().is_err());
        let bad = NotificationSettingsDto {
            pace_threshold: 0.0,
            ..dto
        };
        assert!(bad.to_domain().is_err());
    }
}
