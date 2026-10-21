use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

use time::{Date, Duration};

use crate::domain::entry::Entry;
use crate::domain::habit::{Habit, HabitId};
use crate::domain::levels::{level_for, LevelProgress};
use crate::domain::points::{
    milestone_award, milestone_award_key, AwardKind, Milestone, PointAward, MILESTONES,
    MILESTONE_FIRST_ENTRY, MILESTONE_FIRST_HALVED_HABIT, MILESTONE_FIRST_IMPROVED_WEEK,
    MILESTONE_POINTS_1000, MILESTONE_STREAK_100, MILESTONE_STREAK_30, MILESTONE_STREAK_7,
};
use crate::domain::projections::{daily_series, is_clean_day};
use crate::domain::repository::{
    EntryRepository, HabitRepository, PointAwardRepository, RepoResult, SettingsRepository,
};
use crate::domain::settings::WeekStart;

const LOGGED_DAY_POINTS: i64 = 5;
const CLEAN_DAY_POINTS: i64 = 10;
const IMPROVED_WEEK_POINTS: i64 = 25;
const STREAK_BONUS_POINTS: i64 = 50;
const STREAK_BONUS_EVERY: i64 = 7;
const POINTS_BADGE_THRESHOLD: i64 = 1000;
const RECENT_AWARDS: usize = 20;

const STREAK_MILESTONES: [(i64, &str); 3] = [
    (7, MILESTONE_STREAK_7),
    (30, MILESTONE_STREAK_30),
    (100, MILESTONE_STREAK_100),
];

fn day_key(prefix: &str, day: Date) -> String {
    format!("{prefix}:{day}")
}

fn week_start_of(day: Date, week_start: WeekStart) -> Date {
    let offset = match week_start {
        WeekStart::Monday => day.weekday().number_days_from_monday(),
        WeekStart::Sunday => day.weekday().number_days_from_sunday(),
    };
    day - Duration::days(offset as i64)
}

/// Every award the history in `entries` has earned as of `today`, whether or
/// not it's already in the ledger — the repository's INSERT OR IGNORE on
/// `award_key` is what makes re-running this harmless. Entries dated after
/// `today` are ignored; "completed" days and weeks are those wholly before it.
/// Sorted by the day each award is for, so a backfill lands in the ledger in
/// chronological order.
pub fn candidate_awards(
    entries: &[Entry],
    habits: &HashMap<HabitId, Habit>,
    week_start: WeekStart,
    today: Date,
) -> Vec<PointAward> {
    let entries: Vec<Entry> = entries
        .iter()
        .filter(|e| e.occurred_at.date() <= today)
        .cloned()
        .collect();
    let Some(first_day) = entries.iter().map(|e| e.occurred_at.date()).min() else {
        return Vec::new();
    };

    let mut out = vec![milestone_award(MILESTONE_FIRST_ENTRY, first_day)];
    logged_day_awards(&entries, &mut out);
    clean_day_awards(&entries, habits, first_day, today, &mut out);
    improved_week_awards(&entries, habits, week_start, today, &mut out);

    out.sort_by(|a, b| {
        a.awarded_for
            .cmp(&b.awarded_for)
            .then_with(|| a.award_key.cmp(&b.award_key))
    });
    out
}

/// +5 per day with at least one entry, today included.
fn logged_day_awards(entries: &[Entry], out: &mut Vec<PointAward>) {
    let days: BTreeSet<Date> = entries.iter().map(|e| e.occurred_at.date()).collect();
    out.extend(days.into_iter().map(|day| {
        PointAward::candidate(
            day_key("logged", day),
            AwardKind::LoggedDay,
            LOGGED_DAY_POINTS,
            day,
            None,
        )
    }));
}

/// +10 per completed clean day from the first entry on, +50 on every 7th
/// consecutive one, and the streak milestones the first time a run reaches
/// 7, 30 or 100 days.
fn clean_day_awards(
    entries: &[Entry],
    habits: &HashMap<HabitId, Habit>,
    first_day: Date,
    today: Date,
    out: &mut Vec<PointAward>,
) {
    if first_day >= today {
        return;
    }
    let yesterday = today - Duration::days(1);
    let mut streak = 0i64;
    let mut reached: HashSet<&str> = HashSet::new();
    for day in daily_series(entries, habits, first_day, yesterday) {
        if !is_clean_day(&day) {
            streak = 0;
            continue;
        }
        streak += 1;
        out.push(PointAward::candidate(
            day_key("clean", day.date),
            AwardKind::CleanDay,
            CLEAN_DAY_POINTS,
            day.date,
            None,
        ));
        if streak % STREAK_BONUS_EVERY == 0 {
            out.push(PointAward::candidate(
                day_key("streak7", day.date),
                AwardKind::StreakBonus,
                STREAK_BONUS_POINTS,
                day.date,
                Some(format!("{streak}-day clean streak")),
            ));
        }
        for (length, key) in STREAK_MILESTONES {
            if streak == length && reached.insert(key) {
                out.push(milestone_award(key, day.date));
            }
        }
    }
}

/// +25 per habit per completed week that improved on the week before it:
/// less of a harmful habit, more of a good one. Neutral habits (life minutes
/// per unit of 0) don't count either way, and a week only counts when the
/// one before it has entries to compare against.
fn improved_week_awards(
    entries: &[Entry],
    habits: &HashMap<HabitId, Habit>,
    week_start: WeekStart,
    today: Date,
    out: &mut Vec<PointAward>,
) {
    // habit -> week start -> total quantity. A week present in the map had
    // at least one entry, which is what "had data" means.
    let mut weekly: HashMap<HabitId, BTreeMap<Date, f64>> = HashMap::new();
    for entry in entries {
        let week = week_start_of(entry.occurred_at.date(), week_start);
        *weekly
            .entry(entry.habit_id)
            .or_default()
            .entry(week)
            .or_insert(0.0) += entry.quantity;
    }

    let mut first_improved: Option<Date> = None;
    let mut first_halved: Option<Date> = None;
    for (habit_id, weeks) in &weekly {
        let Some(habit) = habits.get(habit_id) else {
            continue;
        };
        let harmful = habit.life_minutes_per_unit > 0.0;
        if !harmful && habit.life_minutes_per_unit >= 0.0 {
            continue;
        }
        for (&previous_week, &previous) in weeks {
            let week = previous_week + Duration::days(7);
            let week_end = week + Duration::days(6);
            if week_end >= today {
                break;
            }
            let current = weeks.get(&week).copied().unwrap_or(0.0);
            let improved = if harmful {
                current < previous
            } else {
                current > previous
            };
            if !improved {
                continue;
            }
            out.push(PointAward::candidate(
                format!("habit-improved:{}:{week}", habit_id.0),
                AwardKind::HabitImproved,
                IMPROVED_WEEK_POINTS,
                week_end,
                Some(habit.name.clone()),
            ));
            first_improved = Some(first_improved.map_or(week_end, |d| d.min(week_end)));
            if harmful && current <= previous * 0.5 {
                first_halved = Some(first_halved.map_or(week_end, |d| d.min(week_end)));
            }
        }
    }
    if let Some(day) = first_improved {
        out.push(milestone_award(MILESTONE_FIRST_IMPROVED_WEEK, day));
    }
    if let Some(day) = first_halved {
        out.push(milestone_award(MILESTONE_FIRST_HALVED_HABIT, day));
    }
}

/// Records whatever the whole history has earned that isn't in the ledger
/// yet, and returns how many awards were new. Safe to call as often as
/// wanted: the first call backfills everything, later ones add only what's
/// changed, and nothing already awarded is ever taken back.
pub fn evaluate_awards(
    habit_repo: &dyn HabitRepository,
    entry_repo: &dyn EntryRepository,
    settings_repo: &dyn SettingsRepository,
    award_repo: &dyn PointAwardRepository,
    today: Date,
) -> RepoResult<usize> {
    let habits: HashMap<HabitId, Habit> = habit_repo
        .list(true)?
        .into_iter()
        .filter_map(|h| h.id.map(|id| (id, h)))
        .collect();
    let entries = entry_repo.list_all()?;
    let week_start = settings_repo.get()?.week_start;

    let candidates = candidate_awards(&entries, &habits, week_start, today);
    let mut inserted = award_repo.insert_new(&candidates)?;
    // Depends on the ledger total rather than the history, so it's checked
    // after the history's awards are in.
    if award_repo.total_points()? >= POINTS_BADGE_THRESHOLD {
        inserted += award_repo.insert_new(&[milestone_award(MILESTONE_POINTS_1000, today)])?;
    }
    Ok(inserted)
}

pub struct MilestoneStatus {
    pub milestone: Milestone,
    pub achieved_on: Option<Date>,
}

pub struct Progress {
    pub total_points: i64,
    pub level: LevelProgress,
    pub recent_awards: Vec<PointAward>,
    /// Every milestone, achieved or not, in definition order.
    pub milestones: Vec<MilestoneStatus>,
    /// How many awards this call added, so the caller knows whether to
    /// announce a change.
    pub newly_awarded: usize,
}

/// Evaluates first, so the progress returned always reflects the latest
/// history — this is the hook the frontend calls on startup and after every
/// entries/habits change.
pub fn get_progress(
    habit_repo: &dyn HabitRepository,
    entry_repo: &dyn EntryRepository,
    settings_repo: &dyn SettingsRepository,
    award_repo: &dyn PointAwardRepository,
    today: Date,
) -> RepoResult<Progress> {
    let newly_awarded = evaluate_awards(habit_repo, entry_repo, settings_repo, award_repo, today)?;
    let total_points = award_repo.total_points()?;
    let achieved: HashMap<String, Date> = award_repo
        .list_by_kind(AwardKind::Milestone)?
        .into_iter()
        .map(|a| (a.award_key, a.awarded_for))
        .collect();
    let milestones = MILESTONES
        .iter()
        .map(|m| MilestoneStatus {
            milestone: *m,
            achieved_on: achieved.get(&milestone_award_key(m.key)).copied(),
        })
        .collect();
    Ok(Progress {
        total_points,
        level: level_for(total_points),
        recent_awards: award_repo.recent(RECENT_AWARDS)?,
        milestones,
        newly_awarded,
    })
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use time::macros::date;
    use time::{PrimitiveDateTime, Time};

    use super::*;
    use crate::domain::repository::{EntryRepository, HabitRepository, SettingsRepository};
    use crate::domain::Settings;
    use crate::infrastructure::db::{
        Db, SqliteEntryRepository, SqliteHabitRepository, SqlitePointAwardRepository,
        SqliteSettingsRepository,
    };

    // 2026-10-21 is a Wednesday.
    const TODAY: Date = date!(2026 - 10 - 21);
    const HARMFUL: HabitId = HabitId(1);
    const GOOD: HabitId = HabitId(2);

    fn habits() -> HashMap<HabitId, Habit> {
        let mut harmful = Habit::new("Cigarettes", "cigarette");
        harmful.id = Some(HARMFUL);
        harmful.life_minutes_per_unit = 11.0;
        let mut good = Habit::new("Running", "km");
        good.id = Some(GOOD);
        good.life_minutes_per_unit = -5.0;
        HashMap::from([(HARMFUL, harmful), (GOOD, good)])
    }

    fn entry(habit: HabitId, day: Date, quantity: f64) -> Entry {
        Entry::new(
            habit,
            PrimitiveDateTime::new(day, Time::from_hms(12, 0, 0).unwrap()),
            quantity,
        )
    }

    fn keys(awards: &[PointAward], kind: AwardKind) -> Vec<String> {
        awards
            .iter()
            .filter(|a| a.kind == kind)
            .map(|a| a.award_key.clone())
            .collect()
    }

    fn milestone_date(awards: &[PointAward], key: &str) -> Option<Date> {
        awards
            .iter()
            .find(|a| a.award_key == milestone_award_key(key))
            .map(|a| a.awarded_for)
    }

    #[test]
    fn each_logged_day_earns_five_including_today() {
        let entries = vec![
            entry(HARMFUL, date!(2026 - 10 - 19), 1.0),
            entry(GOOD, date!(2026 - 10 - 19), 1.0),
            entry(GOOD, TODAY, 1.0),
            entry(GOOD, date!(2026 - 10 - 22), 1.0),
        ];
        let awards = candidate_awards(&entries, &habits(), WeekStart::Monday, TODAY);
        assert_eq!(
            keys(&awards, AwardKind::LoggedDay),
            ["logged:2026-10-19", "logged:2026-10-21"]
        );
        assert!(awards
            .iter()
            .filter(|a| a.kind == AwardKind::LoggedDay)
            .all(|a| a.points == 5));
    }

    #[test]
    fn completed_days_without_harmful_entries_are_clean() {
        let entries = vec![
            entry(HARMFUL, date!(2026 - 10 - 18), 1.0),
            // Only a good habit: still clean.
            entry(GOOD, date!(2026 - 10 - 19), 3.0),
        ];
        let awards = candidate_awards(&entries, &habits(), WeekStart::Monday, TODAY);
        // 10-20 is clean with no entries; today isn't completed yet.
        assert_eq!(
            keys(&awards, AwardKind::CleanDay),
            ["clean:2026-10-19", "clean:2026-10-20"]
        );
    }

    #[test]
    fn streak_bonus_lands_on_day_7_and_14() {
        let start = date!(2026 - 09 - 01);
        let entries = vec![entry(HARMFUL, start, 1.0)];
        // Clean from 09-02 through 09-16: a 15-day streak.
        let awards = candidate_awards(
            &entries,
            &habits(),
            WeekStart::Monday,
            date!(2026 - 09 - 17),
        );
        assert_eq!(
            keys(&awards, AwardKind::StreakBonus),
            ["streak7:2026-09-08", "streak7:2026-09-15"]
        );
        assert_eq!(keys(&awards, AwardKind::CleanDay).len(), 15);
        assert_eq!(
            milestone_date(&awards, MILESTONE_STREAK_7),
            Some(date!(2026 - 09 - 08))
        );
        assert_eq!(milestone_date(&awards, MILESTONE_STREAK_30), None);
    }

    #[test]
    fn a_broken_streak_restarts_the_bonus_count() {
        let entries = vec![
            entry(HARMFUL, date!(2026 - 09 - 01), 1.0),
            entry(HARMFUL, date!(2026 - 09 - 05), 1.0),
        ];
        // 3 clean, break, then 11 clean (09-06..09-16): one bonus, on 09-12.
        let awards = candidate_awards(
            &entries,
            &habits(),
            WeekStart::Monday,
            date!(2026 - 09 - 17),
        );
        assert_eq!(
            keys(&awards, AwardKind::StreakBonus),
            ["streak7:2026-09-12"]
        );
    }

    #[test]
    fn harmful_habit_improves_by_going_down_and_halving_unlocks_a_milestone() {
        let entries = vec![
            entry(HARMFUL, date!(2026 - 10 - 06), 4.0),
            entry(HARMFUL, date!(2026 - 10 - 13), 2.0),
        ];
        let awards = candidate_awards(&entries, &habits(), WeekStart::Monday, TODAY);
        // Week of 10-12 is complete; the current week (10-19) isn't judged.
        assert_eq!(
            keys(&awards, AwardKind::HabitImproved),
            ["habit-improved:1:2026-10-12"]
        );
        let improved = awards
            .iter()
            .find(|a| a.kind == AwardKind::HabitImproved)
            .unwrap();
        assert_eq!(improved.points, 25);
        assert_eq!(improved.awarded_for, date!(2026 - 10 - 18));
        assert_eq!(
            milestone_date(&awards, MILESTONE_FIRST_IMPROVED_WEEK),
            Some(date!(2026 - 10 - 18))
        );
        assert_eq!(
            milestone_date(&awards, MILESTONE_FIRST_HALVED_HABIT),
            Some(date!(2026 - 10 - 18))
        );
    }

    #[test]
    fn good_habit_improves_by_going_up() {
        let entries = vec![
            entry(GOOD, date!(2026 - 10 - 06), 1.0),
            entry(GOOD, date!(2026 - 10 - 13), 3.0),
            // Harmful habit held steady: no award.
            entry(HARMFUL, date!(2026 - 10 - 06), 2.0),
            entry(HARMFUL, date!(2026 - 10 - 13), 2.0),
        ];
        let awards = candidate_awards(&entries, &habits(), WeekStart::Monday, TODAY);
        assert_eq!(
            keys(&awards, AwardKind::HabitImproved),
            ["habit-improved:2:2026-10-12"]
        );
        // A good habit going up isn't a harmful one being cut.
        assert_eq!(milestone_date(&awards, MILESTONE_FIRST_HALVED_HABIT), None);
    }

    #[test]
    fn no_improvement_without_a_previous_week_of_data() {
        let entries = vec![entry(GOOD, date!(2026 - 10 - 13), 3.0)];
        let awards = candidate_awards(&entries, &habits(), WeekStart::Monday, TODAY);
        assert!(keys(&awards, AwardKind::HabitImproved).is_empty());
        assert_eq!(milestone_date(&awards, MILESTONE_FIRST_IMPROVED_WEEK), None);
    }

    #[test]
    fn week_start_decides_which_week_a_sunday_belongs_to() {
        let entries = vec![
            entry(HARMFUL, date!(2026 - 10 - 06), 3.0),
            entry(HARMFUL, date!(2026 - 10 - 11), 3.0), // a Sunday
            entry(HARMFUL, date!(2026 - 10 - 13), 4.0),
        ];
        // Monday weeks: 6 then 4, improved.
        let monday = candidate_awards(&entries, &habits(), WeekStart::Monday, TODAY);
        assert_eq!(
            keys(&monday, AwardKind::HabitImproved),
            ["habit-improved:1:2026-10-12"]
        );
        // Sunday weeks: 3 then 7, worse.
        let sunday = candidate_awards(&entries, &habits(), WeekStart::Sunday, TODAY);
        assert!(keys(&sunday, AwardKind::HabitImproved).is_empty());
    }

    #[test]
    fn first_entry_milestone_is_dated_to_the_first_entry() {
        let entries = vec![
            entry(GOOD, date!(2026 - 10 - 20), 1.0),
            entry(GOOD, date!(2026 - 10 - 10), 1.0),
        ];
        let awards = candidate_awards(&entries, &habits(), WeekStart::Monday, TODAY);
        assert_eq!(
            milestone_date(&awards, MILESTONE_FIRST_ENTRY),
            Some(date!(2026 - 10 - 10))
        );
        assert!(candidate_awards(&[], &habits(), WeekStart::Monday, TODAY).is_empty());
    }

    struct Fixture {
        habits: SqliteHabitRepository,
        entries: SqliteEntryRepository,
        settings: SqliteSettingsRepository,
        awards: SqlitePointAwardRepository,
    }

    impl Fixture {
        fn new() -> Self {
            let db = Arc::new(Db::open_in_memory().unwrap());
            Self {
                habits: SqliteHabitRepository::new(Arc::clone(&db)),
                entries: SqliteEntryRepository::new(Arc::clone(&db)),
                settings: SqliteSettingsRepository::new(Arc::clone(&db)),
                awards: SqlitePointAwardRepository::new(db),
            }
        }

        fn progress(&self, today: Date) -> Progress {
            get_progress(
                &self.habits,
                &self.entries,
                &self.settings,
                &self.awards,
                today,
            )
            .unwrap()
        }
    }

    #[test]
    fn evaluating_twice_awards_nothing_new() {
        let f = Fixture::new();
        let mut habit = Habit::new("Cigarettes", "cigarette");
        habit.life_minutes_per_unit = 11.0;
        let id = f.habits.insert(&habit).unwrap();
        f.entries
            .insert(&entry(id, date!(2026 - 10 - 06), 4.0))
            .unwrap();
        f.entries
            .insert(&entry(id, date!(2026 - 10 - 13), 2.0))
            .unwrap();

        let first = f.progress(TODAY);
        let second = f.progress(TODAY);

        assert!(first.newly_awarded > 0);
        assert_eq!(second.newly_awarded, 0);
        assert_eq!(first.total_points, second.total_points);
        assert_eq!(first.level.name, second.level.name);
    }

    #[test]
    fn later_edits_never_take_points_back() {
        let f = Fixture::new();
        let mut habit = Habit::new("Running", "km");
        habit.life_minutes_per_unit = -5.0;
        let id = f.habits.insert(&habit).unwrap();
        let entry_id = f
            .entries
            .insert(&entry(id, date!(2026 - 10 - 19), 1.0))
            .unwrap();
        let before = f.progress(TODAY).total_points;

        f.entries.delete(entry_id).unwrap();

        assert_eq!(f.progress(TODAY).total_points, before);
    }

    #[test]
    fn milestones_list_achieved_and_locked_and_respect_week_start_setting() {
        let f = Fixture::new();
        f.settings
            .save(&Settings {
                week_start: WeekStart::Sunday,
                ..Settings::default()
            })
            .unwrap();
        let mut habit = Habit::new("Cigarettes", "cigarette");
        habit.life_minutes_per_unit = 11.0;
        let id = f.habits.insert(&habit).unwrap();
        for (day, qty) in [
            (date!(2026 - 10 - 06), 3.0),
            (date!(2026 - 10 - 11), 3.0),
            (date!(2026 - 10 - 13), 4.0),
        ] {
            f.entries.insert(&entry(id, day, qty)).unwrap();
        }

        let progress = f.progress(TODAY);

        assert_eq!(progress.milestones.len(), MILESTONES.len());
        let status = |key: &str| {
            progress
                .milestones
                .iter()
                .find(|m| m.milestone.key == key)
                .unwrap()
                .achieved_on
        };
        assert_eq!(status(MILESTONE_FIRST_ENTRY), Some(date!(2026 - 10 - 06)));
        // With Sunday weeks this history got worse, so still locked.
        assert_eq!(status(MILESTONE_FIRST_IMPROVED_WEEK), None);
        assert_eq!(status(MILESTONE_POINTS_1000), None);
    }

    #[test]
    fn reaching_a_thousand_points_unlocks_the_badge() {
        let f = Fixture::new();
        let mut habit = Habit::new("Running", "km");
        habit.life_minutes_per_unit = -5.0;
        let id = f.habits.insert(&habit).unwrap();
        f.entries.insert(&entry(id, TODAY, 1.0)).unwrap();
        // 10 (first entry) + 5 (logged today) + 985 = 1000.
        f.awards
            .insert_new(&[PointAward::candidate(
                "test:seed".to_string(),
                AwardKind::CleanDay,
                985,
                TODAY,
                None,
            )])
            .unwrap();

        let progress = f.progress(TODAY);

        assert_eq!(progress.total_points, 1000);
        assert_eq!(progress.level.name, "Current");
        let badge = progress
            .milestones
            .iter()
            .find(|m| m.milestone.key == MILESTONE_POINTS_1000)
            .unwrap();
        assert_eq!(badge.achieved_on, Some(TODAY));
    }
}
