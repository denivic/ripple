use std::sync::Arc;

use rusqlite::{params, Row};
use time::{OffsetDateTime, PrimitiveDateTime};

use crate::domain::points::{AwardKind, PointAward};
use crate::domain::repository::{PointAwardRepository, RepoResult, RepositoryError};

use super::codec::{format_date, format_datetime, parse_date, parse_datetime};
use super::Db;

pub struct SqlitePointAwardRepository {
    db: Arc<Db>,
}

impl SqlitePointAwardRepository {
    pub fn new(db: Arc<Db>) -> Self {
        Self { db }
    }
}

const COLUMNS: &str = "id, award_key, kind, points, awarded_for, detail, created_at";

fn kind_to_str(k: AwardKind) -> &'static str {
    match k {
        AwardKind::LoggedDay => "logged",
        AwardKind::CleanDay => "clean",
        AwardKind::HabitImproved => "habit-improved",
        AwardKind::StreakBonus => "streak",
        AwardKind::Milestone => "milestone",
    }
}

fn kind_from_str(s: &str) -> Option<AwardKind> {
    match s {
        "logged" => Some(AwardKind::LoggedDay),
        "clean" => Some(AwardKind::CleanDay),
        "habit-improved" => Some(AwardKind::HabitImproved),
        "streak" => Some(AwardKind::StreakBonus),
        "milestone" => Some(AwardKind::Milestone),
        _ => None,
    }
}

type RawAward = (i64, String, String, i64, String, Option<String>, String);

fn row_to_raw(row: &Row) -> rusqlite::Result<RawAward> {
    Ok((
        row.get(0)?,
        row.get(1)?,
        row.get(2)?,
        row.get(3)?,
        row.get(4)?,
        row.get(5)?,
        row.get(6)?,
    ))
}

fn raw_to_award(raw: RawAward) -> RepoResult<PointAward> {
    let (id, award_key, kind, points, awarded_for, detail, created_at) = raw;
    let kind = kind_from_str(&kind)
        .ok_or_else(|| RepositoryError(format!("unknown award kind '{kind}'")))?;
    Ok(PointAward {
        id: Some(id),
        award_key,
        kind,
        points,
        awarded_for: parse_date(&awarded_for)?,
        detail,
        created_at: Some(parse_datetime(&created_at)?),
    })
}

/// `created_at` is UTC: it's bookkeeping for "most recent", not a day the
/// user would read, so it doesn't need the local clock entries use.
fn now_utc() -> PrimitiveDateTime {
    let now = OffsetDateTime::now_utc();
    PrimitiveDateTime::new(now.date(), now.time())
}

impl SqlitePointAwardRepository {
    fn query(&self, sql: &str, params: impl rusqlite::Params) -> RepoResult<Vec<PointAward>> {
        let conn = self.db.lock()?;
        let mut stmt = conn.prepare(sql)?;
        let raws = stmt
            .query_map(params, row_to_raw)?
            .collect::<rusqlite::Result<Vec<_>>>()?;
        raws.into_iter().map(raw_to_award).collect()
    }
}

impl PointAwardRepository for SqlitePointAwardRepository {
    fn insert_new(&self, awards: &[PointAward]) -> RepoResult<usize> {
        let created_at = format_datetime(now_utc());
        let mut conn = self.db.lock()?;
        let tx = conn.transaction()?;
        let mut inserted = 0;
        {
            let mut stmt = tx.prepare(
                "INSERT OR IGNORE INTO point_awards
                 (award_key, kind, points, awarded_for, detail, created_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            )?;
            for award in awards {
                inserted += stmt.execute(params![
                    award.award_key,
                    kind_to_str(award.kind),
                    award.points,
                    format_date(award.awarded_for),
                    award.detail,
                    created_at,
                ])?;
            }
        }
        tx.commit()?;
        Ok(inserted)
    }

    fn total_points(&self) -> RepoResult<i64> {
        let conn = self.db.lock()?;
        conn.query_row(
            "SELECT COALESCE(SUM(points), 0) FROM point_awards",
            [],
            |row| row.get(0),
        )
        .map_err(Into::into)
    }

    fn recent(&self, limit: usize) -> RepoResult<Vec<PointAward>> {
        self.query(
            &format!("SELECT {COLUMNS} FROM point_awards ORDER BY id DESC LIMIT ?1"),
            params![limit as i64],
        )
    }

    fn list_by_kind(&self, kind: AwardKind) -> RepoResult<Vec<PointAward>> {
        self.query(
            &format!("SELECT {COLUMNS} FROM point_awards WHERE kind = ?1 ORDER BY id"),
            params![kind_to_str(kind)],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::macros::date;

    fn repo() -> SqlitePointAwardRepository {
        SqlitePointAwardRepository::new(Arc::new(Db::open_in_memory().unwrap()))
    }

    fn clean(day: time::Date) -> PointAward {
        PointAward::candidate(
            format!("clean:{}", format_date(day)),
            AwardKind::CleanDay,
            10,
            day,
            None,
        )
    }

    #[test]
    fn insert_new_skips_keys_already_in_the_ledger() {
        let repo = repo();
        let first = [clean(date!(2026 - 10 - 19)), clean(date!(2026 - 10 - 20))];
        assert_eq!(repo.insert_new(&first).unwrap(), 2);
        let again = [clean(date!(2026 - 10 - 20)), clean(date!(2026 - 10 - 21))];
        assert_eq!(repo.insert_new(&again).unwrap(), 1);
        assert_eq!(repo.total_points().unwrap(), 30);
    }

    #[test]
    fn recent_is_newest_first_and_round_trips_fields() {
        let repo = repo();
        repo.insert_new(&[clean(date!(2026 - 10 - 19))]).unwrap();
        repo.insert_new(&[PointAward::candidate(
            "milestone:first-entry".to_string(),
            AwardKind::Milestone,
            10,
            date!(2026 - 10 - 18),
            Some("First ripple".to_string()),
        )])
        .unwrap();

        let recent = repo.recent(20).unwrap();
        assert_eq!(recent.len(), 2);
        assert_eq!(recent[0].kind, AwardKind::Milestone);
        assert_eq!(recent[0].awarded_for, date!(2026 - 10 - 18));
        assert_eq!(recent[0].detail.as_deref(), Some("First ripple"));
        assert!(recent[0].created_at.is_some());
        assert_eq!(repo.list_by_kind(AwardKind::Milestone).unwrap().len(), 1);
    }
}
