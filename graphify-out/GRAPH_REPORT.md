# Graph Report - ripple  (2026-10-21)

## Corpus Check
- 167 files · ~78,737 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 1806 nodes · 4399 edges · 86 communities (56 shown, 16 thin omitted)
- Extraction: 97% EXTRACTED · 3% INFERRED · 0% AMBIGUOUS · INFERRED: 127 edges (avg confidence: 0.85)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `68b9b552`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- IwaError
- ipc/index.ts
- dto.rs
- charts/index.ts
- commands.rs
- projections.rs
- grid/index.ts
- domain/notifications.rs
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
- settings_repository.rs
- svelte.config.js
- vite
- svelte-check
- Tauri
- application/email.rs
- map_row
- XlsxSource
- TypeScript
- Vite
- ssr
- SqliteHabitRepository
- award_points.rs
- models.ts
- application/notifications.rs
- Sheet
- graphify
- ripple
- ledger.svelte.ts
- profile_repository.rs
- today/+page.svelte
- habit-presets.ts
- HabitsStore
- http_api.rs
- .sheets
- templates.rs
- HabitId
- SqliteEntryRepository
- migrations.rs
- progress.svelte.ts
- SecretError
- README.md (Tauri+SvelteKit+TS template)
- import_workbook.rs
- .aged_28
- compute_timeline
- email-settings.ts
- tray.rs
- ColumnMapping
- TimeLoss
- Db
- currency_repository.rs
- RepositoryError
- spawn
- time-field.ts

## God Nodes (most connected - your core abstractions)
1. `HabitId` - 58 edges
2. `Db` - 51 edges
3. `AppState` - 40 edges
4. `PointAward` - 33 edges
5. `tick()` - 28 edges
6. `compute_today_summary()` - 24 edges
7. `EmailSettings` - 24 edges
8. `candidate_awards()` - 23 edges
9. `IwaError` - 23 edges
10. `daily_series()` - 22 edges

## Surprising Connections (you probably didn't know these)
- `reminder_notice()` --calls--> `reminder_needed()`  [INFERRED]
  src-tauri/src/application/notifications.rs → src-tauri/src/domain/notifications.rs
- `SvelteKit HTML Shell (app.html)` --implements--> `SvelteKit`  [INFERRED]
  src/app.html → README.md
- `clean_day_awards()` --calls--> `daily_series()`  [INFERRED]
  src-tauri/src/application/award_points.rs → src-tauri/src/domain/projections.rs
- `clean_day_awards()` --calls--> `is_clean_day()`  [INFERRED]
  src-tauri/src/application/award_points.rs → src-tauri/src/domain/projections.rs
- `compute_timeline()` --calls--> `day_range()`  [INFERRED]
  src-tauri/src/application/compute_series.rs → src-tauri/src/application/mod.rs

## Import Cycles
- 2-file cycle: `src-tauri/src/infrastructure/db/mapping_repository.rs -> src-tauri/src/infrastructure/db/mod.rs -> src-tauri/src/infrastructure/db/mapping_repository.rs`
- 2-file cycle: `src-tauri/src/infrastructure/db/mod.rs -> src-tauri/src/infrastructure/db/notification_repository.rs -> src-tauri/src/infrastructure/db/mod.rs`
- 4-file cycle: `src/lib/grid/ImportWizard.svelte -> src/lib/stores/index.ts -> src/lib/stores/ledger.svelte.ts -> src/lib/grid/index.ts -> src/lib/grid/ImportWizard.svelte`

## Communities (86 total, 16 thin omitted)

### Community 0 - "IwaError"
Cohesion: 0.05
Nodes (70): Item, Iterator, ArchiveMessage, parse_archives(), parses_a_real_archive_stream(), parses_consecutive_archives(), real_archive_bytes(), Result (+62 more)

### Community 1 - "ipc/index.ts"
Cohesion: 0.13
Nodes (31): EmailForm, ColumnMappingDto, CurrencyConversionDto, EmailProviderDto, EmailSettingsDto, EntryDto, HabitDto, HabitMappingDto (+23 more)

### Community 2 - "dto.rs"
Cohesion: 0.05
Nodes (67): DailyPoint, HabitBreakdownItem, HabitPreset, HabitWeekTrendDto, MilestoneStatus, Progress, Option, HabitBreakdownItem (+59 more)

### Community 3 - "charts/index.ts"
Cohesion: 0.08
Nodes (30): horizontalBarPath(), verticalBarPath(), formatCalendarDate(), formatClock(), formatClockTime(), formatDayMonth(), formatDuration(), formatHourLabel() (+22 more)

### Community 4 - "commands.rs"
Cohesion: 0.09
Nodes (73): CurrencyConversionDto, EmailSettingsDto, EntryDto, HabitDto, HabitPresetDto, ImportPreviewDto, ImportSummaryDto, NotificationSettingsDto (+65 more)

### Community 5 - "projections.rs"
Cohesion: 0.17
Nodes (29): current_streak_days(), daily_series(), daily_series_zero_fills_gaps(), daily_totals_map(), DailyTotal, entry(), good_habit_entries_keep_a_streak_alive(), habits() (+21 more)

### Community 6 - "grid/index.ts"
Cohesion: 0.05
Nodes (40): formatCell(), HabitLookup, pad(), parseCell(), parseDateCell(), parseNumberCell(), ParseResult, habits (+32 more)

### Community 7 - "domain/notifications.rs"
Cohesion: 0.05
Nodes (62): Fn, all_on(), already_logged_channels_are_skipped(), can_deliver_now(), Category, CategoryPrefs, Channel, channel_key() (+54 more)

### Community 8 - "Apple Design"
Cohesion: 0.10
Nodes (20): 10. Gesture design details (the "feel" checklist), 11. Frame-level smoothness, 12. Materials & depth — translucency conveys hierarchy, 13. Multimodal feedback — motion + sound + haptics, 14. Reduced motion & accessibility, 15. Typography — optical sizing, tracking, leading, 16. Design foundations — the eight principles, 17. Process (+12 more)

### Community 10 - "theme/index.ts"
Cohesion: 0.10
Nodes (24): SettingsStore, AccessibilityPreferences, applyAccessibilityOverrides(), NO_ACCESSIBILITY_OVERRIDES, ACCESSIBILITY_QUERIES, applyTheme(), readAccessibilityPreferences(), themeToCssVars() (+16 more)

### Community 11 - "convert_currency.rs"
Cohesion: 0.09
Nodes (28): a_rate_error_changes_nothing(), applies_the_fetched_rate_to_habits_and_profile(), convert_currency(), FakeRates, fetch_rate(), malformed_codes_are_rejected_before_any_fetch(), network_errors_read_clearly(), Arc (+20 more)

### Community 12 - "mapping.rs"
Cohesion: 0.44
Nodes (10): apply_mapping(), column_habit_mapping_resolves_by_name(), MappingResult, maps_a_well_formed_row_with_fixed_habit(), parses_slash_dates(), quantity_defaults_to_one_unit_when_unmapped(), reports_a_row_error_instead_of_failing_the_whole_import(), Vec (+2 more)

### Community 13 - "cell_storage.rs"
Cohesion: 0.19
Nodes (23): cell_kind(), CellKind, decode_cell(), DecodedCell, decodes_a_bool_cell(), decodes_a_date_cell_relative_to_the_2001_epoch(), decodes_a_duration_cell_as_minutes(), decodes_a_negative_fractional_number() (+15 more)

### Community 14 - "tauri.conf.json"
Cohesion: 0.09
Nodes (22): icons/128x128@2x.png, icons/128x128.png, icons/32x32.png, icons/icon.icns, icons/icon.ico, app, security, windows (+14 more)

### Community 15 - "today_summary.rs"
Cohesion: 0.17
Nodes (21): day_range(), Date, PrimitiveDateTime, average_daily_life_minutes(), compute_today_summary(), current_streak(), habit_week_trends(), HabitWeekTrend (+13 more)

### Community 16 - "stores/index.ts"
Cohesion: 0.12
Nodes (7): i(), BOUNCE_DAMPING, SpringOptions, toSpringOptions(), active, breakdownValueLabel(), formatDuration()

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
Cohesion: 0.14
Nodes (13): autostart:default, core:default, dialog:allow-message, dialog:allow-open, dialog:allow-save, main, notification:default, opener:default (+5 more)

### Community 26 - "package.json"
Cohesion: 0.33
Nodes (5): description, license, name, type, version

### Community 28 - "settings_repository.rs"
Cohesion: 0.11
Nodes (30): Default, Option, Self, String, Settings, StartupRoute, TimeFormat, WeightUnit (+22 more)

### Community 39 - "application/email.rs"
Cohesion: 0.06
Nodes (58): clear_secret(), delete_all_secrets(), delete_all_secrets_clears_every_account(), EmailSettingsView, FakeTransport, FakeTransports, get_settings(), MemoryRepo (+50 more)

### Community 40 - "map_row"
Cohesion: 0.39
Nodes (9): cell_to_datetime(), map_row(), MappedEntry, parse_flexible_datetime(), CellValue, Option, PrimitiveDateTime, Result (+1 more)

### Community 41 - "XlsxSource"
Cohesion: 0.24
Nodes (8): Data, cell_from_calamine(), AsRef, CellValue, Path, PathBuf, Self, XlsxSource

### Community 45 - "SqliteHabitRepository"
Cohesion: 0.22
Nodes (14): insert_then_get_round_trips(), list_excludes_archived_by_default(), repo(), row_to_habit(), Arc, Habit, Option, RepoResult (+6 more)

### Community 48 - "award_points.rs"
Cohesion: 0.05
Nodes (83): Params, RawAward, a_broken_streak_restarts_the_bonus_count(), candidate_awards(), clean_day_awards(), completed_days_without_harmful_entries_are_clean(), day_key(), each_logged_day_earns_five_including_today() (+75 more)

### Community 49 - "models.ts"
Cohesion: 0.13
Nodes (18): formatLocalDate(), formatLocalDateTime(), pad(), parseLocalDate(), parseLocalDateTime(), DailyPointDto, HabitBreakdownItemDto, PeriodCompareDto (+10 more)

### Community 50 - "application/notifications.rs"
Cohesion: 0.10
Nodes (57): Condvar, RefCell, award(), Channels, daily_evaluation_awards_and_announces_once_per_day(), date_label(), deliver(), deliver_progress() (+49 more)

### Community 51 - "Sheet"
Cohesion: 0.20
Nodes (11): CellValue, ImportError, Error, Option, PrimitiveDateTime, String, Vec, Sheet (+3 more)

### Community 55 - "ledger.svelte.ts"
Cohesion: 0.11
Nodes (14): commands, onDataChanged(), Entry, entryFromDto(), entryToDto(), TodaySummary, EntryFields, fieldsEqual() (+6 more)

### Community 56 - "profile_repository.rs"
Cohesion: 0.15
Nodes (17): Profile, Date, Option, Sex, ProfileRepository, get_before_any_save_returns_default(), repo(), Arc (+9 more)

### Community 57 - "today/+page.svelte"
Cohesion: 0.09
Nodes (18): preset(), addCustomHabit(), addHabitOpen, choosePreset(), closeAddHabit(), handlePresetKeydown(), newHabitCost, newHabitLifeMinutes (+10 more)

### Community 59 - "habit-presets.ts"
Cohesion: 0.29
Nodes (10): categoryLabel(), filterPresets(), formatPresetSource(), groupPresets(), PRESET_CATEGORIES, PresetGroup, presetUnitLabel(), presets (+2 more)

### Community 60 - "HabitsStore"
Cohesion: 0.29
Nodes (4): Habit, habitFromDto(), habitToDto(), HabitsStore

### Community 61 - "http_api.rs"
Cohesion: 0.06
Nodes (55): H, Message, Send, EmailTransport, OutgoingEmail, email(), errors_are_readable_and_never_contain_the_key(), FakeHttp (+47 more)

### Community 62 - ".sheets"
Cohesion: 0.18
Nodes (10): CsvSource, infer_cell(), AsRef, CellValue, Path, PathBuf, Result, Self (+2 more)

### Community 64 - "templates.rs"
Cohesion: 0.14
Nodes (30): EmailMessage, AwardLine, escape(), every_template_has_text_html_and_no_external_resources(), format_minutes(), format_quantity(), html(), ImprovedHabit (+22 more)

### Community 65 - "HabitId"
Cohesion: 0.14
Nodes (28): bac_never_negative(), bac_rises_with_drinks_and_falls_with_time(), bac_sex_affects_distribution_ratio(), caffeine_concentration_decays_by_half_at_half_life(), estimated_bac_percent(), estimated_caffeine_mg_per_l(), explicit_duration_overrides_minutes_per_unit(), habit() (+20 more)

### Community 66 - "SqliteEntryRepository"
Cohesion: 0.10
Nodes (26): Into, Entry, EntryId, Option, PrimitiveDateTime, Self, String, Habit (+18 more)

### Community 67 - "migrations.rs"
Cohesion: 0.45
Nodes (19): DbError, fresh_database_reaches_current_version(), migrate_v1(), migrate_v2(), migrate_v3(), migrate_v4(), migrate_v5(), migrate_v6() (+11 more)

### Community 70 - "SecretError"
Cohesion: 0.18
Nodes (14): SecretError, keychain_errors_never_echo_stored_bytes(), KeychainSecretStore, map_error(), memory_store_sets_gets_and_deletes(), MemorySecretStore, Entry, Error (+6 more)

### Community 79 - "import_workbook.rs"
Cohesion: 0.39
Nodes (11): apply_import(), column_habit_mapping_creates_habits_by_name(), detect_source(), imports_a_csv_with_a_fixed_habit_and_remembers_the_mapping(), preview_import(), Box, Path, PathBuf (+3 more)

### Community 80 - ".aged_28"
Cohesion: 0.30
Nodes (14): a_full_thirty_days_of_history_averages_over_thirty_days(), a_net_gain_projects_negative_months_lost(), a_short_history_is_averaged_over_the_days_since_the_first_entry(), a_single_first_entry_is_averaged_over_the_seven_day_floor(), an_empty_window_projects_zero_months(), entries_older_than_thirty_days_are_excluded_from_the_average(), Fixture, life_shortened_does_not_inflate_the_waking_share() (+6 more)

### Community 81 - "compute_timeline"
Cohesion: 0.33
Nodes (7): Duration, compute_timeline(), compute_timeline_includes_entries_from_archived_habits(), compute_timeline_zero_fills_and_compares_against_previous_equal_length_period(), DailyPoint, Date, RepoResult

### Community 84 - "email-settings.ts"
Cohesion: 0.31
Nodes (8): blankToNull(), dtoFromForm(), formFromDto(), PROVIDER_OPTIONS, providerHint(), secretLabel(), SECURITY_OPTIONS, form

### Community 86 - "tray.rs"
Cohesion: 0.27
Nodes (9): App, build(), on_window_event(), AppHandle, R, Result, show_main_window(), Window (+1 more)

### Community 87 - "ColumnMapping"
Cohesion: 0.23
Nodes (10): ImportPreview, ImportSummary, Option, String, Vec, ColumnMapping, HabitMapping, RowError (+2 more)

### Community 88 - "TimeLoss"
Cohesion: 0.40
Nodes (3): Self, TimeLoss, HabitTotal

### Community 89 - "Db"
Cohesion: 0.21
Nodes (13): backup_then_restore_round_trips_data(), Db, migrations_are_idempotent(), reset_clears_notification_tables(), reset_erases_all_data_and_restarts_ids(), AsRef, Connection, Mutex (+5 more)

### Community 90 - "currency_repository.rs"
Cohesion: 0.21
Nodes (15): default_currency(), a_failure_mid_transaction_rolls_everything_back(), archived_habits_are_converted_too(), converts_habits_profile_and_currency_together(), costs(), Fixture, habit(), keeps_other_settings_when_creating_the_currency() (+7 more)

### Community 93 - "RepositoryError"
Cohesion: 0.16
Nodes (16): RepositoryError, Error, From, Parse, Self, now(), repo(), Arc (+8 more)

### Community 97 - "spawn"
Cohesion: 0.33
Nodes (10): PermissionState, DbEmailSender, local_now(), permission_label(), AppHandle, Arc, PrimitiveDateTime, R (+2 more)

### Community 103 - "time-field.ts"
Cohesion: 0.36
Nodes (7): ClockParts, formatClockValue(), fromHour12(), minuteOptions(), parseClockValue(), Period, toHour12()

## Knowledge Gaps
- **147 isolated node(s):** `name`, `version`, `description`, `type`, `dev` (+142 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 401 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **16 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Db` connect `Db` to `spawn`, `SqliteEntryRepository`, `commands.rs`, `application/email.rs`, `domain/notifications.rs`, `convert_currency.rs`, `SqliteHabitRepository`, `award_points.rs`, `application/notifications.rs`, `profile_repository.rs`, `currency_repository.rs`, `settings_repository.rs`, `RepositoryError`?**
  _High betweenness centrality (0.291) - this node is a cross-community bridge._
- **Why does `NotificationHub` connect `application/notifications.rs` to `compute_timeline`, `commands.rs`, `spawn`?**
  _High betweenness centrality (0.065) - this node is a cross-community bridge._
- **Why does `AppState` connect `commands.rs` to `Db`, `application/notifications.rs`, `tray.rs`?**
  _High betweenness centrality (0.042) - this node is a cross-community bridge._
- **Are the 10 inferred relationships involving `HabitId` (e.g. with `explicit_duration_overrides_minutes_per_unit()` and `habit()`) actually correct?**
  _`HabitId` has 10 INFERRED edges - model-reasoned connections that need verification._
- **What connects `name`, `version`, `description` to the rest of the system?**
  _147 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `IwaError` be split into smaller, more focused modules?**
  _Cohesion score 0.05329593267882188 - nodes in this community are weakly interconnected._
- **Should `ipc/index.ts` be split into smaller, more focused modules?**
  _Cohesion score 0.12773109243697478 - nodes in this community are weakly interconnected._