use serde::{Deserialize, Serialize};

use crate::application::compute_series::{
    DailyPoint, HabitBreakdownItem, PeriodCompareResult, TimelineResult,
};
use crate::application::import_workbook::{ImportPreview, ImportSummary};
use crate::application::today_summary::TodaySummary;
use crate::domain::cost_model::HabitPreset;
use crate::domain::{Entry, EntryId, Habit, HabitId, Profile, Sex};
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
    pub remaining_waking_life_months_at_todays_rate: Option<f64>,
}

impl From<TodaySummary> for TodaySummaryDto {
    fn from(s: TodaySummary) -> Self {
        Self {
            time_spent_minutes: s.time_spent_minutes,
            life_shortened_minutes: s.life_shortened_minutes,
            money_spent: s.money_spent,
            opportunity_cost: s.opportunity_cost,
            waking_life_share_today: s.waking_life_share_today,
            remaining_waking_life_months_at_todays_rate: s
                .remaining_waking_life_months_at_todays_rate,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HabitPresetDto {
    pub name: String,
    pub unit_label: String,
    pub life_minutes_per_unit: f64,
}

impl From<HabitPreset> for HabitPresetDto {
    fn from(p: HabitPreset) -> Self {
        Self {
            name: p.name.to_string(),
            unit_label: p.unit_label.to_string(),
            life_minutes_per_unit: p.life_minutes_per_unit,
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
