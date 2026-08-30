use time::Date;

use super::entry::Entry;
use super::habit::Habit;
use super::profile::{Profile, Sex};

// Habit has no category/kind field yet, so nothing can identify "this entry
// is alcohol" or "...caffeine" to route through the Widmark/caffeine
// functions below — that needs a habit-category concept this phase doesn't
// add. Kept ready (and unit-tested) for whenever that lands, rather than
// wired to a guess; each item that's unreachable until then is marked
// individually below.

/// Defaults cited in plan-v1.md, offered as starting points for user-editable
/// per-habit `life_minutes_per_unit`, not as claims of clinical precision.
pub const DEFAULT_LIFE_MINUTES_PER_CIGARETTE: f64 = 11.0;
pub const DEFAULT_LIFE_MINUTES_PER_STANDARD_DRINK: f64 = 30.0;

pub struct HabitPreset {
    pub name: &'static str,
    pub unit_label: &'static str,
    pub life_minutes_per_unit: f64,
}

/// Quick-start suggestions for the "add habit" flow — a starting point the
/// user edits, not a fixed taxonomy.
pub fn default_habit_presets() -> Vec<HabitPreset> {
    vec![
        HabitPreset {
            name: "Cigarettes",
            unit_label: "cigarette",
            life_minutes_per_unit: DEFAULT_LIFE_MINUTES_PER_CIGARETTE,
        },
        HabitPreset {
            name: "Alcohol",
            unit_label: "drink",
            life_minutes_per_unit: DEFAULT_LIFE_MINUTES_PER_STANDARD_DRINK,
        },
    ]
}

/// Grams of pure alcohol in one US standard drink (NIAAA definition).
#[allow(dead_code)]
pub const STANDARD_DRINK_GRAMS_ALCOHOL: f64 = 14.0;

// Widmark distribution ratio r: population-average fraction of body weight
// alcohol distributes into. Real physiology varies with body composition, so
// callers must present the result as an estimate, never a measurement.
#[allow(dead_code)]
const WIDMARK_R_MALE: f64 = 0.68;
#[allow(dead_code)]
const WIDMARK_R_FEMALE: f64 = 0.55;
// Average hepatic elimination rate, in BAC percentage points per hour.
#[allow(dead_code)]
const ALCOHOL_ELIMINATION_PERCENT_PER_HOUR: f64 = 0.015;

// Average adult caffeine half-life; apparent volume of distribution per kg.
#[allow(dead_code)]
const CAFFEINE_HALF_LIFE_HOURS: f64 = 5.0;
#[allow(dead_code)]
const CAFFEINE_VOLUME_OF_DISTRIBUTION_L_PER_KG: f64 = 0.5;

/// "True time lost" per plan-v1.md is these two bands, always kept separate:
/// minutes actually spent doing the thing, and minutes the thing is estimated
/// to shorten your life by. Never silently merged into one number.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct TimeLoss {
    pub time_spent_minutes: f64,
    pub life_shortened_minutes: f64,
}

impl TimeLoss {
    pub fn total_minutes(&self) -> f64 {
        self.time_spent_minutes + self.life_shortened_minutes
    }

    pub fn zero() -> Self {
        Self::default()
    }

    pub fn add(self, other: Self) -> Self {
        Self {
            time_spent_minutes: self.time_spent_minutes + other.time_spent_minutes,
            life_shortened_minutes: self.life_shortened_minutes + other.life_shortened_minutes,
        }
    }
}

pub fn time_loss_for_entry(habit: &Habit, entry: &Entry) -> TimeLoss {
    TimeLoss {
        time_spent_minutes: entry.duration_minutes.unwrap_or(0.0),
        life_shortened_minutes: entry.quantity * habit.life_minutes_per_unit,
    }
}

pub fn money_spent_for_entry(habit: &Habit, entry: &Entry) -> f64 {
    entry.quantity * habit.cost_per_unit
}

/// Opportunity cost of the time actually spent (not the life-shortening
/// estimate — that time was never available to spend on anything else).
pub fn opportunity_cost_for_entry(habit: &Habit, entry: &Entry, profile: &Profile) -> Option<f64> {
    let net_hourly_income = profile.net_hourly_income?;
    let hours = time_loss_for_entry(habit, entry).time_spent_minutes / 60.0;
    Some(hours * net_hourly_income)
}

/// Share of *waking* life a quantity of lost minutes represents, over `days`.
/// Waking time, not calendar time, is the honest denominator: nobody was
/// going to spend their sleeping hours on this instead.
pub fn waking_life_share(total_minutes_lost: f64, days: f64, profile: &Profile) -> f64 {
    let waking_minutes = profile.waking_hours_per_day() * 60.0 * days;
    if waking_minutes <= 0.0 {
        return 0.0;
    }
    total_minutes_lost / waking_minutes
}

/// Years remaining against the profile's life-expectancy assumption, as of
/// `as_of`. `None` if birth date or the assumption itself is unset.
pub fn remaining_life_years(profile: &Profile, as_of: Date) -> Option<f64> {
    let birth_date = profile.birth_date?;
    let life_expectancy_years = profile.life_expectancy_years?;
    let age_years = (as_of - birth_date).whole_days() as f64 / 365.2425;
    Some((life_expectancy_years - age_years).max(0.0))
}

/// Expresses `life_shortened_minutes_per_year` as a share of the profile's
/// remaining *waking* years, itself re-expressed as months of that remaining
/// waking life — the "2.1 months of your remaining waking life" framing.
pub fn remaining_waking_life_months_at_rate(
    life_shortened_minutes_per_year: f64,
    profile: &Profile,
    as_of: Date,
) -> Option<f64> {
    let remaining_years = remaining_life_years(profile, as_of)?;
    let waking_minutes_per_year = profile.waking_hours_per_day() * 60.0 * 365.2425;
    if waking_minutes_per_year <= 0.0 {
        return Some(0.0);
    }
    let remaining_waking_minutes = remaining_years * waking_minutes_per_year;
    if remaining_waking_minutes <= 0.0 {
        return Some(0.0);
    }
    let share = life_shortened_minutes_per_year / waking_minutes_per_year;
    Some(share * remaining_years * 12.0)
}

/// Widmark equation: peak BAC from distribution, minus hepatic elimination
/// since the first drink. Returns an estimated BAC percentage (e.g. 0.08 for
/// 0.08%), floored at zero. This is not a substitute for a breathalyzer.
#[allow(dead_code)]
pub fn estimated_bac_percent(
    standard_drinks: f64,
    weight_kg: f64,
    sex: Sex,
    hours_since_first_drink: f64,
) -> f64 {
    let r = match sex {
        Sex::Male => WIDMARK_R_MALE,
        Sex::Female => WIDMARK_R_FEMALE,
    };
    let alcohol_grams = standard_drinks * STANDARD_DRINK_GRAMS_ALCOHOL;
    let weight_grams = weight_kg * 1000.0;
    if weight_grams <= 0.0 {
        return 0.0;
    }
    let peak_bac_percent = (alcohol_grams / (weight_grams * r)) * 100.0;
    let eliminated = ALCOHOL_ELIMINATION_PERCENT_PER_HOUR * hours_since_first_drink;
    (peak_bac_percent - eliminated).max(0.0)
}

/// First-order decay estimate of caffeine concentration, for surfacing "how
/// much is still in your system" rather than a diagnosed sleep-debt figure.
#[allow(dead_code)]
pub fn estimated_caffeine_mg_per_l(dose_mg: f64, weight_kg: f64, hours_elapsed: f64) -> f64 {
    let vd_l = CAFFEINE_VOLUME_OF_DISTRIBUTION_L_PER_KG * weight_kg;
    if vd_l <= 0.0 {
        return 0.0;
    }
    let initial_concentration = dose_mg / vd_l;
    initial_concentration * 0.5_f64.powf(hours_elapsed / CAFFEINE_HALF_LIFE_HOURS)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::habit::HabitId;
    use time::macros::datetime;

    fn habit() -> Habit {
        let mut h = Habit::new("Cigarettes", "cigarette");
        h.id = Some(HabitId(1));
        h.life_minutes_per_unit = DEFAULT_LIFE_MINUTES_PER_CIGARETTE;
        h.cost_per_unit = 0.60;
        h
    }

    #[test]
    fn time_loss_splits_spent_and_shortened() {
        let h = habit();
        let mut e = Entry::new(HabitId(1), datetime!(2026-01-01 08:00), 4.0);
        e.duration_minutes = Some(20.0);
        let loss = time_loss_for_entry(&h, &e);
        assert_eq!(loss.time_spent_minutes, 20.0);
        assert_eq!(loss.life_shortened_minutes, 44.0);
        assert_eq!(loss.total_minutes(), 64.0);
    }

    #[test]
    fn money_spent_scales_with_quantity() {
        let h = habit();
        let e = Entry::new(HabitId(1), datetime!(2026-01-01 08:00), 4.0);
        assert!((money_spent_for_entry(&h, &e) - 2.40).abs() < 1e-9);
    }

    #[test]
    fn waking_life_share_uses_waking_hours_not_calendar_hours() {
        let profile = Profile {
            typical_sleep_hours: Some(8.0),
            ..Default::default()
        };
        // 16 waking hours/day * 60 = 960 waking minutes/day, over 1 day.
        let share = waking_life_share(96.0, 1.0, &profile);
        assert!((share - 0.1).abs() < 1e-9);
    }

    #[test]
    fn waking_life_share_is_zero_with_no_waking_hours() {
        let profile = Profile {
            typical_sleep_hours: Some(24.0),
            ..Default::default()
        };
        assert_eq!(waking_life_share(100.0, 1.0, &profile), 0.0);
    }

    #[test]
    fn remaining_life_years_requires_birth_date_and_assumption() {
        let profile = Profile::default();
        assert_eq!(
            remaining_life_years(
                &profile,
                Date::from_calendar_date(2026, time::Month::January, 1).unwrap()
            ),
            None
        );
    }

    #[test]
    fn remaining_life_years_never_goes_negative() {
        let profile = Profile {
            birth_date: Some(Date::from_calendar_date(1900, time::Month::January, 1).unwrap()),
            life_expectancy_years: Some(80.0),
            ..Default::default()
        };
        let years = remaining_life_years(
            &profile,
            Date::from_calendar_date(2026, time::Month::January, 1).unwrap(),
        )
        .unwrap();
        assert_eq!(years, 0.0);
    }

    #[test]
    fn bac_rises_with_drinks_and_falls_with_time() {
        let bac_now = estimated_bac_percent(4.0, 70.0, Sex::Male, 0.0);
        let bac_later = estimated_bac_percent(4.0, 70.0, Sex::Male, 3.0);
        assert!(bac_now > bac_later);
        assert!(bac_now > 0.0);
    }

    #[test]
    fn bac_never_negative() {
        let bac = estimated_bac_percent(1.0, 70.0, Sex::Male, 100.0);
        assert_eq!(bac, 0.0);
    }

    #[test]
    fn bac_sex_affects_distribution_ratio() {
        let male = estimated_bac_percent(4.0, 70.0, Sex::Male, 0.0);
        let female = estimated_bac_percent(4.0, 70.0, Sex::Female, 0.0);
        assert!(female > male);
    }

    #[test]
    fn caffeine_concentration_decays_by_half_at_half_life() {
        let c0 = estimated_caffeine_mg_per_l(200.0, 70.0, 0.0);
        let c1 = estimated_caffeine_mg_per_l(200.0, 70.0, CAFFEINE_HALF_LIFE_HOURS);
        assert!((c1 - c0 / 2.0).abs() < 1e-9);
    }

    #[test]
    fn opportunity_cost_none_without_income() {
        let h = habit();
        let e = Entry::new(HabitId(1), datetime!(2026-01-01 08:00), 4.0);
        let profile = Profile::default();
        assert_eq!(opportunity_cost_for_entry(&h, &e, &profile), None);
    }

    #[test]
    fn opportunity_cost_scales_with_spent_time_only() {
        let h = habit();
        let mut e = Entry::new(HabitId(1), datetime!(2026-01-01 08:00), 4.0);
        e.duration_minutes = Some(30.0);
        let profile = Profile {
            net_hourly_income: Some(40.0),
            ..Default::default()
        };
        // life-shortened minutes must NOT enter opportunity cost.
        assert!((opportunity_cost_for_entry(&h, &e, &profile).unwrap() - 20.0).abs() < 1e-9);
    }
}
