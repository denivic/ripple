# Graph Report - ripple  (2026-10-21)

## Corpus Check
- 148 files · ~59,212 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 1329 nodes · 3066 edges · 77 communities (47 shown, 16 thin omitted)
- Extraction: 97% EXTRACTED · 3% INFERRED · 0% AMBIGUOUS · INFERRED: 95 edges (avg confidence: 0.85)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `f9951bb5`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- source.rs
- ipc/index.ts
- dto.rs
- charts/index.ts
- commands.rs
- HabitId
- grid/index.ts
- SqliteHabitRepository
- Apple Design
- @types/d3-array
- theme/index.ts
- convert_currency.rs
- mapping.rs
- cell_storage.rs
- tauri.conf.json
- today_summary.rs
- stores/index.ts
- ExportError
- devDependencies
- dependencies
- SvelteKit
- Ripple App Icon/Logo
- compilerOptions
- scripts
- permissions
- @sveltejs/adapter-static
- package.json
- @types/d3-shape
- repository.rs
- svelte.config.js
- vite
- svelte-check
- Tauri
- navigation.ts
- settings_repository.rs
- XlsxSource
- TypeScript
- Vite
- ssr
- Db
- award_points.rs
- models.ts
- protobuf.rs
- Sheet
- graphify
- ripple
- PointAward
- onDataChanged
- profile_repository.rs
- today/+page.svelte
- point_award_repository.rs
- habit-presets.ts
- HabitsStore
- IwaError
- .sheets
- Progress
- parse_archives
- .new
- decode_iwa_chunks
- Settings
- Habit
- ProgressStore
- README.md (Tauri+SvelteKit+TS template)

## God Nodes (most connected - your core abstractions)
1. `HabitId` - 52 edges
2. `Db` - 36 edges
3. `AppState` - 32 edges
4. `PointAward` - 24 edges
5. `candidate_awards()` - 23 edges
6. `IwaError` - 23 edges
7. `compute_today_summary()` - 22 edges
8. `daily_series()` - 21 edges
9. `LedgerStore` - 21 edges
10. `Apple Design` - 20 edges

## Surprising Connections (you probably didn't know these)
- `SvelteKit HTML Shell (app.html)` --implements--> `SvelteKit`  [INFERRED]
  src/app.html → README.md
- `clean_day_awards()` --calls--> `milestone_award()`  [INFERRED]
  src-tauri/src/application/award_points.rs → src-tauri/src/domain/points.rs
- `clean_day_awards()` --calls--> `daily_series()`  [INFERRED]
  src-tauri/src/application/award_points.rs → src-tauri/src/domain/projections.rs
- `clean_day_awards()` --calls--> `is_clean_day()`  [INFERRED]
  src-tauri/src/application/award_points.rs → src-tauri/src/domain/projections.rs
- `improved_week_awards()` --calls--> `milestone_award()`  [INFERRED]
  src-tauri/src/application/award_points.rs → src-tauri/src/domain/points.rs

## Import Cycles
- 2-file cycle: `src-tauri/src/infrastructure/db/mapping_repository.rs -> src-tauri/src/infrastructure/db/mod.rs -> src-tauri/src/infrastructure/db/mapping_repository.rs`
- 2-file cycle: `src-tauri/src/infrastructure/db/mod.rs -> src-tauri/src/infrastructure/db/point_award_repository.rs -> src-tauri/src/infrastructure/db/mod.rs`
- 4-file cycle: `src/lib/grid/ImportWizard.svelte -> src/lib/stores/index.ts -> src/lib/stores/ledger.svelte.ts -> src/lib/grid/index.ts -> src/lib/grid/ImportWizard.svelte`

## Communities (77 total, 16 thin omitted)

### Community 0 - "source.rs"
Cohesion: 0.22
Nodes (16): archive(), archive_info(), build_synthetic_table(), Encoder, pack_decimal128(), read_string_table(), read_table(), reads_a_single_number_cell_through_the_full_navigation_chain() (+8 more)

### Community 1 - "ipc/index.ts"
Cohesion: 0.14
Nodes (25): commands, ColumnMappingDto, CurrencyConversionDto, EntryDto, HabitDto, HabitMappingDto, HabitPresetDto, HabitWeekTrendDto (+17 more)

### Community 2 - "dto.rs"
Cohesion: 0.06
Nodes (61): DailyPoint, HabitBreakdownItem, HabitPreset, HabitWeekTrendDto, PeriodCompareResult, Option, CurrencyConversion, derived_values_show_their_arithmetic() (+53 more)

### Community 3 - "charts/index.ts"
Cohesion: 0.08
Nodes (27): horizontalBarPath(), verticalBarPath(), formatCalendarDate(), formatDayTick, formatDuration(), formatHourTick, formatMillisecond, formatMinuteTick (+19 more)

### Community 4 - "commands.rs"
Cohesion: 0.12
Nodes (56): AppHandle, CurrencyConversionDto, EntryDto, HabitDto, HabitPresetDto, ImportPreviewDto, ImportSummaryDto, ProfileDto (+48 more)

### Community 5 - "HabitId"
Cohesion: 0.06
Nodes (74): compute_timeline(), compute_timeline_includes_entries_from_archived_habits(), compute_timeline_zero_fills_and_compares_against_previous_equal_length_period(), DailyPoint, HabitBreakdownItem, Date, RepoResult, Vec (+66 more)

### Community 6 - "grid/index.ts"
Cohesion: 0.06
Nodes (36): formatCell(), HabitLookup, pad(), parseCell(), parseDateCell(), parseNumberCell(), ParseResult, habits (+28 more)

### Community 7 - "SqliteHabitRepository"
Cohesion: 0.22
Nodes (14): insert_then_get_round_trips(), list_excludes_archived_by_default(), repo(), row_to_habit(), Arc, Habit, Option, RepoResult (+6 more)

### Community 8 - "Apple Design"
Cohesion: 0.10
Nodes (20): 10. Gesture design details (the "feel" checklist), 11. Frame-level smoothness, 12. Materials & depth — translucency conveys hierarchy, 13. Multimodal feedback — motion + sound + haptics, 14. Reduced motion & accessibility, 15. Typography — optical sizing, tracking, leading, 16. Design foundations — the eight principles, 17. Process (+12 more)

### Community 10 - "theme/index.ts"
Cohesion: 0.10
Nodes (24): SettingsStore, AccessibilityPreferences, applyAccessibilityOverrides(), NO_ACCESSIBILITY_OVERRIDES, ACCESSIBILITY_QUERIES, applyTheme(), readAccessibilityPreferences(), themeToCssVars() (+16 more)

### Community 11 - "convert_currency.rs"
Cohesion: 0.11
Nodes (26): Agent, a_rate_error_changes_nothing(), applies_the_fetched_rate_to_habits_and_profile(), convert_currency(), FakeRates, fetch_rate(), malformed_codes_are_rejected_before_any_fetch(), network_errors_read_clearly() (+18 more)

### Community 12 - "mapping.rs"
Cohesion: 0.08
Nodes (50): Box, apply_import(), column_habit_mapping_creates_habits_by_name(), detect_source(), ImportPreview, imports_a_csv_with_a_fixed_habit_and_remembers_the_mapping(), ImportSummary, preview_import() (+42 more)

### Community 13 - "cell_storage.rs"
Cohesion: 0.19
Nodes (23): cell_kind(), CellKind, decode_cell(), DecodedCell, decodes_a_bool_cell(), decodes_a_date_cell_relative_to_the_2001_epoch(), decodes_a_duration_cell_as_minutes(), decodes_a_negative_fractional_number() (+15 more)

### Community 14 - "tauri.conf.json"
Cohesion: 0.09
Nodes (22): icons/128x128@2x.png, icons/128x128.png, icons/32x32.png, icons/icon.icns, icons/icon.ico, app, security, windows (+14 more)

### Community 15 - "today_summary.rs"
Cohesion: 0.12
Nodes (35): day_range(), Date, PrimitiveDateTime, a_full_thirty_days_of_history_averages_over_thirty_days(), a_net_gain_projects_negative_months_lost(), a_short_history_is_averaged_over_the_days_since_the_first_entry(), a_single_first_entry_is_averaged_over_the_seven_day_floor(), an_empty_window_projects_zero_months() (+27 more)

### Community 16 - "stores/index.ts"
Cohesion: 0.13
Nodes (9): i(), BOUNCE_DAMPING, SpringOptions, toSpringOptions(), active, announce(), await(), breakdownValueLabel() (+1 more)

### Community 17 - "ExportError"
Cohesion: 0.13
Nodes (17): Path, Result, write_csv(), JsonRow, Option, Path, Result, write_json() (+9 more)

### Community 18 - "devDependencies"
Cohesion: 0.11
Nodes (19): devDependencies, svelte, @sveltejs/kit, @sveltejs/vite-plugin-svelte, @tauri-apps/cli, @types/d3-scale, @types/d3-time, @types/d3-time-format (+11 more)

### Community 19 - "dependencies"
Cohesion: 0.12
Nodes (17): d3-time-format, dependencies, d3-array, d3-scale, d3-shape, d3-time, d3-time-format, @tauri-apps/api (+9 more)

### Community 22 - "compilerOptions"
Cohesion: 0.15
Nodes (12): ./.svelte-kit/tsconfig.json, compilerOptions, allowJs, checkJs, esModuleInterop, forceConsistentCasingInFileNames, moduleResolution, resolveJsonModule (+4 more)

### Community 23 - "scripts"
Cohesion: 0.22
Nodes (9): scripts, build, check, check:watch, dev, preview, tauri, test (+1 more)

### Community 24 - "permissions"
Cohesion: 0.17
Nodes (11): core:default, dialog:allow-message, dialog:allow-open, dialog:allow-save, main, opener:default, description, identifier (+3 more)

### Community 26 - "package.json"
Cohesion: 0.33
Nodes (5): description, license, name, type, version

### Community 28 - "repository.rs"
Cohesion: 0.19
Nodes (14): evaluate_awards(), get_progress(), RepoResult, export_entries(), export_resolves_habit_names_and_counts_rows(), ExportFormat, Path, Result (+6 more)

### Community 39 - "navigation.ts"
Cohesion: 0.29
Nodes (6): CaretState, CellPosition, GridBounds, nextCell(), bounds, midCaret

### Community 40 - "settings_repository.rs"
Cohesion: 0.17
Nodes (19): StartupRoute, WeightUnit, get_before_any_save_returns_default(), malformed_stored_currency_falls_back_to_default(), repo(), Arc, Option, RepoResult (+11 more)

### Community 41 - "XlsxSource"
Cohesion: 0.18
Nodes (10): Data, cell_from_calamine(), AsRef, CellValue, Path, PathBuf, Result, Self (+2 more)

### Community 45 - "Db"
Cohesion: 0.06
Nodes (62): Mutex, MutexGuard, EntryId, a_failure_mid_transaction_rolls_everything_back(), archived_habits_are_converted_too(), converts_habits_profile_and_currency_together(), costs(), Fixture (+54 more)

### Community 48 - "award_points.rs"
Cohesion: 0.24
Nodes (24): a_broken_streak_restarts_the_bonus_count(), candidate_awards(), clean_day_awards(), completed_days_without_harmful_entries_are_clean(), day_key(), each_logged_day_earns_five_including_today(), first_entry_milestone_is_dated_to_the_first_entry(), good_habit_improves_by_going_up() (+16 more)

### Community 49 - "models.ts"
Cohesion: 0.12
Nodes (21): formatLocalDate(), formatLocalDateTime(), pad(), parseLocalDate(), parseLocalDateTime(), DailyPointDto, HabitBreakdownItemDto, PeriodCompareDto (+13 more)

### Community 50 - "protobuf.rs"
Cohesion: 0.17
Nodes (19): Item, Iterator, all(), decodes_a_real_archive_info(), Field, first(), first_and_all_find_by_field_number(), parse_fields() (+11 more)

### Community 51 - "Sheet"
Cohesion: 0.15
Nodes (15): CellValue, Error, Option, PrimitiveDateTime, String, Vec, Sheet, TabularSource (+7 more)

### Community 54 - "PointAward"
Cohesion: 0.17
Nodes (17): Params, milestone_date(), AwardKind, Milestone, milestone_award(), milestone_award_key(), PointAward, Date (+9 more)

### Community 55 - "onDataChanged"
Cohesion: 0.24
Nodes (4): onDataChanged(), Timeline, DEFAULT_SETTINGS, TimelineStore

### Community 56 - "profile_repository.rs"
Cohesion: 0.15
Nodes (17): Profile, Date, Option, Sex, ProfileRepository, get_before_any_save_returns_default(), repo(), Arc (+9 more)

### Community 57 - "today/+page.svelte"
Cohesion: 0.10
Nodes (16): addCustomHabit(), addHabitOpen, closeAddHabit(), handlePresetKeydown(), newHabitCost, newHabitLifeMinutes, newHabitMinutesPerUnit, newHabitName (+8 more)

### Community 58 - "point_award_repository.rs"
Cohesion: 0.18
Nodes (15): RawAward, clean(), insert_new_skips_keys_already_in_the_ledger(), kind_to_str(), now_utc(), recent_is_newest_first_and_round_trips_fields(), repo(), row_to_raw() (+7 more)

### Community 59 - "habit-presets.ts"
Cohesion: 0.27
Nodes (9): categoryLabel(), filterPresets(), formatPresetSource(), groupPresets(), PRESET_CATEGORIES, PresetGroup, presets, PresetCategory (+1 more)

### Community 60 - "HabitsStore"
Cohesion: 0.29
Nodes (4): Habit, habitFromDto(), habitToDto(), HabitsStore

### Community 61 - "IwaError"
Cohesion: 0.26
Nodes (12): Bundle, is_index_iwa(), Path, PathBuf, Result, Self, Vec, walk_iwa_files() (+4 more)

### Community 62 - ".sheets"
Cohesion: 0.20
Nodes (9): CsvSource, infer_cell(), AsRef, CellValue, Path, PathBuf, Result, Self (+1 more)

### Community 63 - "Progress"
Cohesion: 0.29
Nodes (9): MilestoneStatus, Progress, Option, level_for(), LevelProgress, progress_within_a_level(), Option, thresholds_map_to_their_level() (+1 more)

### Community 64 - "parse_archives"
Cohesion: 0.33
Nodes (8): ArchiveMessage, parse_archives(), parses_a_real_archive_stream(), parses_consecutive_archives(), real_archive_bytes(), Result, Vec, HashMap

### Community 65 - ".new"
Cohesion: 0.44
Nodes (7): entry(), evaluating_twice_awards_nothing_new(), Fixture, later_edits_never_take_points_back(), milestones_list_achieved_and_locked_and_respect_week_start_setting(), reaching_a_thousand_points_unlocks_the_badge(), Self

### Community 66 - "decode_iwa_chunks"
Cohesion: 0.42
Nodes (8): decode_iwa_chunks(), decodes_a_single_chunk(), decodes_multiple_back_to_back_chunks(), rejects_a_nonzero_chunk_type(), rejects_truncated_input(), Result, Vec, snappy_literal_chunk()

### Community 67 - "Settings"
Cohesion: 0.33
Nodes (5): Default, Option, Self, String, Settings

### Community 68 - "Habit"
Cohesion: 0.40
Nodes (5): Into, Habit, Option, Self, String

## Knowledge Gaps
- **141 isolated node(s):** `name`, `version`, `description`, `type`, `dev` (+136 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 337 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **16 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Db` connect `Db` to `commands.rs`, `SqliteHabitRepository`, `settings_repository.rs`, `convert_currency.rs`, `mapping.rs`, `profile_repository.rs`, `point_award_repository.rs`?**
  _High betweenness centrality (0.295) - this node is a cross-community bridge._
- **Why does `Sheet` connect `Sheet` to `source.rs`, `dto.rs`, `XlsxSource`, `mapping.rs`, `.sheets`?**
  _High betweenness centrality (0.146) - this node is a cross-community bridge._
- **Why does `read_table()` connect `source.rs` to `parse_archives`, `cell_storage.rs`, `protobuf.rs`, `Sheet`, `IwaError`?**
  _High betweenness centrality (0.094) - this node is a cross-community bridge._
- **Are the 10 inferred relationships involving `HabitId` (e.g. with `explicit_duration_overrides_minutes_per_unit()` and `habit()`) actually correct?**
  _`HabitId` has 10 INFERRED edges - model-reasoned connections that need verification._
- **What connects `name`, `version`, `description` to the rest of the system?**
  _141 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `ipc/index.ts` be split into smaller, more focused modules?**
  _Cohesion score 0.13911290322580644 - nodes in this community are weakly interconnected._
- **Should `dto.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.05555555555555555 - nodes in this community are weakly interconnected._