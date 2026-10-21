//! Cited starting points for the "add habit" flow. Every value is a
//! population-average estimate the user can edit, not a clinical claim.
//! The table was researched offline and is compiled in, so presets work
//! without a network and can't drift from what was checked.

/// What kind of habit a preset describes; the add-habit picker groups by it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PresetCategory {
    Tobacco,
    Alcohol,
    Cannabis,
    Food,
    Drink,
    Sedentary,
    /// Habits estimated to *add* life (negative `life_minutes_per_unit`).
    Gain,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PresetConfidence {
    High,
    Medium,
    Low,
    /// No credible per-unit figure exists. These presets carry a value of 0
    /// and a note asking the user to set their own, rather than an invented
    /// number.
    Insufficient,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HabitPreset {
    pub id: &'static str,
    pub name: &'static str,
    pub unit_label: &'static str,
    /// Positive means life lost per unit; negative means life gained.
    pub life_minutes_per_unit: f64,
    /// Typical minutes one unit takes, for time-spent tracking. Unlike the
    /// life figure this is not a research value, just an editable default;
    /// 0 means the habit isn't time-tracked.
    pub minutes_per_unit: f64,
    pub category: PresetCategory,
    /// Absent only for `Insufficient` presets where no source exists at all.
    pub source_title: Option<&'static str>,
    pub source_url: Option<&'static str>,
    pub year: Option<u16>,
    pub confidence: PresetConfidence,
    /// True when the value is computed from the source (e.g. microlives ×
    /// 30 min) rather than quoted; `derivation` then shows the arithmetic.
    pub derived: bool,
    pub derivation: Option<&'static str>,
    pub note: &'static str,
}

pub fn habit_presets() -> &'static [HabitPreset] {
    HABIT_PRESETS
}

static HABIT_PRESETS: &[HabitPreset] = &[
    HabitPreset {
        id: "cigarette",
        name: "Cigarettes",
        unit_label: "cigarette",
        life_minutes_per_unit: 20.0,
        minutes_per_unit: 5.0,
        category: PresetCategory::Tobacco,
        source_title: Some("The price of a cigarette: 20 minutes of life? (Jackson, Jarvis & West, Addiction 2025;120(5):810-812)"),
        source_url: Some("https://doi.org/10.1111/add.16757"),
        year: Some(2025),
        confidence: PresetConfidence::High,
        derived: false,
        derivation: None,
        note: "About 20 minutes of life expectancy lost per cigarette on average (17 for men, 22 for women).",
    },
    HabitPreset {
        id: "alcohol-drink-10g",
        name: "Alcoholic drinks (10 g)",
        unit_label: "drink (10 g alcohol)",
        life_minutes_per_unit: 15.0,
        minutes_per_unit: 0.0,
        category: PresetCategory::Alcohol,
        source_title: Some("Using speed of ageing and 'microlives' to communicate the effects of lifetime habits and environment (Spiegelhalter, BMJ 2012;345:e8223, Table 1)"),
        source_url: Some("https://doi.org/10.1136/bmj.e8223"),
        year: Some(2012),
        confidence: PresetConfidence::Low,
        derived: true,
        derivation: Some("Table 1: each subsequent drink of 10 g alcohol (up to 6/day) = -1/2 microlife/day for men and women; 0.5 x 30 min = 15 min."),
        note: "Each drink beyond the first of the day; the 2012 table credited the first drink with a small gain, which newer evidence disputes.",
    },
    HabitPreset {
        id: "alcohol-us-standard-drink",
        name: "Alcoholic drinks (US standard)",
        unit_label: "US standard drink (14 g)",
        life_minutes_per_unit: 21.0,
        minutes_per_unit: 0.0,
        category: PresetCategory::Alcohol,
        source_title: Some("Using speed of ageing and 'microlives' to communicate the effects of lifetime habits and environment (Spiegelhalter, BMJ 2012;345:e8223, Table 1)"),
        source_url: Some("https://doi.org/10.1136/bmj.e8223"),
        year: Some(2012),
        confidence: PresetConfidence::Low,
        derived: true,
        derivation: Some("15 min per 10 g drink (Spiegelhalter Table 1, -1/2 microlife) scaled linearly to a 14 g US standard drink: 15 x 14/10 = 21 min."),
        note: "Scaled from the 10 g estimate, assuming harm is proportional to grams of alcohol.",
    },
    HabitPreset {
        id: "red-meat-portion",
        name: "Red meat",
        unit_label: "portion (85 g)",
        life_minutes_per_unit: 30.0,
        minutes_per_unit: 0.0,
        category: PresetCategory::Food,
        source_title: Some("Using speed of ageing and 'microlives' to communicate the effects of lifetime habits and environment (Spiegelhalter, BMJ 2012;345:e8223, Table 1)"),
        source_url: Some("https://doi.org/10.1136/bmj.e8223"),
        year: Some(2012),
        confidence: PresetConfidence::Low,
        derived: true,
        derivation: Some("Table 1: red meat, 1 portion (85 g, 3 oz) per day = -1 microlife; 1 x 30 min = 30 min."),
        note: "Averaged over a lifetime habit of one 85 g portion a day.",
    },
    HabitPreset {
        id: "tv-2h",
        name: "Watching TV",
        unit_label: "2 hours of TV",
        life_minutes_per_unit: 30.0,
        minutes_per_unit: 120.0,
        category: PresetCategory::Sedentary,
        source_title: Some("Using speed of ageing and 'microlives' to communicate the effects of lifetime habits and environment (Spiegelhalter, BMJ 2012;345:e8223, Table 1)"),
        source_url: Some("https://doi.org/10.1136/bmj.e8223"),
        year: Some(2012),
        confidence: PresetConfidence::Low,
        derived: true,
        derivation: Some("Table 1: 2 hours watching television per day = -1 microlife; 1 x 30 min = 30 min."),
        note: "Stands in for sedentary sitting; based on a lifetime daily habit.",
    },
    HabitPreset {
        id: "hot-dog",
        name: "Hot dogs",
        unit_label: "hot dog in bun (140 g)",
        life_minutes_per_unit: 36.3,
        minutes_per_unit: 0.0,
        category: PresetCategory::Food,
        source_title: Some("Small targeted dietary changes can yield substantial gains for human and environmental health (Stylianou, Fulgoni & Jolliet, Nature Food 2021;2:616-627, Supplementary Data 1, HENI per serving)"),
        source_url: Some("https://doi.org/10.1038/s43016-021-00343-4"),
        year: Some(2021),
        confidence: PresetConfidence::Medium,
        derived: false,
        derivation: None,
        note: "Beef hot dog on a white bun; minutes of healthy life (not just lifespan).",
    },
    HabitPreset {
        id: "bacon",
        name: "Bacon",
        unit_label: "serving (15 g, ~2 slices)",
        life_minutes_per_unit: 6.44,
        minutes_per_unit: 0.0,
        category: PresetCategory::Food,
        source_title: Some("Small targeted dietary changes can yield substantial gains for human and environmental health (Stylianou, Fulgoni & Jolliet, Nature Food 2021;2:616-627, Supplementary Data 1, HENI per serving)"),
        source_url: Some("https://doi.org/10.1038/s43016-021-00343-4"),
        year: Some(2021),
        confidence: PresetConfidence::Medium,
        derived: false,
        derivation: None,
        note: "Cooked pork bacon; minutes of healthy life.",
    },
    HabitPreset {
        id: "deli-ham",
        name: "Deli ham",
        unit_label: "serving (55 g)",
        life_minutes_per_unit: 28.52,
        minutes_per_unit: 0.0,
        category: PresetCategory::Food,
        source_title: Some("Small targeted dietary changes can yield substantial gains for human and environmental health (Stylianou, Fulgoni & Jolliet, Nature Food 2021;2:616-627, Supplementary Data 1, HENI per serving)"),
        source_url: Some("https://doi.org/10.1038/s43016-021-00343-4"),
        year: Some(2021),
        confidence: PresetConfidence::Medium,
        derived: false,
        derivation: None,
        note: "Packaged or deli ham; processed meat dominates the score.",
    },
    HabitPreset {
        id: "pork-sausage",
        name: "Pork sausage",
        unit_label: "serving (55 g)",
        life_minutes_per_unit: 22.15,
        minutes_per_unit: 0.0,
        category: PresetCategory::Food,
        source_title: Some("Small targeted dietary changes can yield substantial gains for human and environmental health (Stylianou, Fulgoni & Jolliet, Nature Food 2021;2:616-627, Supplementary Data 1, HENI per serving)"),
        source_url: Some("https://doi.org/10.1038/s43016-021-00343-4"),
        year: Some(2021),
        confidence: PresetConfidence::Medium,
        derived: false,
        derivation: None,
        note: "Pork sausage; processed meat dominates the score.",
    },
    HabitPreset {
        id: "cheeseburger",
        name: "Cheeseburgers",
        unit_label: "fast-food cheeseburger (140 g)",
        life_minutes_per_unit: 6.26,
        minutes_per_unit: 0.0,
        category: PresetCategory::Food,
        source_title: Some("Small targeted dietary changes can yield substantial gains for human and environmental health (Stylianou, Fulgoni & Jolliet, Nature Food 2021;2:616-627, Supplementary Data 1, HENI per serving)"),
        source_url: Some("https://doi.org/10.1038/s43016-021-00343-4"),
        year: Some(2021),
        confidence: PresetConfidence::Medium,
        derived: false,
        derivation: None,
        note: "Single-patty fast-food cheeseburger; minutes of healthy life.",
    },
    HabitPreset {
        id: "pepperoni-pizza",
        name: "Pepperoni pizza",
        unit_label: "slice (140 g)",
        life_minutes_per_unit: 5.32,
        minutes_per_unit: 0.0,
        category: PresetCategory::Food,
        source_title: Some("Small targeted dietary changes can yield substantial gains for human and environmental health (Stylianou, Fulgoni & Jolliet, Nature Food 2021;2:616-627, Supplementary Data 1, HENI per serving)"),
        source_url: Some("https://doi.org/10.1038/s43016-021-00343-4"),
        year: Some(2021),
        confidence: PresetConfidence::Medium,
        derived: false,
        derivation: None,
        note: "Restaurant pepperoni pizza, medium crust; minutes of healthy life.",
    },
    HabitPreset {
        id: "sugary-soda",
        name: "Sugary soda",
        unit_label: "can (12 oz / 360 g)",
        life_minutes_per_unit: 12.48,
        minutes_per_unit: 0.0,
        category: PresetCategory::Drink,
        source_title: Some("Small targeted dietary changes can yield substantial gains for human and environmental health (Stylianou, Fulgoni & Jolliet, Nature Food 2021;2:616-627, Supplementary Data 1, HENI per serving)"),
        source_url: Some("https://doi.org/10.1038/s43016-021-00343-4"),
        year: Some(2021),
        confidence: PresetConfidence::Medium,
        derived: false,
        derivation: None,
        note: "Sugar-sweetened cola; diet versions score near zero in the same study.",
    },
    HabitPreset {
        id: "energy-drink",
        name: "Energy drinks",
        unit_label: "can",
        life_minutes_per_unit: 0.0,
        minutes_per_unit: 0.0,
        category: PresetCategory::Drink,
        source_title: None,
        source_url: None,
        year: None,
        confidence: PresetConfidence::Insufficient,
        derived: false,
        derivation: None,
        note: "No per-can estimate published; the sugar-sweetened soda value is a rough guide for sugary versions.",
    },
    HabitPreset {
        id: "moderate-exercise-20min",
        name: "Moderate exercise",
        unit_label: "20 minutes (first of the day)",
        life_minutes_per_unit: -60.0,
        minutes_per_unit: 20.0,
        category: PresetCategory::Gain,
        source_title: Some("Using speed of ageing and 'microlives' to communicate the effects of lifetime habits and environment (Spiegelhalter, BMJ 2012;345:e8223, Table 1)"),
        source_url: Some("https://doi.org/10.1136/bmj.e8223"),
        year: Some(2012),
        confidence: PresetConfidence::Medium,
        derived: true,
        derivation: Some("Table 1: first 20 minutes of moderate exercise per day = +2 microlives (men and women); -2 x 30 min = -60 min (gain)."),
        note: "Life gained by the first 20 minutes of moderate exercise in a day; extra minutes add much less.",
    },
    HabitPreset {
        id: "fruit-veg-5-a-day",
        name: "Fruit & vegetables (5 a day)",
        unit_label: "day with 5+ servings",
        life_minutes_per_unit: -120.0,
        minutes_per_unit: 0.0,
        category: PresetCategory::Gain,
        source_title: Some("Using speed of ageing and 'microlives' to communicate the effects of lifetime habits and environment (Spiegelhalter, BMJ 2012;345:e8223, Table 1)"),
        source_url: Some("https://doi.org/10.1136/bmj.e8223"),
        year: Some(2012),
        confidence: PresetConfidence::Low,
        derived: true,
        derivation: Some("Table 1: fruit and vegetable intake >=5 servings/day = +4 microlives (men and women); -4 x 30 min = -120 min (gain)."),
        note: "Per day of eating at least five servings; based on blood vitamin C as a marker, so treat as rough.",
    },
    HabitPreset {
        id: "coffee-2-3-cups",
        name: "Coffee (2-3 cups)",
        unit_label: "day with 2-3 cups",
        life_minutes_per_unit: -30.0,
        minutes_per_unit: 0.0,
        category: PresetCategory::Gain,
        source_title: Some("Using speed of ageing and 'microlives' to communicate the effects of lifetime habits and environment (Spiegelhalter, BMJ 2012;345:e8223, Table 1)"),
        source_url: Some("https://doi.org/10.1136/bmj.e8223"),
        year: Some(2012),
        confidence: PresetConfidence::Low,
        derived: true,
        derivation: Some("Table 1: coffee intake 2-3 cups/day = +1 microlife; -1 x 30 min = -30 min (gain)."),
        note: "An association from observational studies, not proof that coffee extends life.",
    },
    HabitPreset {
        id: "nuts",
        name: "Nuts",
        unit_label: "handful (30 g peanuts)",
        life_minutes_per_unit: -25.5,
        minutes_per_unit: 0.0,
        category: PresetCategory::Gain,
        source_title: Some("Small targeted dietary changes can yield substantial gains for human and environmental health (Stylianou, Fulgoni & Jolliet, Nature Food 2021;2:616-627, Supplementary Data 1, HENI per serving)"),
        source_url: Some("https://doi.org/10.1038/s43016-021-00343-4"),
        year: Some(2021),
        confidence: PresetConfidence::Medium,
        derived: false,
        derivation: None,
        note: "Salted dry-roasted peanuts; other nuts score similarly (pistachios +25.3).",
    },
    HabitPreset {
        id: "fruit-piece",
        name: "Fruit",
        unit_label: "piece of fruit (140 g apple)",
        life_minutes_per_unit: -13.46,
        minutes_per_unit: 0.0,
        category: PresetCategory::Gain,
        source_title: Some("Small targeted dietary changes can yield substantial gains for human and environmental health (Stylianou, Fulgoni & Jolliet, Nature Food 2021;2:616-627, Supplementary Data 1, HENI per serving)"),
        source_url: Some("https://doi.org/10.1038/s43016-021-00343-4"),
        year: Some(2021),
        confidence: PresetConfidence::Medium,
        derived: false,
        derivation: None,
        note: "Raw apple; a banana scores almost the same (+13.5).",
    },
    HabitPreset {
        id: "salmon",
        name: "Salmon",
        unit_label: "serving (85 g)",
        life_minutes_per_unit: -15.98,
        minutes_per_unit: 0.0,
        category: PresetCategory::Gain,
        source_title: Some("Small targeted dietary changes can yield substantial gains for human and environmental health (Stylianou, Fulgoni & Jolliet, Nature Food 2021;2:616-627, Supplementary Data 1, HENI per serving)"),
        source_url: Some("https://doi.org/10.1038/s43016-021-00343-4"),
        year: Some(2021),
        confidence: PresetConfidence::Medium,
        derived: false,
        derivation: None,
        note: "Baked or broiled salmon; omega-3 fats drive the gain.",
    },
    HabitPreset {
        id: "beans",
        name: "Beans",
        unit_label: "serving (90 g cooked)",
        life_minutes_per_unit: -10.42,
        minutes_per_unit: 0.0,
        category: PresetCategory::Gain,
        source_title: Some("Small targeted dietary changes can yield substantial gains for human and environmental health (Stylianou, Fulgoni & Jolliet, Nature Food 2021;2:616-627, Supplementary Data 1, HENI per serving)"),
        source_url: Some("https://doi.org/10.1038/s43016-021-00343-4"),
        year: Some(2021),
        confidence: PresetConfidence::Medium,
        derived: false,
        derivation: None,
        note: "Cooked dry beans without added fat.",
    },
    HabitPreset {
        id: "cigar",
        name: "Cigars",
        unit_label: "cigar",
        life_minutes_per_unit: 0.0,
        minutes_per_unit: 45.0,
        category: PresetCategory::Tobacco,
        source_title: None,
        source_url: None,
        year: None,
        confidence: PresetConfidence::Insufficient,
        derived: false,
        derivation: None,
        note: "Cigars carry real health risks, but no study gives a per-cigar figure; set your own value.",
    },
    HabitPreset {
        id: "pipe",
        name: "Pipe tobacco",
        unit_label: "bowl",
        life_minutes_per_unit: 0.0,
        minutes_per_unit: 20.0,
        category: PresetCategory::Tobacco,
        source_title: None,
        source_url: None,
        year: None,
        confidence: PresetConfidence::Insufficient,
        derived: false,
        derivation: None,
        note: "No per-bowl life-expectancy estimate exists; set your own value.",
    },
    HabitPreset {
        id: "vape",
        name: "Vaping / e-cigarettes",
        unit_label: "session",
        life_minutes_per_unit: 0.0,
        minutes_per_unit: 5.0,
        category: PresetCategory::Tobacco,
        source_title: Some("A Magic Bullet? The Potential Impact of E-Cigarettes on the Toll of Cigarette Smoking (Mendez & Warner, Nicotine Tob Res 2021;23(4):654-661) - population model only, no per-use figure"),
        source_url: Some("https://doi.org/10.1093/ntr/ntaa160"),
        year: Some(2021),
        confidence: PresetConfidence::Insufficient,
        derived: false,
        derivation: None,
        note: "Long-term data on vaping are too new for a per-use estimate; set your own value.",
    },
    HabitPreset {
        id: "cannabis-joint",
        name: "Cannabis joints",
        unit_label: "joint",
        life_minutes_per_unit: 0.0,
        minutes_per_unit: 10.0,
        category: PresetCategory::Cannabis,
        source_title: None,
        source_url: None,
        year: None,
        confidence: PresetConfidence::Insufficient,
        derived: false,
        derivation: None,
        note: "No credible per-joint life-expectancy estimate exists; studies of heavy use are confounded. Set your own value.",
    },
];

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn ids_are_unique() {
        let mut seen = HashSet::new();
        for p in habit_presets() {
            assert!(seen.insert(p.id), "duplicate preset id {}", p.id);
        }
    }

    #[test]
    fn every_value_is_finite() {
        for p in habit_presets() {
            assert!(p.life_minutes_per_unit.is_finite(), "{}", p.id);
        }
    }

    #[test]
    fn every_estimate_cites_an_https_source() {
        for p in habit_presets() {
            if p.confidence != PresetConfidence::Insufficient {
                assert!(
                    p.source_title.is_some_and(|t| !t.trim().is_empty()),
                    "{} has no source title",
                    p.id
                );
                assert!(p.source_url.is_some(), "{} has no source url", p.id);
                assert!(p.year.is_some(), "{} has no year", p.id);
            }
            if let Some(url) = p.source_url {
                assert!(url.starts_with("https://"), "{}: {url}", p.id);
            }
            if let Some(title) = p.source_title {
                assert!(!title.trim().is_empty(), "{}", p.id);
            }
        }
    }

    #[test]
    fn minutes_per_unit_are_finite_and_non_negative() {
        for p in habit_presets() {
            assert!(
                p.minutes_per_unit.is_finite() && p.minutes_per_unit >= 0.0,
                "{}",
                p.id
            );
        }
        let by_id = |id: &str| habit_presets().iter().find(|p| p.id == id).unwrap();
        assert_eq!(by_id("cigarette").minutes_per_unit, 5.0);
        assert_eq!(by_id("tv-2h").minutes_per_unit, 120.0);
        assert_eq!(by_id("coffee-2-3-cups").minutes_per_unit, 0.0);
    }

    #[test]
    fn insufficient_presets_are_zero_with_a_note() {
        for p in habit_presets() {
            if p.confidence == PresetConfidence::Insufficient {
                assert_eq!(p.life_minutes_per_unit, 0.0, "{}", p.id);
                assert!(!p.note.trim().is_empty(), "{}", p.id);
            }
        }
    }

    #[test]
    fn derived_values_show_their_arithmetic() {
        for p in habit_presets() {
            assert_eq!(p.derived, p.derivation.is_some(), "{}", p.id);
        }
    }

    #[test]
    fn gain_presets_are_negative_and_others_are_not() {
        for p in habit_presets() {
            if p.category == PresetCategory::Gain {
                assert!(p.life_minutes_per_unit < 0.0, "{}", p.id);
            } else {
                assert!(p.life_minutes_per_unit >= 0.0, "{}", p.id);
            }
        }
    }
}
