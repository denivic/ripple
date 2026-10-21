use std::collections::HashMap;

use time::{Date, Duration, PrimitiveDateTime, Time, Weekday};

use super::entry::Entry;
use super::habit::{Habit, HabitId};

/// The four notification categories. Their ids are stable: they're the
/// primary key of `notification_prefs` and the prefix of every dedup key.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Category {
    Reminders,
    Progress,
    WeeklySummary,
    StreakPace,
}

impl Category {
    pub const ALL: [Category; 4] = [
        Category::Reminders,
        Category::Progress,
        Category::WeeklySummary,
        Category::StreakPace,
    ];

    pub fn id(self) -> &'static str {
        match self {
            Category::Reminders => "reminders",
            Category::Progress => "progress",
            Category::WeeklySummary => "weekly_summary",
            Category::StreakPace => "streak_pace",
        }
    }

    pub fn from_id(s: &str) -> Option<Self> {
        Category::ALL.into_iter().find(|c| c.id() == s)
    }
}

/// Per-category switch and channel choice. Default: on, local only.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CategoryPrefs {
    pub enabled: bool,
    pub local: bool,
    pub email: bool,
}

impl Default for CategoryPrefs {
    fn default() -> Self {
        Self {
            enabled: true,
            local: true,
            email: false,
        }
    }
}

impl CategoryPrefs {
    /// The channels this category would actually go out on.
    pub fn channels(self) -> Vec<Channel> {
        if !self.enabled {
            return Vec::new();
        }
        let mut out = Vec::new();
        if self.local {
            out.push(Channel::Local);
        }
        if self.email {
            out.push(Channel::Email);
        }
        out
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Channel {
    Local,
    Email,
}

impl Channel {
    fn suffix(self) -> &'static str {
        match self {
            Channel::Local => "local",
            Channel::Email => "email",
        }
    }
}

/// Local and email deliveries are logged separately, so turning email on
/// later doesn't re-send what already went out locally (and vice versa).
pub fn channel_key(base: &str, channel: Channel) -> String {
    format!("{base}:{}", channel.suffix())
}

/// The channels of `prefs` whose key for `base` isn't in the log yet.
pub fn pending_channels(
    prefs: CategoryPrefs,
    base: &str,
    already_sent: impl Fn(&str) -> bool,
) -> Vec<Channel> {
    prefs
        .channels()
        .into_iter()
        .filter(|c| !already_sent(&channel_key(base, *c)))
        .collect()
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct NotificationPrefs {
    pub reminders: CategoryPrefs,
    pub progress: CategoryPrefs,
    pub weekly_summary: CategoryPrefs,
    pub streak_pace: CategoryPrefs,
}

impl NotificationPrefs {
    pub fn get(&self, category: Category) -> CategoryPrefs {
        match category {
            Category::Reminders => self.reminders,
            Category::Progress => self.progress,
            Category::WeeklySummary => self.weekly_summary,
            Category::StreakPace => self.streak_pace,
        }
    }

    pub fn set(&mut self, category: Category, prefs: CategoryPrefs) {
        match category {
            Category::Reminders => self.reminders = prefs,
            Category::Progress => self.progress = prefs,
            Category::WeeklySummary => self.weekly_summary = prefs,
            Category::StreakPace => self.streak_pace = prefs,
        }
    }

    pub fn any_local(&self) -> bool {
        Category::ALL
            .iter()
            .any(|c| self.get(*c).channels().contains(&Channel::Local))
    }
}

/// A local-time window during which nothing is delivered. `start > end`
/// means it wraps past midnight (e.g. 22:00–07:00); `start == end` is an
/// empty window rather than a whole day, so a slip of the picker can't
/// silence everything.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QuietHours {
    pub start: Time,
    pub end: Time,
}

impl QuietHours {
    pub fn contains(self, t: Time) -> bool {
        if self.start <= self.end {
            self.start <= t && t < self.end
        } else {
            t >= self.start || t < self.end
        }
    }

    /// When something due at `at` may go out: `at` itself outside the
    /// window, otherwise the moment the window ends (the next day's `end`
    /// when it wraps past midnight and `at` is before it).
    pub fn release_time(self, at: PrimitiveDateTime) -> PrimitiveDateTime {
        if !self.contains(at.time()) {
            return at;
        }
        let end_today = PrimitiveDateTime::new(at.date(), self.end);
        if end_today > at {
            end_today
        } else {
            end_today + Duration::days(1)
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct NotificationSchedule {
    pub reminder_time: Time,
    pub weekly_day: Weekday,
    pub weekly_time: Time,
    pub streak_alert_time: Time,
    /// Pace alert fires when today's quantity exceeds this × the 28-day
    /// daily average.
    pub pace_threshold: f64,
    pub quiet_hours: Option<QuietHours>,
    pub bundle_progress: bool,
    pub launch_at_login: bool,
    pub close_to_tray: bool,
}

fn hm(hour: u8, minute: u8) -> Time {
    Time::from_hms(hour, minute, 0).expect("valid time")
}

impl Default for NotificationSchedule {
    fn default() -> Self {
        Self {
            reminder_time: hm(20, 0),
            weekly_day: Weekday::Sunday,
            weekly_time: hm(18, 0),
            streak_alert_time: hm(21, 0),
            pace_threshold: 1.5,
            quiet_hours: None,
            bundle_progress: true,
            launch_at_login: false,
            close_to_tray: true,
        }
    }
}

impl NotificationSchedule {
    /// `at` held back past quiet hours, if any.
    pub fn release_time(&self, at: PrimitiveDateTime) -> PrimitiveDateTime {
        self.quiet_hours.map_or(at, |q| q.release_time(at))
    }

    pub fn is_quiet(&self, t: Time) -> bool {
        self.quiet_hours.is_some_and(|q| q.contains(t))
    }
}

/// "HH:MM", 24-hour.
pub fn parse_clock(s: &str) -> Option<Time> {
    let (h, m) = s.split_once(':')?;
    if h.len() != 2 || m.len() != 2 {
        return None;
    }
    Time::from_hms(h.parse().ok()?, m.parse().ok()?, 0).ok()
}

pub fn format_clock(t: Time) -> String {
    format!("{:02}:{:02}", t.hour(), t.minute())
}

pub fn weekday_from_str(s: &str) -> Option<Weekday> {
    Some(match s {
        "monday" => Weekday::Monday,
        "tuesday" => Weekday::Tuesday,
        "wednesday" => Weekday::Wednesday,
        "thursday" => Weekday::Thursday,
        "friday" => Weekday::Friday,
        "saturday" => Weekday::Saturday,
        "sunday" => Weekday::Sunday,
        _ => return None,
    })
}

pub fn weekday_to_str(d: Weekday) -> &'static str {
    match d {
        Weekday::Monday => "monday",
        Weekday::Tuesday => "tuesday",
        Weekday::Wednesday => "wednesday",
        Weekday::Thursday => "thursday",
        Weekday::Friday => "friday",
        Weekday::Saturday => "saturday",
        Weekday::Sunday => "sunday",
    }
}

/// The time-of-day notifications. Pace and progress are event-driven and
/// don't appear here.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScheduledKind {
    Reminder,
    WeeklySummary,
    StreakAlert,
}

impl ScheduledKind {
    pub fn category(self) -> Category {
        match self {
            ScheduledKind::Reminder => Category::Reminders,
            ScheduledKind::WeeklySummary => Category::WeeklySummary,
            ScheduledKind::StreakAlert => Category::StreakPace,
        }
    }
}

/// One scheduled notification that is due now. `day` is the day it was
/// scheduled for, which is what its content and dedup key are about — not
/// necessarily today, when quiet hours pushed it past midnight.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Occurrence {
    pub kind: ScheduledKind,
    pub day: Date,
}

impl Occurrence {
    pub fn key(&self) -> String {
        match self.kind {
            ScheduledKind::Reminder => reminder_key(self.day),
            ScheduledKind::WeeklySummary => weekly_key(self.day),
            ScheduledKind::StreakAlert => streak_key(self.day),
        }
    }
}

pub fn reminder_key(day: Date) -> String {
    format!("reminders:{day}")
}

/// ISO week, e.g. "weekly_summary:2026-W43".
pub fn weekly_key(day: Date) -> String {
    let (year, week, _) = day.to_iso_week_date();
    format!("weekly_summary:{year}-W{week:02}")
}

pub fn streak_key(day: Date) -> String {
    format!("streak:{day}")
}

pub fn pace_key(habit: HabitId, day: Date) -> String {
    format!("pace:{}:{day}", habit.0)
}

pub fn progress_key(award_key: &str) -> String {
    format!("progress:{award_key}")
}

/// Which scheduled notifications are deliverable at `now`, before any
/// "already sent" or content check. An occurrence scheduled at S is
/// deliverable from its release time R (S, or the end of quiet hours if S
/// falls inside them) until the end of R's day — so a reminder missed while
/// the app was closed still catches up that evening, a reminder held by
/// quiet hours that wrap midnight goes out the next morning, and nothing
/// older than that is ever dug up. Disabled categories never appear.
pub fn due_occurrences(
    schedule: &NotificationSchedule,
    prefs: &NotificationPrefs,
    now: PrimitiveDateTime,
) -> Vec<Occurrence> {
    let mut out = Vec::new();
    // Yesterday matters only for occurrences quiet hours carried over.
    for day in [now.date() - Duration::days(1), now.date()] {
        let slots = [
            (ScheduledKind::Reminder, schedule.reminder_time, true),
            (
                ScheduledKind::WeeklySummary,
                schedule.weekly_time,
                day.weekday() == schedule.weekly_day,
            ),
            (ScheduledKind::StreakAlert, schedule.streak_alert_time, true),
        ];
        for (kind, at, applies) in slots {
            if !applies || prefs.get(kind.category()).channels().is_empty() {
                continue;
            }
            let release = schedule.release_time(PrimitiveDateTime::new(day, at));
            if release <= now && release.date() == now.date() {
                out.push(Occurrence { kind, day });
            }
        }
    }
    out
}

/// Whether event-driven notices (pace, progress) may go out at `now`.
pub fn can_deliver_now(schedule: &NotificationSchedule, now: PrimitiveDateTime) -> bool {
    !schedule.is_quiet(now.time())
}

/// The reminder is a nudge to log: once anything is logged for the day it
/// has nothing to say.
pub fn reminder_needed(entries_logged_that_day: usize) -> bool {
    entries_logged_that_day == 0
}

/// The streak alert's day count, when it should fire: the streak (today
/// included) is at least 2 days and today is still clean.
pub fn streak_alert(current_streak_days: i64, today_clean: bool) -> Option<i64> {
    (today_clean && current_streak_days >= 2).then_some(current_streak_days)
}

/// Days the pace baseline averages over, ending the day before.
pub const PACE_WINDOW_DAYS: i64 = 28;

#[derive(Debug, Clone, PartialEq)]
pub struct PaceAlert {
    pub habit_id: HabitId,
    pub habit_name: String,
    pub unit_label: String,
    pub today_quantity: f64,
    pub average_quantity: f64,
}

/// Harmful (life-costing), unarchived habits whose quantity on `day` is
/// above `threshold` × their daily average over the 28 days before it. A
/// habit with no history (average 0) never alerts: there's no pace to beat.
pub fn pace_alerts(
    entries: &[Entry],
    habits: &HashMap<HabitId, Habit>,
    day: Date,
    threshold: f64,
) -> Vec<PaceAlert> {
    let window_start = day - Duration::days(PACE_WINDOW_DAYS);
    let mut today: HashMap<HabitId, f64> = HashMap::new();
    let mut window: HashMap<HabitId, f64> = HashMap::new();
    for e in entries {
        let d = e.occurred_at.date();
        if d == day {
            *today.entry(e.habit_id).or_default() += e.quantity;
        } else if d >= window_start && d < day {
            *window.entry(e.habit_id).or_default() += e.quantity;
        }
    }
    let mut out: Vec<PaceAlert> = today
        .into_iter()
        .filter_map(|(id, qty)| {
            let habit = habits.get(&id)?;
            if habit.archived || habit.life_minutes_per_unit <= 0.0 {
                return None;
            }
            let average = window.get(&id).copied().unwrap_or(0.0) / PACE_WINDOW_DAYS as f64;
            (average > 0.0 && qty > threshold * average).then(|| PaceAlert {
                habit_id: id,
                habit_name: habit.name.clone(),
                unit_label: habit.unit_label.clone(),
                today_quantity: qty,
                average_quantity: average,
            })
        })
        .collect();
    out.sort_by_key(|a| a.habit_id);
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::macros::{date, datetime, time};

    fn all_on() -> NotificationPrefs {
        NotificationPrefs::default()
    }

    fn kinds(v: &[Occurrence]) -> Vec<(ScheduledKind, Date)> {
        v.iter().map(|o| (o.kind, o.day)).collect()
    }

    // 2026-10-21 is a Wednesday; 2026-10-25 a Sunday.

    #[test]
    fn reminder_is_not_due_before_its_time_and_is_after() {
        let s = NotificationSchedule::default();
        let before = due_occurrences(&s, &all_on(), datetime!(2026-10-21 19:59));
        assert!(before.is_empty());
        let at = due_occurrences(&s, &all_on(), datetime!(2026-10-21 20:00));
        assert_eq!(kinds(&at), vec![(ScheduledKind::Reminder, date!(2026 - 10 - 21))]);
        // Still deliverable later the same evening (app opened late).
        let late = due_occurrences(&s, &all_on(), datetime!(2026-10-21 23:59));
        assert!(kinds(&late).contains(&(ScheduledKind::Reminder, date!(2026 - 10 - 21))));
    }

    #[test]
    fn yesterdays_unsent_reminder_is_not_dug_up_without_quiet_hours() {
        let s = NotificationSchedule::default();
        let due = due_occurrences(&s, &all_on(), datetime!(2026-10-22 08:00));
        assert!(due.is_empty());
    }

    #[test]
    fn disabled_category_is_never_due() {
        let s = NotificationSchedule::default();
        let mut prefs = all_on();
        prefs.reminders.enabled = false;
        let due = due_occurrences(&s, &prefs, datetime!(2026-10-21 20:30));
        assert!(due.is_empty());
        // Enabled but with no channel ticked is the same as off.
        let mut prefs = all_on();
        prefs.reminders.local = false;
        assert!(due_occurrences(&s, &prefs, datetime!(2026-10-21 20:30)).is_empty());
    }

    #[test]
    fn quiet_hours_delay_until_the_window_ends() {
        let s = NotificationSchedule {
            quiet_hours: Some(QuietHours {
                start: time!(19:30),
                end: time!(21:15),
            }),
            ..Default::default()
        };
        assert!(due_occurrences(&s, &all_on(), datetime!(2026-10-21 20:30)).is_empty());
        let due = due_occurrences(&s, &all_on(), datetime!(2026-10-21 21:15));
        // Reminder (20:00) and streak alert (21:00) both released at 21:15.
        assert_eq!(
            kinds(&due),
            vec![
                (ScheduledKind::Reminder, date!(2026 - 10 - 21)),
                (ScheduledKind::StreakAlert, date!(2026 - 10 - 21)),
            ]
        );
    }

    #[test]
    fn quiet_hours_wrapping_midnight_carry_over_to_the_next_morning() {
        let s = NotificationSchedule {
            reminder_time: time!(23:30),
            quiet_hours: Some(QuietHours {
                start: time!(22:00),
                end: time!(07:00),
            }),
            ..Default::default()
        };
        let reminders = |now| {
            due_occurrences(&s, &all_on(), now)
                .into_iter()
                .filter(|o| o.kind == ScheduledKind::Reminder)
                .collect::<Vec<_>>()
        };
        // The 20th's reminder, carried over to this morning, is still
        // deliverable today; the 21st's own is held back.
        assert_eq!(
            kinds(&reminders(datetime!(2026-10-21 23:45))),
            vec![(ScheduledKind::Reminder, date!(2026 - 10 - 20))]
        );
        assert!(!reminders(datetime!(2026-10-22 06:59))
            .iter()
            .any(|o| o.day == date!(2026 - 10 - 21)));
        let due = reminders(datetime!(2026-10-22 07:00));
        // Released the next morning but still about, and keyed to, the 21st.
        assert_eq!(kinds(&due), vec![(ScheduledKind::Reminder, date!(2026 - 10 - 21))]);
        assert_eq!(due[0].key(), "reminders:2026-10-21");
    }

    #[test]
    fn quiet_hours_contains_handles_wrap_and_empty_window() {
        let wrap = QuietHours {
            start: time!(22:00),
            end: time!(07:00),
        };
        assert!(wrap.contains(time!(23:00)));
        assert!(wrap.contains(time!(03:00)));
        assert!(!wrap.contains(time!(07:00)));
        assert!(!wrap.contains(time!(12:00)));
        let empty = QuietHours {
            start: time!(09:00),
            end: time!(09:00),
        };
        assert!(!empty.contains(time!(09:00)));
        assert_eq!(
            wrap.release_time(datetime!(2026-10-22 03:00)),
            datetime!(2026-10-22 07:00)
        );
        assert_eq!(
            wrap.release_time(datetime!(2026-10-21 23:00)),
            datetime!(2026-10-22 07:00)
        );
    }

    #[test]
    fn event_notices_wait_out_quiet_hours() {
        let s = NotificationSchedule {
            quiet_hours: Some(QuietHours {
                start: time!(22:00),
                end: time!(07:00),
            }),
            ..Default::default()
        };
        assert!(!can_deliver_now(&s, datetime!(2026-10-21 23:00)));
        assert!(can_deliver_now(&s, datetime!(2026-10-22 07:00)));
    }

    #[test]
    fn weekly_summary_only_on_its_day() {
        let s = NotificationSchedule::default();
        let wednesday = due_occurrences(&s, &all_on(), datetime!(2026-10-21 18:30));
        assert!(!kinds(&wednesday)
            .iter()
            .any(|(k, _)| *k == ScheduledKind::WeeklySummary));
        let sunday = due_occurrences(&s, &all_on(), datetime!(2026-10-25 18:30));
        assert_eq!(
            kinds(&sunday),
            vec![(ScheduledKind::WeeklySummary, date!(2026 - 10 - 25))]
        );
        assert_eq!(sunday[0].key(), "weekly_summary:2026-W43");
        let s = NotificationSchedule {
            weekly_day: Weekday::Wednesday,
            ..Default::default()
        };
        let wednesday = due_occurrences(&s, &all_on(), datetime!(2026-10-21 18:30));
        assert_eq!(
            kinds(&wednesday),
            vec![(ScheduledKind::WeeklySummary, date!(2026 - 10 - 21))]
        );
    }

    #[test]
    fn already_logged_channels_are_skipped() {
        let base = "reminders:2026-10-21";
        let prefs = CategoryPrefs {
            enabled: true,
            local: true,
            email: true,
        };
        let sent = ["reminders:2026-10-21:local".to_string()];
        let pending = pending_channels(prefs, base, |k| sent.iter().any(|s| s == k));
        assert_eq!(pending, vec![Channel::Email]);
        let pending = pending_channels(prefs, base, |_| false);
        assert_eq!(pending, vec![Channel::Local, Channel::Email]);
        let pending = pending_channels(prefs, base, |_| true);
        assert!(pending.is_empty());
    }

    #[test]
    fn dedup_keys_match_the_spec() {
        let d = date!(2026 - 10 - 21);
        assert_eq!(reminder_key(d), "reminders:2026-10-21");
        assert_eq!(weekly_key(d), "weekly_summary:2026-W43");
        assert_eq!(weekly_key(date!(2026 - 01 - 01)), "weekly_summary:2026-W01");
        assert_eq!(streak_key(d), "streak:2026-10-21");
        assert_eq!(pace_key(HabitId(7), d), "pace:7:2026-10-21");
        assert_eq!(progress_key("clean:2026-10-20"), "progress:clean:2026-10-20");
        assert_eq!(
            channel_key(&progress_key("clean:2026-10-20"), Channel::Email),
            "progress:clean:2026-10-20:email"
        );
    }

    #[test]
    fn reminder_skipped_once_anything_is_logged() {
        assert!(reminder_needed(0));
        assert!(!reminder_needed(1));
    }

    #[test]
    fn streak_alert_needs_two_days_and_a_clean_today() {
        assert_eq!(streak_alert(0, true), None);
        assert_eq!(streak_alert(1, true), None);
        assert_eq!(streak_alert(2, true), Some(2));
        assert_eq!(streak_alert(9, true), Some(9));
        assert_eq!(streak_alert(9, false), None);
    }

    fn habit(id: i64, life_minutes: f64) -> (HabitId, Habit) {
        let mut h = Habit::new(format!("H{id}"), "unit");
        h.id = Some(HabitId(id));
        h.life_minutes_per_unit = life_minutes;
        (HabitId(id), h)
    }

    fn entry(id: i64, at: PrimitiveDateTime, qty: f64) -> Entry {
        Entry::new(HabitId(id), at, qty)
    }

    #[test]
    fn pace_alert_fires_above_threshold_times_average() {
        let habits: HashMap<_, _> = [habit(1, 11.0), habit(2, 0.0)].into_iter().collect();
        let day = date!(2026 - 10 - 21);
        let mut entries = Vec::new();
        // 28 units over the previous 28 days -> average 1/day.
        for i in 1..=28 {
            let d = PrimitiveDateTime::new(day - Duration::days(i), time!(12:00));
            entries.push(entry(1, d, 1.0));
            entries.push(entry(2, d, 1.0));
        }
        // Outside the window: ignored.
        entries.push(entry(
            1,
            PrimitiveDateTime::new(day - Duration::days(29), time!(12:00)),
            100.0,
        ));
        let today = |q: f64| entry(1, PrimitiveDateTime::new(day, time!(09:00)), q);

        let at_threshold = [entries.clone(), vec![today(1.5)]].concat();
        assert!(pace_alerts(&at_threshold, &habits, day, 1.5).is_empty());

        let mut over = [entries.clone(), vec![today(2.0)]].concat();
        // A good habit over its pace never alerts.
        over.push(entry(2, PrimitiveDateTime::new(day, time!(09:00)), 10.0));
        let alerts = pace_alerts(&over, &habits, day, 1.5);
        assert_eq!(alerts.len(), 1);
        assert_eq!(alerts[0].habit_id, HabitId(1));
        assert_eq!(alerts[0].today_quantity, 2.0);
        assert!((alerts[0].average_quantity - 1.0).abs() < 1e-9);
    }

    #[test]
    fn pace_alert_needs_a_nonzero_average() {
        let habits: HashMap<_, _> = [habit(1, 11.0)].into_iter().collect();
        let day = date!(2026 - 10 - 21);
        let entries = vec![entry(1, PrimitiveDateTime::new(day, time!(09:00)), 50.0)];
        assert!(pace_alerts(&entries, &habits, day, 1.5).is_empty());
    }

    #[test]
    fn clock_parsing_round_trips() {
        assert_eq!(parse_clock("07:05"), Some(time!(07:05)));
        assert_eq!(parse_clock("7:05"), None);
        assert_eq!(parse_clock("24:00"), None);
        assert_eq!(format_clock(time!(07:05)), "07:05");
        assert_eq!(weekday_from_str("sunday"), Some(Weekday::Sunday));
        assert_eq!(weekday_to_str(Weekday::Friday), "friday");
    }
}
