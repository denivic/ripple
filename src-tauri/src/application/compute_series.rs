use std::collections::HashMap;

use time::Duration;

use super::day_range;
use crate::domain::habit::{Habit, HabitId};
use crate::domain::projections::{
    current_streak_days, daily_series, hour_weekday_matrix, longest_streak_days, period_compare,
    totals_by_habit, Metric,
};
use crate::domain::repository::{EntryRepository, HabitRepository, RepoResult};
use time::Date;

pub struct DailyPoint {
    pub date: Date,
    pub time_spent_minutes: f64,
    pub life_shortened_minutes: f64,
    pub money: f64,
}

pub struct HabitBreakdownItem {
    pub habit_id: HabitId,
    pub time_spent_minutes: f64,
    pub life_shortened_minutes: f64,
    pub money: f64,
}

pub struct PeriodCompareResult {
    pub current_total_minutes: f64,
    pub previous_total_minutes: f64,
    pub delta_minutes: f64,
    pub percent_change: Option<f64>,
}

pub struct TimelineResult {
    pub daily: Vec<DailyPoint>,
    pub habit_breakdown: Vec<HabitBreakdownItem>,
    /// `[weekday 0=Mon..6=Sun][hour 0..23]`, total-time-lost minutes.
    pub hour_weekday_matrix: [[f64; 24]; 7],
    pub current_streak_days: i64,
    pub longest_streak_days: i64,
    pub period_compare: PeriodCompareResult,
}

/// Orchestrates the two repositories and the pure `domain::projections` math
/// the Timeline charts need, in one round trip. Archived habits are still
/// fetched — an entry logged against a since-archived habit must keep
/// contributing to its historical totals, not silently vanish.
pub fn compute_timeline(
    habit_repo: &dyn HabitRepository,
    entry_repo: &dyn EntryRepository,
    start: Date,
    end: Date,
) -> RepoResult<TimelineResult> {
    let habits: HashMap<HabitId, Habit> = habit_repo
        .list(true)?
        .into_iter()
        .filter_map(|h| h.id.map(|id| (id, h)))
        .collect();

    let (range_start, _) = day_range(start);
    let (_, range_end) = day_range(end);
    let entries = entry_repo.list_between(range_start, range_end)?;

    let daily = daily_series(&entries, &habits, start, end);
    let daily_points = daily
        .iter()
        .map(|d| DailyPoint {
            date: d.date,
            time_spent_minutes: d.loss.time_spent_minutes,
            life_shortened_minutes: d.loss.life_shortened_minutes,
            money: d.money,
        })
        .collect();

    let habit_breakdown = totals_by_habit(&entries, &habits, Metric::TotalTimeLost)
        .into_iter()
        .map(|h| HabitBreakdownItem {
            habit_id: h.habit_id,
            time_spent_minutes: h.loss.time_spent_minutes,
            life_shortened_minutes: h.loss.life_shortened_minutes,
            money: h.money,
        })
        .collect();

    let matrix = hour_weekday_matrix(&entries, &habits, Metric::TotalTimeLost);

    let period_days = (end - start).whole_days() + 1;
    let previous_end = start - Duration::days(1);
    let previous_start = previous_end - Duration::days(period_days - 1);
    let (prev_range_start, _) = day_range(previous_start);
    let (_, prev_range_end) = day_range(previous_end);
    let previous_entries = entry_repo.list_between(prev_range_start, prev_range_end)?;
    let previous_daily = daily_series(&previous_entries, &habits, previous_start, previous_end);

    let compare = period_compare(&daily, &previous_daily, Metric::TotalTimeLost);

    Ok(TimelineResult {
        daily: daily_points,
        habit_breakdown,
        hour_weekday_matrix: matrix,
        current_streak_days: current_streak_days(&daily),
        longest_streak_days: longest_streak_days(&daily),
        period_compare: PeriodCompareResult {
            current_total_minutes: compare.current_total,
            previous_total_minutes: compare.previous_total,
            delta_minutes: compare.delta,
            percent_change: compare.percent_change,
        },
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entry::Entry;
    use crate::infrastructure::db::{Db, SqliteEntryRepository, SqliteHabitRepository};
    use std::sync::Arc;
    use time::macros::{date, datetime};

    #[test]
    fn compute_timeline_includes_entries_from_archived_habits() {
        let db = Arc::new(Db::open_in_memory().unwrap());
        let habit_repo = SqliteHabitRepository::new(Arc::clone(&db));
        let entry_repo = SqliteEntryRepository::new(Arc::clone(&db));

        let mut habit = Habit::new("Cigarettes", "cigarette");
        habit.life_minutes_per_unit = 11.0;
        let habit_id = habit_repo.insert(&habit).unwrap();
        entry_repo
            .insert(&Entry::new(
                habit_id,
                datetime!(2026 - 03 - 10 08:00:00),
                2.0,
            ))
            .unwrap();
        habit_repo.archive(habit_id).unwrap();

        let result = compute_timeline(
            &habit_repo,
            &entry_repo,
            date!(2026 - 03 - 10),
            date!(2026 - 03 - 10),
        )
        .unwrap();

        assert_eq!(result.daily.len(), 1);
        assert_eq!(result.daily[0].life_shortened_minutes, 22.0);
        assert_eq!(result.habit_breakdown.len(), 1);
    }

    #[test]
    fn compute_timeline_zero_fills_and_compares_against_previous_equal_length_period() {
        let db = Arc::new(Db::open_in_memory().unwrap());
        let habit_repo = SqliteHabitRepository::new(Arc::clone(&db));
        let entry_repo = SqliteEntryRepository::new(Arc::clone(&db));

        let mut habit = Habit::new("Cigarettes", "cigarette");
        habit.life_minutes_per_unit = 10.0;
        let habit_id = habit_repo.insert(&habit).unwrap();
        // Current 2-day period: 2026-03-10..=2026-03-11
        entry_repo
            .insert(&Entry::new(
                habit_id,
                datetime!(2026 - 03 - 10 08:00:00),
                1.0,
            ))
            .unwrap();
        // Previous 2-day period: 2026-03-08..=2026-03-09
        entry_repo
            .insert(&Entry::new(
                habit_id,
                datetime!(2026 - 03 - 08 08:00:00),
                2.0,
            ))
            .unwrap();

        let result = compute_timeline(
            &habit_repo,
            &entry_repo,
            date!(2026 - 03 - 10),
            date!(2026 - 03 - 11),
        )
        .unwrap();

        assert_eq!(result.daily.len(), 2);
        assert_eq!(result.daily[1].life_shortened_minutes, 0.0); // zero-filled
        assert_eq!(result.period_compare.current_total_minutes, 10.0);
        assert_eq!(result.period_compare.previous_total_minutes, 20.0);
        assert_eq!(result.period_compare.delta_minutes, -10.0);
    }
}
