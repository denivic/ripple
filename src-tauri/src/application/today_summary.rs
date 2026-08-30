use std::collections::HashMap;

use time::Date;

use super::day_range;
use crate::domain::cost_model::{
    money_spent_for_entry, opportunity_cost_for_entry, remaining_waking_life_months_at_rate,
    time_loss_for_entry, waking_life_share,
};
use crate::domain::habit::{Habit, HabitId};
use crate::domain::repository::{EntryRepository, HabitRepository, ProfileRepository, RepoResult};

pub struct TodaySummary {
    pub time_spent_minutes: f64,
    pub life_shortened_minutes: f64,
    pub money_spent: f64,
    pub opportunity_cost: Option<f64>,
    pub waking_life_share_today: f64,
    /// Today's life-shortened minutes annualized and projected against the
    /// profile's remaining waking years — the "at this rate, N months of
    /// your remaining waking life" framing. `None` without a complete profile.
    pub remaining_waking_life_months_at_todays_rate: Option<f64>,
}

const DAYS_PER_YEAR: f64 = 365.2425;

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
    let remaining_waking_life_months_at_todays_rate = remaining_waking_life_months_at_rate(
        life_shortened_minutes * DAYS_PER_YEAR,
        &profile,
        today,
    );

    Ok(TodaySummary {
        time_spent_minutes,
        life_shortened_minutes,
        money_spent,
        opportunity_cost: has_income.then_some(opportunity_cost),
        waking_life_share_today,
        remaining_waking_life_months_at_todays_rate,
    })
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

        assert_eq!(summary.remaining_waking_life_months_at_todays_rate, None);
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

        assert!(summary.remaining_waking_life_months_at_todays_rate.unwrap() > 0.0);
    }
}
