/// Water-themed levels, by the total points needed to reach each one.
pub const LEVELS: [(&str, i64); 8] = [
    ("Drop", 0),
    ("Ripple", 100),
    ("Stream", 300),
    ("Current", 700),
    ("Wave", 1500),
    ("Tide", 3000),
    ("Ocean", 6000),
    ("Still Water", 12000),
];

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LevelProgress {
    pub name: &'static str,
    pub index: usize,
    pub next_name: Option<&'static str>,
    pub points_into_level: i64,
    /// `None` at the top level, where there's nothing left to reach.
    pub points_to_next: Option<i64>,
    /// 0..=1 share of the way to the next level; 1 at the top level.
    pub fraction: f64,
}

pub fn level_for(points: i64) -> LevelProgress {
    let points = points.max(0);
    let index = LEVELS
        .iter()
        .rposition(|&(_, threshold)| points >= threshold)
        .unwrap_or(0);
    let (name, threshold) = LEVELS[index];
    let points_into_level = points - threshold;
    match LEVELS.get(index + 1) {
        Some(&(next_name, next_threshold)) => {
            let span = next_threshold - threshold;
            LevelProgress {
                name,
                index,
                next_name: Some(next_name),
                points_into_level,
                points_to_next: Some(next_threshold - points),
                fraction: points_into_level as f64 / span as f64,
            }
        }
        None => LevelProgress {
            name,
            index,
            next_name: None,
            points_into_level,
            points_to_next: None,
            fraction: 1.0,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thresholds_map_to_their_level() {
        for (i, &(name, threshold)) in LEVELS.iter().enumerate() {
            let level = level_for(threshold);
            assert_eq!(level.name, name);
            assert_eq!(level.index, i);
            assert_eq!(level.points_into_level, 0);
            if threshold > 0 {
                assert_eq!(level_for(threshold - 1).index, i - 1);
            }
        }
    }

    #[test]
    fn progress_within_a_level() {
        let level = level_for(150);
        assert_eq!(level.name, "Ripple");
        assert_eq!(level.next_name, Some("Stream"));
        assert_eq!(level.points_into_level, 50);
        assert_eq!(level.points_to_next, Some(150));
        assert_eq!(level.fraction, 0.25);
    }

    #[test]
    fn top_level_has_no_next() {
        let level = level_for(20_000);
        assert_eq!(level.name, "Still Water");
        assert_eq!(level.index, 7);
        assert_eq!(level.next_name, None);
        assert_eq!(level.points_to_next, None);
        assert_eq!(level.fraction, 1.0);
    }
}
