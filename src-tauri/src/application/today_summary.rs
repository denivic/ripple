use std::collections::HashMap;

use time::{Date, Duration};

use super::day_range;
use crate::domain::cost_model::{
    money_spent_for_entry, opportunity_cost_for_entry, projected_waking_months_lost,
    time_loss_for_entry, waking_life_share,
};
use crate::domain::habit::{Habit, HabitId};
use crate::domain::projections::daily_series;
use crate::domain::repository::{EntryRepository, HabitRepository, ProfileRepository, RepoResult};

pub struct TodaySummary {
    pub time_spent_minutes: f64,
    pub life_shortened_minutes: f64,
    pub money_spent: f64,
    pub opportunity_cost: Option<f64>,
    pub waking_life_share_today: f64,
    /// Months of waking life lost by the life-expectancy age if the average
    /// daily net life-minutes over the projection window held until then.
    /// Negative means months gained. `None` without a complete profile.
    pub projected_waking_months_lost: Option<f64>,
    /// How many days the projection's daily average is taken over.
    pub projection_window_days: u32,
    pub average_daily_life_minutes: f64,
    pub life_expectancy_age: Option<f64>,
}

/// The projection averages the last 30 days, today included...
const PROJECTION_WINDOW_DAYS: u32 = 30;
/// ...or the days since the first entry if that's fewer, but never under a
/// week, so a single entry on day one isn't treated as a daily habit.
const MIN_PROJECTION_WINDOW_DAYS: u32 = 7;

pub fn compute_today_summary(
    habit_repo: &dyn HabitRepository,
    entry_repo: &dyn EntryRepository,
    profile_repo: &dyn ProfileRepository,
    today: Date,
) -> RepoResult<TodaySummary> {
    let habits: HashMap<HabitId, Habit> = habit_repo
        .list(true)?
        .into_iter()
        .filter_map(|h| h.id.map(|id| (id, h)))
        .collect();
    let (start, end) = day_range(today);
    let entries = entry_repo.list_between(start, end)?;
    let profile = profile_repo.get()?;

    let mut time_spent_minutes = 0.0;
    let mut life_shortened_minutes = 0.0;
    let mut money_spent = 0.0;
    let mut opportunity_cost = 0.0;
    let mut has_income = false;

    for entry in &entries {
        let Some(habit) = habits.get(&entry.habit_id) else {
            continue;
        };
        let loss = time_loss_for_entry(habit, entry);
        time_spent_minutes += loss.time_spent_minutes;
        life_shortened_minutes += loss.life_shortened_minutes;
        money_spent += money_spent_for_entry(habit, entry);
        if let Some(cost) = opportunity_cost_for_entry(habit, entry, &profile) {
            opportunity_cost += cost;
            has_income = true;
        }
    }

    let waking_life_share_today =
        waking_life_share(time_spent_minutes + life_shortened_minutes, 1.0, &profile);
    let (window_days, average_daily_life_minutes) =
        average_daily_life_minutes(entry_repo, &habits, today)?;
    let projected_waking_months_lost =
        projected_waking_months_lost(average_daily_life_minutes, &profile, today);

    Ok(TodaySummary {
        time_spent_minutes,
        life_shortened_minutes,
        money_spent,
        opportunity_cost: has_income.then_some(opportunity_cost),
        waking_life_share_today,
        projected_waking_months_lost,
        projection_window_days: window_days,
        average_daily_life_minutes,
        life_expectancy_age: profile.life_expectancy_years,
    })
}

/// Net life-minutes per day (losses minus gains) averaged over the
/// projection window, alongside the window's length in days.
fn average_daily_life_minutes(
    entry_repo: &dyn EntryRepository,
    habits: &HashMap<HabitId, Habit>,
    today: Date,
) -> RepoResult<(u32, f64)> {
    let full_start = today - Duration::days(i64::from(PROJECTION_WINDOW_DAYS) - 1);
    let (range_start, _) = day_range(full_start);
    let (_, range_end) = day_range(today);
    let entries = entry_repo.list_between(range_start, range_end)?;

    // Nothing logged before the window's first day can shorten it, so the
    // full history is only fetched when the window's own entries start late.
    let earliest_in_window = entries.iter().map(|e| e.occurred_at.date()).min();
    let window_days = match earliest_in_window {
        None => PROJECTION_WINDOW_DAYS,
        Some(d) if d <= full_start => PROJECTION_WINDOW_DAYS,
        Some(_) => {
            let first = entry_repo
                .list_all()?
                .iter()
                .map(|e| e.occurred_at.date())
                .min()
                .unwrap_or(today);
            let days_since_first = (today - first).whole_days() + 1;
            days_since_first.clamp(
                i64::from(MIN_PROJECTION_WINDOW_DAYS),
                i64::from(PROJECTION_WINDOW_DAYS),
            ) as u32
        }
    };

    let total: f64 = daily_series(&entries, habits, full_start, today)
        .iter()
        .map(|d| d.loss.life_shortened_minutes)
        .sum();
    Ok((window_days, total / f64::from(window_days)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entry::Entry;
    use crate::domain::habit::Habit;
    use crate::domain::profile::Profile;
    use crate::infrastructure::db::{
        Db, SqliteEntryRepository, SqliteHabitRepository, SqliteProfileRepository,
    };
    use std::sync::Arc;
    use time::macros::{date, datetime};
    use time::{PrimitiveDateTime, Time};

    #[test]
    fn today_summary_sums_only_todays_entries() {
        let db = Arc::new(Db::open_in_memory().unwrap());
        let habit_repo = SqliteHabitRepository::new(Arc::clone(&db));
        let entry_repo = SqliteEntryRepository::new(Arc::clone(&db));
        let profile_repo = SqliteProfileRepository::new(Arc::clone(&db));

        let mut habit = Habit::new("Cigarettes", "cigarette");
        habit.life_minutes_per_unit = 11.0;
        habit.cost_per_unit = 0.6;
        let habit_id = habit_repo.insert(&habit).unwrap();
        entry_repo
            .insert(&Entry::new(
                habit_id,
                datetime!(2026 - 03 - 10 08:00:00),
                2.0,
            ))
            .unwrap();
        entry_repo
            .insert(&Entry::new(
                habit_id,
                datetime!(2026 - 03 - 09 08:00:00),
                100.0,
            ))
            .unwrap();

        let summary = compute_today_summary(
            &habit_repo,
            &entry_repo,
            &profile_repo,
            date!(2026 - 03 - 10),
        )
        .unwrap();

        assert_eq!(summary.life_shortened_minutes, 22.0);
        assert!((summary.money_spent - 1.20).abs() < 1e-9);
    }

    #[test]
    fn today_summary_omits_remaining_life_projection_without_a_complete_profile() {
        let db = Arc::new(Db::open_in_memory().unwrap());
        let habit_repo = SqliteHabitRepository::new(Arc::clone(&db));
        let entry_repo = SqliteEntryRepository::new(Arc::clone(&db));
        let profile_repo = SqliteProfileRepository::new(Arc::clone(&db));

        let summary = compute_today_summary(
            &habit_repo,
            &entry_repo,
            &profile_repo,
            date!(2026 - 03 - 10),
        )
        .unwrap();

        assert_eq!(summary.projected_waking_months_lost, None);
        assert_eq!(summary.opportunity_cost, None);
    }

    #[test]
    fn today_summary_projects_remaining_life_with_a_complete_profile() {
        let db = Arc::new(Db::open_in_memory().unwrap());
        let habit_repo = SqliteHabitRepository::new(Arc::clone(&db));
        let entry_repo = SqliteEntryRepository::new(Arc::clone(&db));
        let profile_repo = SqliteProfileRepository::new(Arc::clone(&db));

        profile_repo
            .save(&Profile {
                birth_date: Some(date!(1994 - 01 - 01)),
                life_expectancy_years: Some(80.0),
                typical_sleep_hours: Some(8.0),
                ..Default::default()
            })
            .unwrap();

        let mut habit = Habit::new("Cigarettes", "cigarette");
        habit.life_minutes_per_unit = 11.0;
        let habit_id = habit_repo.insert(&habit).unwrap();
        entry_repo
            .insert(&Entry::new(
                habit_id,
                datetime!(2026 - 03 - 10 08:00:00),
                4.0,
            ))
            .unwrap();

        let summary = compute_today_summary(
            &habit_repo,
            &entry_repo,
            &profile_repo,
            date!(2026 - 03 - 10),
        )
        .unwrap();

        assert!(summary.projected_waking_months_lost.unwrap() > 0.0);
    }

    struct Fixture {
        habit_repo: SqliteHabitRepository,
        entry_repo: SqliteEntryRepository,
        profile_repo: SqliteProfileRepository,
    }

    impl Fixture {
        /// Aged exactly 28 on 2026-03-10, life expectancy 80, 8 h of sleep.
        fn aged_28() -> Self {
            let db = Arc::new(Db::open_in_memory().unwrap());
            let fixture = Self {
                habit_repo: SqliteHabitRepository::new(Arc::clone(&db)),
                entry_repo: SqliteEntryRepository::new(Arc::clone(&db)),
                profile_repo: SqliteProfileRepository::new(Arc::clone(&db)),
            };
            fixture
                .profile_repo
                .save(&Profile {
                    birth_date: Some(date!(1998 - 03 - 10)),
                    life_expectancy_years: Some(80.0),
                    typical_sleep_hours: Some(8.0),
                    ..Default::default()
                })
                .unwrap();
            fixture
        }

        fn habit(&self, life_minutes_per_unit: f64) -> HabitId {
            let mut habit = Habit::new("Joint", "joint");
            habit.life_minutes_per_unit = life_minutes_per_unit;
            self.habit_repo.insert(&habit).unwrap()
        }

        fn log(&self, habit_id: HabitId, day: Date, quantity: f64) {
            let at = PrimitiveDateTime::new(day, Time::from_hms(20, 0, 0).unwrap());
            self.entry_repo
                .insert(&Entry::new(habit_id, at, quantity))
                .unwrap();
        }

        fn summary(&self, today: Date) -> TodaySummary {
            compute_today_summary(
                &self.habit_repo,
                &self.entry_repo,
                &self.profile_repo,
                today,
            )
            .unwrap()
        }
    }

    const TODAY: Date = date!(2026 - 03 - 10);

    #[test]
    fn a_single_first_entry_is_averaged_over_the_seven_day_floor() {
        let f = Fixture::aged_28();
        let joint = f.habit(55.0);
        f.log(joint, TODAY, 1.0);

        let summary = f.summary(TODAY);

        assert_eq!(summary.projection_window_days, 7);
        assert!((summary.average_daily_life_minutes - 55.0 / 7.0).abs() < 1e-9);
        assert_eq!(summary.life_expectancy_age, Some(80.0));
        // 55/7 min a day of 960 waking minutes, over ~52 years (624 months).
        let months = summary.projected_waking_months_lost.unwrap();
        assert!((months - 5.107).abs() < 0.001, "{months}");
    }

    #[test]
    fn a_short_history_is_averaged_over_the_days_since_the_first_entry() {
        let f = Fixture::aged_28();
        let joint = f.habit(55.0);
        f.log(joint, TODAY - Duration::days(9), 1.0);
        f.log(joint, TODAY, 1.0);

        let summary = f.summary(TODAY);

        assert_eq!(summary.projection_window_days, 10);
        assert!((summary.average_daily_life_minutes - 11.0).abs() < 1e-9);
    }

    #[test]
    fn a_full_thirty_days_of_history_averages_over_thirty_days() {
        let f = Fixture::aged_28();
        let joint = f.habit(55.0);
        for i in 0..30 {
            f.log(joint, TODAY - Duration::days(i), 1.0);
        }

        let summary = f.summary(TODAY);

        assert_eq!(summary.projection_window_days, 30);
        assert!((summary.average_daily_life_minutes - 55.0).abs() < 1e-9);
        // One joint every day: 55/960 of waking time over ~624 months.
        let months = summary.projected_waking_months_lost.unwrap();
        assert!((months - 35.75).abs() < 0.01, "{months}");
    }

    #[test]
    fn entries_older_than_thirty_days_are_excluded_from_the_average() {
        let f = Fixture::aged_28();
        let joint = f.habit(55.0);
        f.log(joint, TODAY - Duration::days(30), 100.0);
        f.log(joint, TODAY, 1.0);

        let summary = f.summary(TODAY);

        assert_eq!(summary.projection_window_days, 30);
        assert!((summary.average_daily_life_minutes - 55.0 / 30.0).abs() < 1e-9);
    }

    #[test]
    fn an_empty_window_projects_zero_months() {
        let f = Fixture::aged_28();

        let summary = f.summary(TODAY);

        assert_eq!(summary.projection_window_days, 30);
        assert_eq!(summary.average_daily_life_minutes, 0.0);
        assert_eq!(summary.projected_waking_months_lost, Some(0.0));
    }

    #[test]
    fn a_net_gain_projects_negative_months_lost() {
        let f = Fixture::aged_28();
        let joint = f.habit(55.0);
        let walk = f.habit(-120.0);
        for i in 0..30 {
            f.log(joint, TODAY - Duration::days(i), 1.0);
            f.log(walk, TODAY - Duration::days(i), 1.0);
        }

        let summary = f.summary(TODAY);

        assert!((summary.average_daily_life_minutes + 65.0).abs() < 1e-9);
        assert!(summary.projected_waking_months_lost.unwrap() < 0.0);
    }
}
