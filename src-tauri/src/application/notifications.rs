//! Notification scheduling and delivery. The decisions ("what is due at T")
//! are pure functions in `domain::notifications`; this module gathers the
//! state they need from the repositories, renders each notice and pushes it
//! through the local and email channels, recording dedup keys as it goes.

use std::collections::HashMap;
use std::sync::{Condvar, Mutex};

use time::{Date, Duration, PrimitiveDateTime};

use super::award_points;
use super::day_range;
use super::email::templates::{self, AwardLine, ImprovedHabit, WeeklySummary};
use super::email::EmailMessage;
use super::today_summary::compute_today_summary;
use crate::domain::habit::{Habit, HabitId};
use crate::domain::levels::level_for;
use crate::domain::notifications::{
    can_deliver_now, channel_key, due_occurrences, pace_alerts, pace_key, pending_channels,
    progress_key,
    reminder_needed, streak_alert, CategoryPrefs, Channel, Occurrence, ScheduledKind,
    PACE_WINDOW_DAYS,
};
use crate::domain::points::{AwardKind, PointAward};
use crate::domain::projections::{daily_series, is_clean_day};
use crate::domain::repository::{
    EntryRepository, HabitRepository, NotificationLogRepository,
    NotificationSettingsRepository, PointAwardRepository, ProfileRepository, RepoResult,
    RepositoryError, SettingsRepository,
};

/// Port for OS notifications, so the scheduler is testable without Tauri.
pub trait LocalNotifier {
    fn show(&self, title: &str, body: &str) -> Result<(), String>;
}

/// Port for notification email. `Ok(false)` means email isn't configured.
pub trait EmailSender {
    fn send(&self, msg: &EmailMessage) -> Result<bool, String>;
}

/// Everything the scheduler reads and writes.
pub struct NotificationRepos<'a> {
    pub habits: &'a dyn HabitRepository,
    pub entries: &'a dyn EntryRepository,
    pub profile: &'a dyn ProfileRepository,
    pub settings: &'a dyn SettingsRepository,
    pub awards: &'a dyn PointAwardRepository,
    pub notification_settings: &'a dyn NotificationSettingsRepository,
    pub log: &'a dyn NotificationLogRepository,
}

pub struct Channels<'a> {
    pub local: &'a dyn LocalNotifier,
    pub email: &'a dyn EmailSender,
}

/// One rendered notification. A bundle carries one dedup base per item, and
/// a channel counts as sent for the bundle only once all of them are logged.
pub struct Notice {
    pub keys: Vec<String>,
    pub title: String,
    pub body: String,
    pub email: EmailMessage,
}

/// Awards evaluation runs from both the `get_progress` command and the
/// scheduler thread. Holding this across "evaluate, then read back the N
/// newest rows" keeps one caller from picking up the other's awards.
static EVALUATION: Mutex<()> = Mutex::new(());

fn evaluation_guard() -> RepoResult<std::sync::MutexGuard<'static, ()>> {
    EVALUATION
        .lock()
        .map_err(|_| RepositoryError("evaluation mutex poisoned".to_string()))
}

/// `award_points::get_progress`, also returning the awards it just added so
/// they can be announced.
pub fn get_progress_with_new_awards(
    habit_repo: &dyn HabitRepository,
    entry_repo: &dyn EntryRepository,
    settings_repo: &dyn SettingsRepository,
    award_repo: &dyn PointAwardRepository,
    today: Date,
) -> RepoResult<(award_points::Progress, Vec<PointAward>)> {
    let _guard = evaluation_guard()?;
    let progress =
        award_points::get_progress(habit_repo, entry_repo, settings_repo, award_repo, today)?;
    let new = if progress.newly_awarded > 0 {
        award_repo.recent(progress.newly_awarded)?
    } else {
        Vec::new()
    };
    Ok((progress, new))
}

fn evaluate_new_awards(repos: &NotificationRepos, today: Date) -> RepoResult<Vec<PointAward>> {
    let _guard = evaluation_guard()?;
    let n = award_points::evaluate_awards(
        repos.habits,
        repos.entries,
        repos.settings,
        repos.awards,
        today,
    )?;
    if n == 0 {
        return Ok(Vec::new());
    }
    repos.awards.recent(n)
}

#[derive(Default)]
struct HubInner {
    pending_awards: Vec<PointAward>,
    wake: bool,
}

/// Hand-off between the `get_progress` hook and the scheduler thread: new
/// awards are queued here (in memory — they're also in the ledger, so a
/// restart loses only the announcement) and the thread is woken so they go
/// out without waiting for the next minute tick.
#[derive(Default)]
pub struct NotificationHub {
    inner: Mutex<HubInner>,
    cv: Condvar,
}

impl NotificationHub {
    pub fn enqueue_awards(&self, awards: Vec<PointAward>) {
        if awards.is_empty() {
            return;
        }
        if let Ok(mut inner) = self.inner.lock() {
            inner.pending_awards.extend(awards);
            inner.wake = true;
            self.cv.notify_all();
        }
    }

    /// Wakes the scheduler for an immediate tick (e.g. after settings change).
    pub fn wake(&self) {
        if let Ok(mut inner) = self.inner.lock() {
            inner.wake = true;
            self.cv.notify_all();
        }
    }

    /// Blocks until woken or `timeout` passes.
    pub fn wait(&self, timeout: std::time::Duration) {
        let Ok(guard) = self.inner.lock() else {
            std::thread::sleep(timeout);
            return;
        };
        if let Ok((mut guard, _)) = self.cv.wait_timeout_while(guard, timeout, |i| !i.wake) {
            guard.wake = false;
        }
    }

    fn take_awards(&self) -> Vec<PointAward> {
        self.inner
            .lock()
            .map(|mut i| std::mem::take(&mut i.pending_awards))
            .unwrap_or_default()
    }

    /// Returns undelivered awards to the front of the queue without waking.
    fn put_back(&self, mut awards: Vec<PointAward>) {
        if awards.is_empty() {
            return;
        }
        if let Ok(mut inner) = self.inner.lock() {
            awards.append(&mut inner.pending_awards);
            inner.pending_awards = awards;
        }
    }
}

/// How long a failed (or unconfigured) email waits before it's retried, so
/// a broken SMTP setup isn't hammered every minute.
const EMAIL_RETRY_MINUTES: i64 = 15;
/// More new awards than this in one evaluation (a first backfill, say) are
/// always bundled, whatever the setting, rather than flooding the screen.
const MAX_SEPARATE_PROGRESS: usize = 5;

/// Scheduler memory between ticks.
#[derive(Default)]
pub struct SchedulerState {
    last_points_day: Option<Date>,
    email_retry_after: HashMap<String, PrimitiveDateTime>,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct TickOutcome {
    /// Awards the daily evaluation added; the caller announces the change.
    pub new_awards: usize,
    /// Channel deliveries that succeeded.
    pub sent: usize,
}

/// Whether some channel of `prefs` still has an unlogged key in `keys`.
fn has_pending(
    prefs: CategoryPrefs,
    keys: &[String],
    log: &dyn NotificationLogRepository,
) -> RepoResult<bool> {
    // A log read error counts as "sent" inside the predicate (never risk a
    // duplicate) and is then surfaced to the caller.
    let error = std::cell::RefCell::new(None);
    for k in keys {
        let pending = pending_channels(prefs, k, |ck| {
            log.is_sent(ck).unwrap_or_else(|e| {
                error.borrow_mut().get_or_insert(e);
                true
            })
        });
        if let Some(e) = error.borrow_mut().take() {
            return Err(e);
        }
        if !pending.is_empty() {
            return Ok(true);
        }
    }
    Ok(false)
}

struct Delivery {
    sent: usize,
    /// False when a channel still owes a retry.
    complete: bool,
}

/// Sends `notice` on each channel of `prefs` that hasn't logged all its keys,
/// logging the keys after a successful send.
fn deliver(
    notice: &Notice,
    prefs: CategoryPrefs,
    log: &dyn NotificationLogRepository,
    channels: &Channels,
    state: &mut SchedulerState,
    now: PrimitiveDateTime,
) -> RepoResult<Delivery> {
    let mut out = Delivery {
        sent: 0,
        complete: true,
    };
    for channel in prefs.channels() {
        let mut unsent = Vec::new();
        for k in &notice.keys {
            let ck = channel_key(k, channel);
            if !log.is_sent(&ck)? {
                unsent.push(ck);
            }
        }
        let Some(first) = unsent.first().cloned() else {
            continue;
        };
        let result = match channel {
            Channel::Local => channels.local.show(&notice.title, &notice.body).map(|()| true),
            Channel::Email => {
                if state.email_retry_after.get(&first).is_some_and(|t| *t > now) {
                    out.complete = false;
                    continue;
                }
                channels.email.send(&notice.email)
            }
        };
        match result {
            Ok(true) => {
                for ck in &unsent {
                    log.mark_sent(ck)?;
                }
                state.email_retry_after.remove(&first);
                out.sent += 1;
            }
            // Email off: nothing to retry until the user sets it up, but
            // don't re-check every minute either.
            Ok(false) => {
                state
                    .email_retry_after
                    .insert(first, now + Duration::minutes(EMAIL_RETRY_MINUTES));
            }
            Err(e) => {
                eprintln!("notification '{}' failed on {channel:?}: {e}", notice.keys.join(","));
                if channel == Channel::Email {
                    state
                        .email_retry_after
                        .insert(first, now + Duration::minutes(EMAIL_RETRY_MINUTES));
                }
                out.complete = false;
            }
        }
    }
    Ok(out)
}

fn habit_map(repos: &NotificationRepos) -> RepoResult<HashMap<HabitId, Habit>> {
    Ok(repos
        .habits
        .list(true)?
        .into_iter()
        .filter_map(|h| h.id.map(|id| (id, h)))
        .collect())
}

fn date_label(day: Date) -> String {
    format!("{} {} {}", day.weekday(), day.day(), day.month())
}

fn reminder_notice(repos: &NotificationRepos, occ: &Occurrence) -> RepoResult<Option<Notice>> {
    let (start, end) = day_range(occ.day);
    if !reminder_needed(repos.entries.list_between(start, end)?.len()) {
        return Ok(None);
    }
    let label = date_label(occ.day);
    Ok(Some(Notice {
        keys: vec![occ.key()],
        title: "Time to log today's habits".to_string(),
        body: format!("Nothing is logged for {label} yet."),
        email: templates::reminder(&label),
    }))
}

fn streak_notice(repos: &NotificationRepos, occ: &Occurrence) -> RepoResult<Option<Notice>> {
    let habits = habit_map(repos)?;
    let summary = compute_today_summary(repos.habits, repos.entries, repos.profile, occ.day)?;
    let (start, end) = day_range(occ.day);
    let day_entries = repos.entries.list_between(start, end)?;
    let today_clean = daily_series(&day_entries, &habits, occ.day, occ.day)
        .first()
        .is_some_and(is_clean_day);
    let Some(days) = streak_alert(summary.current_streak_days, today_clean) else {
        return Ok(None);
    };
    Ok(Some(Notice {
        keys: vec![occ.key()],
        title: format!("{days}-day clean streak"),
        body: format!("You're on a {days}-day clean streak — keep today clean"),
        email: templates::streak_alert(days.max(0) as u32),
    }))
}

fn weekly_notice(repos: &NotificationRepos, occ: &Occurrence) -> RepoResult<Notice> {
    let day = occ.day;
    let habits = habit_map(repos)?;
    let summary = compute_today_summary(repos.habits, repos.entries, repos.profile, day)?;
    let week_start = day - Duration::days(6);
    let mut points_this_week = 0;
    for kind in [
        AwardKind::LoggedDay,
        AwardKind::CleanDay,
        AwardKind::HabitImproved,
        AwardKind::StreakBonus,
        AwardKind::Milestone,
    ] {
        points_this_week += repos
            .awards
            .list_by_kind(kind)?
            .iter()
            .filter(|a| a.awarded_for >= week_start && a.awarded_for <= day)
            .map(|a| a.points)
            .sum::<i64>();
    }
    // Largest relative drop among harmful habits used in both weeks.
    let most_improved = summary
        .habit_week_trends
        .iter()
        .filter_map(|t| {
            let h = habits.get(&t.habit_id)?;
            let drop = t.previous_week_quantity - t.current_week_quantity;
            (h.life_minutes_per_unit > 0.0 && t.previous_week_quantity > 0.0 && drop > 0.0)
                .then(|| (drop / t.previous_week_quantity, h, drop))
        })
        .max_by(|a, b| a.0.total_cmp(&b.0))
        .map(|(_, h, drop)| ImprovedHabit {
            name: h.name.clone(),
            detail: format!("{} fewer {} than last week", trim_quantity(drop), h.unit_label),
        });
    let week_label = format!(
        "{} {} – {} {}",
        week_start.day(),
        week_start.month(),
        day.day(),
        day.month()
    );
    // Template convention: positive means life gained.
    let this_week = -summary.current_week_life_minutes;
    let last_week = -summary.previous_week_life_minutes;
    let streak = summary.current_streak_days.max(0) as u32;
    Ok(Notice {
        keys: vec![occ.key()],
        title: "Your week in Ripple".to_string(),
        body: format!(
            "Net life {} (last week {}), +{points_this_week} points, {streak}-day clean streak.",
            templates::format_minutes(this_week),
            templates::format_minutes(last_week),
        ),
        email: templates::weekly_summary(&WeeklySummary {
            week_label,
            net_minutes_this_week: this_week,
            net_minutes_last_week: last_week,
            points_this_week,
            clean_streak_days: streak,
            most_improved,
        }),
    })
}

fn trim_quantity(q: f64) -> String {
    if (q - q.round()).abs() < 0.05 {
        format!("{}", q.round() as i64)
    } else {
        format!("{q:.1}")
    }
}

fn pace_notices(repos: &NotificationRepos, day: Date, threshold: f64) -> RepoResult<Vec<Notice>> {
    let habits = habit_map(repos)?;
    let (start, _) = day_range(day - Duration::days(PACE_WINDOW_DAYS));
    let (_, end) = day_range(day);
    let entries = repos.entries.list_between(start, end)?;
    Ok(pace_alerts(&entries, &habits, day, threshold)
        .into_iter()
        .map(|a| Notice {
            keys: vec![pace_key(a.habit_id, day)],
            title: format!("{} is above your usual pace", a.habit_name),
            body: format!(
                "{} {} today vs about {} a day over the last 4 weeks.",
                trim_quantity(a.today_quantity),
                a.unit_label,
                trim_quantity(a.average_quantity)
            ),
            email: templates::pace_alert(
                &a.habit_name,
                &a.unit_label,
                a.today_quantity,
                a.average_quantity,
            ),
        })
        .collect())
}

/// Awards plus any level they crossed into, as notice lines. The level-up
/// is keyed by level name, so it's announced once ever.
fn progress_lines(awards: &[PointAward], total_points: i64) -> Vec<(String, AwardLine)> {
    let mut lines: Vec<(String, AwardLine)> = awards
        .iter()
        .map(|a| {
            (
                progress_key(&a.award_key),
                AwardLine {
                    title: a.label(),
                    points: a.points,
                },
            )
        })
        .collect();
    let gained: i64 = awards.iter().map(|a| a.points).sum();
    let before = level_for(total_points - gained);
    let after = level_for(total_points);
    if after.index > before.index {
        lines.insert(
            0,
            (
                progress_key(&format!("level:{}", after.name)),
                AwardLine {
                    title: format!("Reached level {} — {}", after.index + 1, after.name),
                    points: 0,
                },
            ),
        );
    }
    lines
}

fn progress_notice(lines: &[(String, AwardLine)]) -> Notice {
    let awards: Vec<AwardLine> = lines.iter().map(|(_, l)| l.clone()).collect();
    let total: i64 = awards.iter().map(|a| a.points).sum();
    let (title, body) = match awards.as_slice() {
        [one] if one.points == 0 => (one.title.clone(), "Keep it up.".to_string()),
        [one] => (format!("+{} points", one.points), one.title.clone()),
        many => {
            let shown: Vec<&str> = many.iter().take(3).map(|a| a.title.as_str()).collect();
            let more = many.len().saturating_sub(3);
            let mut body = shown.join(", ");
            if more > 0 {
                body.push_str(&format!(" and {more} more"));
            }
            (format!("+{total} points"), body)
        }
    };
    Notice {
        keys: lines.iter().map(|(k, _)| k.clone()).collect(),
        title,
        body,
        email: templates::progress(&awards),
    }
}

/// Sends what the queued awards owe. Returns the awards still owed (quiet
/// hours, or a channel to retry) so the caller can queue them again.
fn deliver_progress(
    repos: &NotificationRepos,
    channels: &Channels,
    state: &mut SchedulerState,
    prefs: CategoryPrefs,
    bundle: bool,
    awards: Vec<PointAward>,
    now: PrimitiveDateTime,
) -> RepoResult<(usize, Vec<PointAward>)> {
    if awards.is_empty() || prefs.channels().is_empty() {
        // Off: announcements made while off are dropped, not saved up.
        return Ok((0, Vec::new()));
    }
    let lines = progress_lines(&awards, repos.awards.total_points()?);
    let mut owed: Vec<(String, AwardLine)> = Vec::new();
    for line in lines {
        if has_pending(prefs, std::slice::from_ref(&line.0), repos.log)? {
            owed.push(line);
        }
    }
    let mut sent = 0;
    let mut incomplete_keys: Vec<String> = Vec::new();
    let groups: Vec<Vec<(String, AwardLine)>> = if bundle || owed.len() > MAX_SEPARATE_PROGRESS {
        if owed.is_empty() {
            Vec::new()
        } else {
            vec![owed]
        }
    } else {
        owed.into_iter().map(|l| vec![l]).collect()
    };
    for group in groups {
        let notice = progress_notice(&group);
        let d = deliver(&notice, prefs, repos.log, channels, state, now)?;
        sent += d.sent;
        if !d.complete {
            incomplete_keys.extend(notice.keys);
        }
    }
    let retry = awards
        .into_iter()
        .filter(|a| incomplete_keys.contains(&progress_key(&a.award_key)))
        .collect();
    Ok((sent, retry))
}

/// One scheduler pass at local time `now`.
pub fn tick(
    repos: &NotificationRepos,
    channels: &Channels,
    hub: &NotificationHub,
    state: &mut SchedulerState,
    now: PrimitiveDateTime,
) -> RepoResult<TickOutcome> {
    let mut outcome = TickOutcome::default();
    let today = now.date();

    // Points evaluation once per local day, so clean-day awards land (and
    // are announced) while the window is closed.
    if state.last_points_day != Some(today) {
        let new = evaluate_new_awards(repos, today)?;
        outcome.new_awards = new.len();
        hub.enqueue_awards(new);
        state.last_points_day = Some(today);
    }

    let prefs = repos.notification_settings.get_prefs()?;
    let schedule = repos.notification_settings.get_schedule()?;

    for occ in due_occurrences(&schedule, &prefs, now) {
        let cat_prefs = prefs.get(occ.kind.category());
        if !has_pending(cat_prefs, &[occ.key()], repos.log)? {
            continue;
        }
        let notice = match occ.kind {
            ScheduledKind::Reminder => reminder_notice(repos, &occ)?,
            ScheduledKind::StreakAlert => streak_notice(repos, &occ)?,
            ScheduledKind::WeeklySummary => Some(weekly_notice(repos, &occ)?),
        };
        if let Some(notice) = notice {
            outcome.sent += deliver(&notice, cat_prefs, repos.log, channels, state, now)?.sent;
        }
    }

    let deliverable = can_deliver_now(&schedule, now);
    let pace_prefs = prefs.streak_pace;
    if deliverable && !pace_prefs.channels().is_empty() {
        // Yesterday too when quiet hours wrap midnight: an alert held over
        // night is still owed this morning.
        let mut days = vec![today];
        if schedule.quiet_hours.is_some_and(|q| q.start > q.end) {
            days.insert(0, today - Duration::days(1));
        }
        for day in days {
            for notice in pace_notices(repos, day, schedule.pace_threshold)? {
                if has_pending(pace_prefs, &notice.keys, repos.log)? {
                    outcome.sent +=
                        deliver(&notice, pace_prefs, repos.log, channels, state, now)?.sent;
                }
            }
        }
    }

    let queued = hub.take_awards();
    if deliverable {
        let (sent, retry) = deliver_progress(
            repos,
            channels,
            state,
            prefs.progress,
            schedule.bundle_progress,
            queued,
            now,
        )?;
        outcome.sent += sent;
        hub.put_back(retry);
    } else if !prefs.progress.channels().is_empty() {
        hub.put_back(queued);
    }
    Ok(outcome)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::sync::Arc;
    use time::macros::{date, datetime, time};

    use crate::domain::entry::Entry;
    use crate::domain::notifications::{NotificationPrefs, NotificationSchedule, QuietHours};
    use crate::infrastructure::db::{
        Db, SqliteEntryRepository, SqliteHabitRepository, SqliteNotificationRepository,
        SqlitePointAwardRepository, SqliteProfileRepository, SqliteSettingsRepository,
    };

    #[derive(Default)]
    struct FakeLocal(RefCell<Vec<String>>);
    impl LocalNotifier for FakeLocal {
        fn show(&self, title: &str, _body: &str) -> Result<(), String> {
            self.0.borrow_mut().push(title.to_string());
            Ok(())
        }
    }

    struct FakeEmail {
        result: Result<bool, String>,
        sent: RefCell<Vec<String>>,
    }
    impl EmailSender for FakeEmail {
        fn send(&self, msg: &EmailMessage) -> Result<bool, String> {
            if matches!(self.result, Ok(true)) {
                self.sent.borrow_mut().push(msg.subject.clone());
            }
            self.result.clone()
        }
    }

    struct Fixture {
        db: Arc<Db>,
        habits: SqliteHabitRepository,
        entries: SqliteEntryRepository,
        profile: SqliteProfileRepository,
        settings: SqliteSettingsRepository,
        awards: SqlitePointAwardRepository,
        notif: SqliteNotificationRepository,
    }

    impl Fixture {
        fn new() -> Self {
            let db = Arc::new(Db::open_in_memory().unwrap());
            Self {
                habits: SqliteHabitRepository::new(Arc::clone(&db)),
                entries: SqliteEntryRepository::new(Arc::clone(&db)),
                profile: SqliteProfileRepository::new(Arc::clone(&db)),
                settings: SqliteSettingsRepository::new(Arc::clone(&db)),
                awards: SqlitePointAwardRepository::new(Arc::clone(&db)),
                notif: SqliteNotificationRepository::new(Arc::clone(&db)),
                db,
            }
        }

        fn repos(&self) -> NotificationRepos<'_> {
            NotificationRepos {
                habits: &self.habits,
                entries: &self.entries,
                profile: &self.profile,
                settings: &self.settings,
                awards: &self.awards,
                notification_settings: &self.notif,
                log: &self.notif,
            }
        }
    }

    fn email_ok() -> FakeEmail {
        FakeEmail {
            result: Ok(true),
            sent: RefCell::new(Vec::new()),
        }
    }

    #[test]
    fn reminder_sends_once_per_day_and_per_channel() {
        let f = Fixture::new();
        let mut prefs = NotificationPrefs::default();
        prefs.reminders.email = true;
        f.notif.save_prefs(&prefs).unwrap();
        let (local, email) = (FakeLocal::default(), email_ok());
        let channels = Channels { local: &local, email: &email };
        let hub = NotificationHub::default();
        let mut state = SchedulerState::default();

        tick(&f.repos(), &channels, &hub, &mut state, datetime!(2026-10-21 19:59)).unwrap();
        assert!(local.0.borrow().is_empty());

        tick(&f.repos(), &channels, &hub, &mut state, datetime!(2026-10-21 20:00)).unwrap();
        tick(&f.repos(), &channels, &hub, &mut state, datetime!(2026-10-21 20:01)).unwrap();
        assert_eq!(local.0.borrow().as_slice(), ["Time to log today's habits"]);
        assert_eq!(email.sent.borrow().len(), 1);
        assert!(f.notif.is_sent("reminders:2026-10-21:local").unwrap());
        assert!(f.notif.is_sent("reminders:2026-10-21:email").unwrap());
    }

    #[test]
    fn reminder_skipped_when_something_was_logged() {
        let f = Fixture::new();
        let id = f.habits.insert(&Habit::new("Coffee", "cup")).unwrap();
        f.entries
            .insert(&Entry::new(id, datetime!(2026-10-21 08:00), 1.0))
            .unwrap();
        let local = FakeLocal::default();
        let email = email_ok();
        let channels = Channels { local: &local, email: &email };
        let mut state = SchedulerState::default();
        tick(
            &f.repos(),
            &channels,
            &NotificationHub::default(),
            &mut state,
            datetime!(2026-10-21 20:30),
        )
        .unwrap();
        assert!(!local.0.borrow().iter().any(|t| t.contains("log today")));
    }

    #[test]
    fn failed_email_is_not_logged_and_waits_before_retrying() {
        let f = Fixture::new();
        let mut prefs = NotificationPrefs::default();
        prefs.reminders = CategoryPrefs { enabled: true, local: false, email: true };
        prefs.streak_pace.enabled = false;
        f.notif.save_prefs(&prefs).unwrap();
        let local = FakeLocal::default();
        let email = FakeEmail {
            result: Err("smtp down".to_string()),
            sent: RefCell::new(Vec::new()),
        };
        let channels = Channels { local: &local, email: &email };
        let hub = NotificationHub::default();
        let mut state = SchedulerState::default();
        tick(&f.repos(), &channels, &hub, &mut state, datetime!(2026-10-21 20:00)).unwrap();
        assert!(!f.notif.is_sent("reminders:2026-10-21:email").unwrap());
        assert!(state.email_retry_after.contains_key("reminders:2026-10-21:email"));

        let ok = email_ok();
        let channels = Channels { local: &local, email: &ok };
        tick(&f.repos(), &channels, &hub, &mut state, datetime!(2026-10-21 20:05)).unwrap();
        assert!(ok.sent.borrow().is_empty());
        tick(&f.repos(), &channels, &hub, &mut state, datetime!(2026-10-21 20:15)).unwrap();
        assert_eq!(ok.sent.borrow().len(), 1);
        assert!(f.notif.is_sent("reminders:2026-10-21:email").unwrap());
    }

    fn award(key: &str, points: i64) -> PointAward {
        PointAward::candidate(
            key.to_string(),
            AwardKind::CleanDay,
            points,
            date!(2026 - 10 - 20),
            None,
        )
    }

    #[test]
    fn progress_is_bundled_and_held_through_quiet_hours() {
        let f = Fixture::new();
        f.notif
            .save_schedule(&NotificationSchedule {
                quiet_hours: Some(QuietHours { start: time!(22:00), end: time!(07:00) }),
                ..Default::default()
            })
            .unwrap();
        let mut prefs = NotificationPrefs::default();
        prefs.reminders.enabled = false;
        prefs.streak_pace.enabled = false;
        f.notif.save_prefs(&prefs).unwrap();
        let local = FakeLocal::default();
        let email = email_ok();
        let channels = Channels { local: &local, email: &email };
        let hub = NotificationHub::default();
        let mut state = SchedulerState {
            last_points_day: Some(date!(2026 - 10 - 21)),
            ..Default::default()
        };
        hub.enqueue_awards(vec![award("clean:2026-10-19", 10), award("clean:2026-10-20", 10)]);

        tick(&f.repos(), &channels, &hub, &mut state, datetime!(2026-10-21 23:00)).unwrap();
        assert!(local.0.borrow().is_empty());

        state.last_points_day = Some(date!(2026 - 10 - 22));
        tick(&f.repos(), &channels, &hub, &mut state, datetime!(2026-10-22 07:00)).unwrap();
        assert_eq!(local.0.borrow().as_slice(), ["+20 points"]);
        assert!(f.notif.is_sent("progress:clean:2026-10-19:local").unwrap());
        assert!(f.notif.is_sent("progress:clean:2026-10-20:local").unwrap());

        // Re-queuing the same awards sends nothing.
        hub.enqueue_awards(vec![award("clean:2026-10-20", 10)]);
        tick(&f.repos(), &channels, &hub, &mut state, datetime!(2026-10-22 07:01)).unwrap();
        assert_eq!(local.0.borrow().len(), 1);
    }

    #[test]
    fn progress_unbundled_sends_one_per_award() {
        let f = Fixture::new();
        f.notif
            .save_schedule(&NotificationSchedule {
                bundle_progress: false,
                ..Default::default()
            })
            .unwrap();
        let local = FakeLocal::default();
        let email = email_ok();
        let channels = Channels { local: &local, email: &email };
        let hub = NotificationHub::default();
        let mut state = SchedulerState {
            last_points_day: Some(date!(2026 - 10 - 21)),
            ..Default::default()
        };
        hub.enqueue_awards(vec![award("clean:2026-10-19", 10), award("clean:2026-10-20", 10)]);
        tick(&f.repos(), &channels, &hub, &mut state, datetime!(2026-10-21 10:00)).unwrap();
        assert_eq!(local.0.borrow().len(), 2);
    }

    #[test]
    fn daily_evaluation_awards_and_announces_once_per_day() {
        let f = Fixture::new();
        let id = f.habits.insert(&Habit::new("Coffee", "cup")).unwrap();
        f.entries
            .insert(&Entry::new(id, datetime!(2026-10-20 08:00), 1.0))
            .unwrap();
        let local = FakeLocal::default();
        let email = email_ok();
        let channels = Channels { local: &local, email: &email };
        let hub = NotificationHub::default();
        let mut state = SchedulerState::default();
        let out =
            tick(&f.repos(), &channels, &hub, &mut state, datetime!(2026-10-21 00:01)).unwrap();
        assert!(out.new_awards > 0);
        assert!(f.awards.total_points().unwrap() > 0);
        assert!(!local.0.borrow().is_empty());
        let out =
            tick(&f.repos(), &channels, &hub, &mut state, datetime!(2026-10-21 00:02)).unwrap();
        assert_eq!(out.new_awards, 0);
        drop(f.db);
    }

    #[test]
    fn progress_lines_announce_a_level_up() {
        let lines = progress_lines(&[award("clean:2026-10-20", 10)], 105);
        assert_eq!(lines.len(), 2);
        assert_eq!(lines[0].0, "progress:level:Ripple");
        let lines = progress_lines(&[award("clean:2026-10-20", 10)], 50);
        assert_eq!(lines.len(), 1);
    }

    #[test]
    fn weekly_summary_reports_a_harmful_week_as_life_lost() {
        let f = Fixture::new();
        let mut cigs = Habit::new("Cigarettes", "cigarette");
        cigs.life_minutes_per_unit = 20.0;
        let id = f.habits.insert(&cigs).unwrap();
        f.entries
            .insert(&Entry::new(id, datetime!(2026-10-24 09:00), 3.0))
            .unwrap();
        // 2026-10-25 is a Sunday, the default weekly day.
        let occ = Occurrence {
            kind: ScheduledKind::WeeklySummary,
            day: date!(2026 - 10 - 25),
        };
        let notice = weekly_notice(&f.repos(), &occ).unwrap();
        // 60 life-minutes lost: the template's convention is negative = lost.
        assert!(
            notice.email.text.contains("Net life, this week: −1 h"),
            "{}",
            notice.email.text
        );
        assert!(notice.body.starts_with("Net life −1 h"), "{}", notice.body);
    }
}
