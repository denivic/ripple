use std::collections::HashMap;

use time::Date;
use time::Duration;

use super::cost_model::{money_spent_for_entry, time_loss_for_entry, TimeLoss};
use super::entry::Entry;
use super::habit::{Habit, HabitId};

/// TimeSpent/LifeShortened/Money mirror plan-v1.md's three toggleable
/// breakdown metrics; only TotalTimeLost has a live Rust caller today (the
/// application layer sends the full per-day/per-habit breakdown over IPC and
/// lets the frontend pick), but the type is real domain vocabulary, not
/// speculative, so it stays complete rather than trimmed to one variant.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Metric {
    TimeSpent,
    LifeShortened,
    TotalTimeLost,
    Money,
}

fn metric_value(loss: TimeLoss, money: f64, metric: Metric) -> f64 {
    match metric {
        Metric::TimeSpent => loss.time_spent_minutes,
        Metric::LifeShortened => loss.life_shortened_minutes,
        Metric::TotalTimeLost => loss.total_minutes(),
        Metric::Money => money,
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DailyTotal {
    pub date: Date,
    pub loss: TimeLoss,
    pub money: f64,
}

impl DailyTotal {
    fn empty(date: Date) -> Self {
        Self {
            date,
            loss: TimeLoss::zero(),
            money: 0.0,
        }
    }

    pub fn value(&self, metric: Metric) -> f64 {
        metric_value(self.loss, self.money, metric)
    }

    fn add_entry(&mut self, habit: &Habit, entry: &Entry) {
        self.loss = self.loss.add(time_loss_for_entry(habit, entry));
        self.money += money_spent_for_entry(habit, entry);
    }
}

fn daily_totals_map(
    entries: &[Entry],
    habits: &HashMap<HabitId, Habit>,
) -> HashMap<Date, DailyTotal> {
    let mut by_day: HashMap<Date, DailyTotal> = HashMap::new();
    for entry in entries {
        let Some(habit) = habits.get(&entry.habit_id) else {
            continue;
        };
        let date = entry.occurred_at.date();
        by_day
            .entry(date)
            .or_insert_with(|| DailyTotal::empty(date))
            .add_entry(habit, entry);
    }
    by_day
}

/// Every day in `[start, end]`, zero-filled where there's no activity. This is
/// the basis for bars, moving averages and the cumulative line: charts must
/// stay continuous across quiet days rather than skipping them. (The
/// cumulative sum, moving average and forward projection themselves are
/// computed client-side in TypeScript from this series — see
/// `src/lib/charts/series-math.ts` — so a metric toggle doesn't need a
/// round trip; this function is the one place both sides would otherwise
/// duplicate, so it stays the single source of truth.)
pub fn daily_series(
    entries: &[Entry],
    habits: &HashMap<HabitId, Habit>,
    start: Date,
    end: Date,
) -> Vec<DailyTotal> {
    let totals = daily_totals_map(entries, habits);
    let mut out = Vec::new();
    let mut d = start;
    loop {
        out.push(
            totals
                .get(&d)
                .copied()
                .unwrap_or_else(|| DailyTotal::empty(d)),
        );
        if d >= end {
            break;
        }
        d += Duration::days(1);
    }
    out
}

fn is_clean(d: &DailyTotal) -> bool {
    d.loss.total_minutes() == 0.0 && d.money == 0.0
}

pub fn current_streak_days(daily: &[DailyTotal]) -> i64 {
    daily.iter().rev().take_while(|d| is_clean(d)).count() as i64
}

pub fn longest_streak_days(daily: &[DailyTotal]) -> i64 {
    let mut longest = 0i64;
    let mut current = 0i64;
    for d in daily {
        if is_clean(d) {
            current += 1;
            longest = longest.max(current);
        } else {
            current = 0;
        }
    }
    longest
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PeriodCompare {
    pub current_total: f64,
    pub previous_total: f64,
    pub delta: f64,
    pub percent_change: Option<f64>,
}

pub fn period_compare(
    current: &[DailyTotal],
    previous: &[DailyTotal],
    metric: Metric,
) -> PeriodCompare {
    let current_total: f64 = current.iter().map(|d| d.value(metric)).sum();
    let previous_total: f64 = previous.iter().map(|d| d.value(metric)).sum();
    let delta = current_total - previous_total;
    let percent_change = if previous_total.abs() > f64::EPSILON {
        Some(delta / previous_total * 100.0)
    } else {
        None
    };
    PeriodCompare {
        current_total,
        previous_total,
        delta,
        percent_change,
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HabitTotal {
    pub habit_id: HabitId,
    pub loss: TimeLoss,
    pub money: f64,
}

impl HabitTotal {
    pub fn value(&self, metric: Metric) -> f64 {
        metric_value(self.loss, self.money, metric)
    }
}

pub fn totals_by_habit(
    entries: &[Entry],
    habits: &HashMap<HabitId, Habit>,
    sort_metric: Metric,
) -> Vec<HabitTotal> {
    let mut by_habit: HashMap<HabitId, HabitTotal> = HashMap::new();
    for entry in entries {
        let Some(habit) = habits.get(&entry.habit_id) else {
            continue;
        };
        let slot = by_habit.entry(entry.habit_id).or_insert(HabitTotal {
            habit_id: entry.habit_id,
            loss: TimeLoss::zero(),
            money: 0.0,
        });
        slot.loss = slot.loss.add(time_loss_for_entry(habit, entry));
        slot.money += money_spent_for_entry(habit, entry);
    }
    let mut out: Vec<HabitTotal> = by_habit.into_values().collect();
    out.sort_by(|a, b| {
        b.value(sort_metric)
            .partial_cmp(&a.value(sort_metric))
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    out
}

/// `[weekday][hour]`, Monday = 0, for the trigger-surfacing matrix chart.
pub fn hour_weekday_matrix(
    entries: &[Entry],
    habits: &HashMap<HabitId, Habit>,
    metric: Metric,
) -> [[f64; 24]; 7] {
    let mut matrix = [[0.0f64; 24]; 7];
    for entry in entries {
        let Some(habit) = habits.get(&entry.habit_id) else {
            continue;
        };
        let weekday_index = entry.occurred_at.weekday().number_days_from_monday() as usize;
        let hour = entry.occurred_at.hour() as usize;
        let loss = time_loss_for_entry(habit, entry);
        let money = money_spent_for_entry(habit, entry);
        matrix[weekday_index][hour] += metric_value(loss, money, metric);
    }
    matrix
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::habit::HabitId;
    use time::macros::{date, datetime};

    fn habits() -> HashMap<HabitId, Habit> {
        let mut h = Habit::new("Cigarettes", "cigarette");
        h.id = Some(HabitId(1));
        h.life_minutes_per_unit = 11.0;
        h.cost_per_unit = 0.6;
        HashMap::from([(HabitId(1), h)])
    }

    fn entry(day: time::Date, hour: u8, quantity: f64) -> Entry {
        let dt = time::PrimitiveDateTime::new(day, time::Time::from_hms(hour, 0, 0).unwrap());
        Entry::new(HabitId(1), dt, quantity)
    }

    #[test]
    fn daily_series_zero_fills_gaps() {
        let entries = vec![
            entry(date!(2026 - 01 - 01), 9, 2.0),
            entry(date!(2026 - 01 - 03), 9, 1.0),
        ];
        let series = daily_series(
            &entries,
            &habits(),
            date!(2026 - 01 - 01),
            date!(2026 - 01 - 03),
        );
        assert_eq!(series.len(), 3);
        assert_eq!(series[1].value(Metric::LifeShortened), 0.0);
        assert_eq!(series[0].value(Metric::LifeShortened), 22.0);
        assert_eq!(series[2].value(Metric::LifeShortened), 11.0);
    }

    #[test]
    fn streaks_count_clean_days() {
        let entries = vec![entry(date!(2026 - 01 - 01), 9, 1.0)];
        let series = daily_series(
            &entries,
            &habits(),
            date!(2026 - 01 - 01),
            date!(2026 - 01 - 05),
        );
        assert_eq!(current_streak_days(&series), 4);
        assert_eq!(longest_streak_days(&series), 4);
    }

    #[test]
    fn streak_breaks_on_activity() {
        let entries = vec![
            entry(date!(2026 - 01 - 01), 9, 1.0),
            entry(date!(2026 - 01 - 04), 9, 1.0),
        ];
        let series = daily_series(
            &entries,
            &habits(),
            date!(2026 - 01 - 01),
            date!(2026 - 01 - 05),
        );
        // day5 clean, day4 dirty -> current streak is 1 (just day5).
        assert_eq!(current_streak_days(&series), 1);
        assert_eq!(longest_streak_days(&series), 2);
    }

    #[test]
    fn period_compare_computes_signed_percent_delta() {
        let a = vec![entry(date!(2026 - 01 - 01), 9, 2.0)];
        let b = vec![entry(date!(2026 - 01 - 08), 9, 1.0)];
        let ha = habits();
        let current = daily_series(&a, &ha, date!(2026 - 01 - 01), date!(2026 - 01 - 01));
        let previous = daily_series(&b, &ha, date!(2026 - 01 - 08), date!(2026 - 01 - 08));
        let cmp = period_compare(&current, &previous, Metric::LifeShortened);
        assert_eq!(cmp.current_total, 22.0);
        assert_eq!(cmp.previous_total, 11.0);
        assert_eq!(cmp.delta, 11.0);
        assert_eq!(cmp.percent_change, Some(100.0));
    }

    #[test]
    fn period_compare_percent_none_when_previous_zero() {
        let a = vec![entry(date!(2026 - 01 - 01), 9, 2.0)];
        let ha = habits();
        let current = daily_series(&a, &ha, date!(2026 - 01 - 01), date!(2026 - 01 - 01));
        let previous = daily_series(&[], &ha, date!(2026 - 01 - 08), date!(2026 - 01 - 08));
        let cmp = period_compare(&current, &previous, Metric::LifeShortened);
        assert_eq!(cmp.percent_change, None);
    }

    #[test]
    fn totals_by_habit_sorts_descending_by_metric() {
        let mut low = Habit::new("Scrolling", "session");
        low.id = Some(HabitId(2));
        low.life_minutes_per_unit = 1.0;
        let mut hs = habits();
        hs.insert(HabitId(2), low);
        let entries = vec![
            entry(date!(2026 - 01 - 01), 9, 1.0),
            Entry::new(HabitId(2), datetime!(2026-01-01 10:00), 5.0),
        ];
        let totals = totals_by_habit(&entries, &hs, Metric::LifeShortened);
        assert_eq!(totals[0].habit_id, HabitId(1));
        assert_eq!(totals[1].habit_id, HabitId(2));
    }

    #[test]
    fn hour_weekday_matrix_buckets_by_local_time() {
        // 2026-01-05 is a Monday.
        let entries = vec![entry(date!(2026 - 01 - 05), 8, 2.0)];
        let matrix = hour_weekday_matrix(&entries, &habits(), Metric::LifeShortened);
        assert_eq!(matrix[0][8], 22.0);
        assert_eq!(matrix[0][9], 0.0);
        assert_eq!(matrix[1][8], 0.0);
    }
}
