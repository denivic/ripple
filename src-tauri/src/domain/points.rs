use time::{Date, PrimitiveDateTime};

/// Points are a ledger, not a projection: each award is stored once under a
/// unique `award_key` and never removed, so the total only ever goes up even
/// if the history it was earned from is later edited.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AwardKind {
    LoggedDay,
    CleanDay,
    HabitImproved,
    StreakBonus,
    Milestone,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PointAward {
    pub id: Option<i64>,
    /// Idempotency key, e.g. "clean:2026-10-20" or "milestone:first-entry".
    pub award_key: String,
    pub kind: AwardKind,
    pub points: i64,
    /// The day the award was earned for, not when it was recorded.
    pub awarded_for: Date,
    pub detail: Option<String>,
    /// Set by the repository on insert; `None` for a not-yet-stored candidate.
    pub created_at: Option<PrimitiveDateTime>,
}

impl PointAward {
    pub fn candidate(
        award_key: String,
        kind: AwardKind,
        points: i64,
        awarded_for: Date,
        detail: Option<String>,
    ) -> Self {
        Self {
            id: None,
            award_key,
            kind,
            points,
            awarded_for,
            detail,
            created_at: None,
        }
    }

    /// Short human-readable reason, e.g. for "+10 · Clean day".
    pub fn label(&self) -> String {
        match (self.kind, self.detail.as_deref()) {
            (AwardKind::LoggedDay, _) => "Day logged".to_string(),
            (AwardKind::CleanDay, _) => "Clean day".to_string(),
            (AwardKind::HabitImproved, Some(habit)) => format!("{habit} improved"),
            (AwardKind::HabitImproved, None) => "Habit improved".to_string(),
            (AwardKind::StreakBonus, Some(d)) | (AwardKind::Milestone, Some(d)) => d.to_string(),
            (AwardKind::StreakBonus, None) => "Clean streak".to_string(),
            (AwardKind::Milestone, None) => "Milestone".to_string(),
        }
    }
}

/// One-off achievements. `key` is stored as "milestone:<key>".
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Milestone {
    pub key: &'static str,
    pub title: &'static str,
    pub description: &'static str,
    pub points: i64,
}

pub const MILESTONE_FIRST_ENTRY: &str = "first-entry";
pub const MILESTONE_STREAK_7: &str = "streak-7";
pub const MILESTONE_STREAK_30: &str = "streak-30";
pub const MILESTONE_STREAK_100: &str = "streak-100";
pub const MILESTONE_FIRST_IMPROVED_WEEK: &str = "first-improved-week";
pub const MILESTONE_FIRST_HALVED_HABIT: &str = "first-halved-habit";
pub const MILESTONE_POINTS_1000: &str = "points-1000";

pub const MILESTONES: [Milestone; 7] = [
    Milestone {
        key: MILESTONE_FIRST_ENTRY,
        title: "First ripple",
        description: "Log your first entry.",
        points: 10,
    },
    Milestone {
        key: MILESTONE_STREAK_7,
        title: "Calm week",
        description: "Reach a 7-day clean streak.",
        points: 50,
    },
    Milestone {
        key: MILESTONE_STREAK_30,
        title: "Calm month",
        description: "Reach a 30-day clean streak.",
        points: 200,
    },
    Milestone {
        key: MILESTONE_STREAK_100,
        title: "Deep calm",
        description: "Reach a 100-day clean streak.",
        points: 500,
    },
    Milestone {
        key: MILESTONE_FIRST_IMPROVED_WEEK,
        title: "Turning the tide",
        description: "Improve a habit week over week for the first time.",
        points: 50,
    },
    Milestone {
        key: MILESTONE_FIRST_HALVED_HABIT,
        title: "Half as much",
        description: "Cut a harmful habit by half or more week over week.",
        points: 100,
    },
    Milestone {
        key: MILESTONE_POINTS_1000,
        title: "Thousand drops",
        description: "Earn 1,000 points.",
        points: 0,
    },
];

pub fn milestone(key: &str) -> Option<&'static Milestone> {
    MILESTONES.iter().find(|m| m.key == key)
}

pub fn milestone_award_key(key: &str) -> String {
    format!("milestone:{key}")
}

/// Builds the ledger row for a milestone. Panics on an unknown key, which
/// would be a typo in this module rather than bad user data.
pub fn milestone_award(key: &str, awarded_for: Date) -> PointAward {
    let m = milestone(key).expect("known milestone key");
    PointAward::candidate(
        milestone_award_key(m.key),
        AwardKind::Milestone,
        m.points,
        awarded_for,
        Some(m.title.to_string()),
    )
}
