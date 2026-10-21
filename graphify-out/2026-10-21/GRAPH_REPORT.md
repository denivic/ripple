# Graph Report - ripple  (2026-10-21)

## Corpus Check
- 167 files · ~78,233 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 1799 nodes · 4386 edges · 109 communities (78 shown, 17 thin omitted)
- Extraction: 97% EXTRACTED · 3% INFERRED · 0% AMBIGUOUS · INFERRED: 125 edges (avg confidence: 0.85)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `0450c5e0`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- IwaError
- ipc/index.ts
- Self
- charts/index.ts
- commands.rs
- projections.rs
- grid/index.ts
- Time
- Apple Design
- @types/d3-array
- theme/index.ts
- convert_currency.rs
- mapping.rs
- cell_storage.rs
- tauri.conf.json
- compute_today_summary
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
- application/email.rs
- EmailSettings
- XlsxSource
- TypeScript
- Vite
- ssr
- SqliteHabitRepository
- award_points.rs
- models.ts
- tick
- ImportError
- graphify
- ripple
- PointAward
- ledger.svelte.ts
- profile_repository.rs
- today/+page.svelte
- point_award_repository.rs
- habit-presets.ts
- HabitsStore
- http_api.rs
- .sheets
- smtp.rs
- templates.rs
- cost_model.rs
- SqliteEntryRepository
- migrations.rs
- HabitId
- progress.svelte.ts
- SecretError
- README.md (Tauri+SvelteKit+TS template)
- Option
- import_workbook.rs
- today_summary.rs
- compute_timeline
- TimelineResult
- dto.rs
- email-settings.ts
- presets.rs
- tray.rs
- ImportPreview
- TimeLoss
- db/mod.rs
- Db
- SqliteNotificationRepository
- domain/notifications.rs
- ColumnMapping
- application/notifications.rs
- email_settings_repository.rs
- NotificationHub
- spawn
- String
- CategoryPrefs
- Occurrence
- From
- Sheet
- time-field.ts
- due_occurrences
- pending_channels
- EmailTransport
- RepositoryError
- CurrencyConversion

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
- `clean_day_awards()` --calls--> `milestone_award()`  [INFERRED]
  src-tauri/src/application/award_points.rs → src-tauri/src/domain/points.rs
- `clean_day_awards()` --calls--> `daily_series()`  [INFERRED]
  src-tauri/src/application/award_points.rs → src-tauri/src/domain/projections.rs
- `clean_day_awards()` --calls--> `is_clean_day()`  [INFERRED]
  src-tauri/src/application/award_points.rs → src-tauri/src/domain/projections.rs

## Import Cycles
- 2-file cycle: `src-tauri/src/infrastructure/db/mapping_repository.rs -> src-tauri/src/infrastructure/db/mod.rs -> src-tauri/src/infrastructure/db/mapping_repository.rs`
- 2-file cycle: `src-tauri/src/infrastructure/db/mod.rs -> src-tauri/src/infrastructure/db/notification_repository.rs -> src-tauri/src/infrastructure/db/mod.rs`
- 4-file cycle: `src/lib/grid/ImportWizard.svelte -> src/lib/stores/index.ts -> src/lib/stores/ledger.svelte.ts -> src/lib/grid/index.ts -> src/lib/grid/ImportWizard.svelte`

## Communities (109 total, 17 thin omitted)

### Community 0 - "IwaError"
Cohesion: 0.05
Nodes (70): Item, Iterator, ArchiveMessage, parse_archives(), parses_a_real_archive_stream(), parses_consecutive_archives(), real_archive_bytes(), Result (+62 more)

### Community 1 - "ipc/index.ts"
Cohesion: 0.13
Nodes (31): EmailForm, ColumnMappingDto, CurrencyConversionDto, EmailProviderDto, EmailSettingsDto, EntryDto, HabitDto, HabitMappingDto (+23 more)

### Community 2 - "Self"
Cohesion: 0.17
Nodes (14): is_currency_code(), Entry, EntryDto, Profile, ProfileDto, Entry, Error, Profile (+6 more)

### Community 3 - "charts/index.ts"
Cohesion: 0.07
Nodes (31): horizontalBarPath(), verticalBarPath(), formatCalendarDate(), formatClock(), formatClockTime(), formatDayTick, formatDuration(), formatHourLabel() (+23 more)

### Community 4 - "commands.rs"
Cohesion: 0.09
Nodes (73): CurrencyConversionDto, EmailSettingsDto, EntryDto, HabitDto, HabitPresetDto, ImportPreviewDto, ImportSummaryDto, NotificationSettingsDto (+65 more)

### Community 5 - "projections.rs"
Cohesion: 0.15
Nodes (33): money_spent_for_entry(), Entry, Habit, time_loss_for_entry(), current_streak_days(), daily_series(), daily_series_zero_fills_gaps(), daily_totals_map() (+25 more)

### Community 6 - "grid/index.ts"
Cohesion: 0.05
Nodes (40): formatCell(), HabitLookup, pad(), parseCell(), parseDateCell(), parseNumberCell(), ParseResult, habits (+32 more)

### Community 7 - "Time"
Cohesion: 0.15
Nodes (15): can_deliver_now(), format_clock(), hm(), NotificationSchedule, parse_clock(), QuietHours, Default, Option (+7 more)

### Community 8 - "Apple Design"
Cohesion: 0.10
Nodes (20): 10. Gesture design details (the "feel" checklist), 11. Frame-level smoothness, 12. Materials & depth — translucency conveys hierarchy, 13. Multimodal feedback — motion + sound + haptics, 14. Reduced motion & accessibility, 15. Typography — optical sizing, tracking, leading, 16. Design foundations — the eight principles, 17. Process (+12 more)

### Community 10 - "theme/index.ts"
Cohesion: 0.10
Nodes (24): SettingsStore, AccessibilityPreferences, applyAccessibilityOverrides(), NO_ACCESSIBILITY_OVERRIDES, ACCESSIBILITY_QUERIES, applyTheme(), readAccessibilityPreferences(), themeToCssVars() (+16 more)

### Community 11 - "convert_currency.rs"
Cohesion: 0.11
Nodes (26): a_rate_error_changes_nothing(), applies_the_fetched_rate_to_habits_and_profile(), convert_currency(), FakeRates, fetch_rate(), malformed_codes_are_rejected_before_any_fetch(), network_errors_read_clearly(), Arc (+18 more)

### Community 12 - "mapping.rs"
Cohesion: 0.22
Nodes (20): apply_mapping(), cell_to_datetime(), column_habit_mapping_resolves_by_name(), HabitMapping, map_row(), MappedEntry, MappingResult, maps_a_well_formed_row_with_fixed_habit() (+12 more)

### Community 13 - "cell_storage.rs"
Cohesion: 0.19
Nodes (23): cell_kind(), CellKind, decode_cell(), DecodedCell, decodes_a_bool_cell(), decodes_a_date_cell_relative_to_the_2001_epoch(), decodes_a_duration_cell_as_minutes(), decodes_a_negative_fractional_number() (+15 more)

### Community 14 - "tauri.conf.json"
Cohesion: 0.09
Nodes (22): icons/128x128@2x.png, icons/128x128.png, icons/32x32.png, icons/icon.icns, icons/icon.ico, app, security, windows (+14 more)

### Community 15 - "compute_today_summary"
Cohesion: 0.15
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

### Community 28 - "repository.rs"
Cohesion: 0.24
Nodes (15): evaluate_awards(), get_progress(), RepoResult, export_entries(), export_resolves_habit_names_and_counts_rows(), ExportFormat, Path, Result (+7 more)

### Community 39 - "application/email.rs"
Cohesion: 0.15
Nodes (32): clear_secret(), delete_all_secrets(), delete_all_secrets_clears_every_account(), EmailSettingsView, FakeTransport, FakeTransports, get_settings(), MemoryRepo (+24 more)

### Community 40 - "EmailSettings"
Cohesion: 0.10
Nodes (20): secret_label(), api_providers_ignore_smtp_fields(), default_port_follows_security_mode(), EmailProvider, EmailSettings, is_email_address(), malformed_addresses_are_rejected_even_when_off(), present() (+12 more)

### Community 41 - "XlsxSource"
Cohesion: 0.18
Nodes (10): Data, cell_from_calamine(), AsRef, CellValue, Path, PathBuf, Result, Self (+2 more)

### Community 45 - "SqliteHabitRepository"
Cohesion: 0.22
Nodes (14): insert_then_get_round_trips(), list_excludes_archived_by_default(), repo(), row_to_habit(), Arc, Habit, Option, RepoResult (+6 more)

### Community 48 - "award_points.rs"
Cohesion: 0.06
Nodes (68): a_broken_streak_restarts_the_bonus_count(), candidate_awards(), clean_day_awards(), completed_days_without_harmful_entries_are_clean(), day_key(), each_logged_day_earns_five_including_today(), entry(), evaluating_twice_awards_nothing_new() (+60 more)

### Community 49 - "models.ts"
Cohesion: 0.13
Nodes (18): formatLocalDate(), formatLocalDateTime(), pad(), parseLocalDate(), parseLocalDateTime(), DailyPointDto, HabitBreakdownItemDto, PeriodCompareDto (+10 more)

### Community 50 - "tick"
Cohesion: 0.20
Nodes (26): Channels, date_label(), deliver(), deliver_progress(), Delivery, evaluate_new_awards(), evaluation_guard(), habit_map() (+18 more)

### Community 51 - "ImportError"
Cohesion: 0.36
Nodes (6): CellValue, ImportError, Error, Option, PrimitiveDateTime, String

### Community 54 - "PointAward"
Cohesion: 0.17
Nodes (17): RawAward, AwardKind, Milestone, milestone_award(), milestone_award_key(), PointAward, Date, Option (+9 more)

### Community 55 - "ledger.svelte.ts"
Cohesion: 0.11
Nodes (14): commands, onDataChanged(), Entry, entryFromDto(), entryToDto(), TodaySummary, EntryFields, fieldsEqual() (+6 more)

### Community 56 - "profile_repository.rs"
Cohesion: 0.15
Nodes (17): Profile, Date, Option, Sex, ProfileRepository, get_before_any_save_returns_default(), repo(), Arc (+9 more)

### Community 57 - "today/+page.svelte"
Cohesion: 0.09
Nodes (18): preset(), addCustomHabit(), addHabitOpen, choosePreset(), closeAddHabit(), handlePresetKeydown(), newHabitCost, newHabitLifeMinutes (+10 more)

### Community 58 - "point_award_repository.rs"
Cohesion: 0.20
Nodes (14): Params, clean(), insert_new_skips_keys_already_in_the_ledger(), kind_to_str(), now_utc(), recent_is_newest_first_and_round_trips_fields(), repo(), Arc (+6 more)

### Community 59 - "habit-presets.ts"
Cohesion: 0.29
Nodes (10): categoryLabel(), filterPresets(), formatPresetSource(), groupPresets(), PRESET_CATEGORIES, PresetGroup, presetUnitLabel(), presets (+2 more)

### Community 60 - "HabitsStore"
Cohesion: 0.29
Nodes (4): Habit, habitFromDto(), habitToDto(), HabitsStore

### Community 61 - "http_api.rs"
Cohesion: 0.13
Nodes (29): H, Send, email(), errors_are_readable_and_never_contain_the_key(), FakeHttp, from_header(), HttpOutcome, HttpPost (+21 more)

### Community 62 - ".sheets"
Cohesion: 0.18
Nodes (10): CsvSource, infer_cell(), AsRef, CellValue, Path, PathBuf, Result, Self (+2 more)

### Community 63 - "smtp.rs"
Cohesion: 0.13
Nodes (19): Message, build_message(), email(), html_message_is_multipart_alternative(), invalid_addresses_fail_before_sending(), lettre::transport::smtp::Error, lettre::transport::stub::Error, plain_message_has_headers_and_text_body() (+11 more)

### Community 64 - "templates.rs"
Cohesion: 0.14
Nodes (30): EmailMessage, AwardLine, escape(), every_template_has_text_html_and_no_external_resources(), format_minutes(), format_quantity(), html(), ImprovedHabit (+22 more)

### Community 65 - "cost_model.rs"
Cohesion: 0.21
Nodes (15): bac_never_negative(), bac_rises_with_drinks_and_falls_with_time(), bac_sex_affects_distribution_ratio(), caffeine_concentration_decays_by_half_at_half_life(), estimated_bac_percent(), estimated_caffeine_mg_per_l(), opportunity_cost_for_entry(), projected_waking_months_lost() (+7 more)

### Community 66 - "SqliteEntryRepository"
Cohesion: 0.21
Nodes (16): EntryId, deleting_a_habit_cascades_to_its_entries(), insert_then_get_round_trips_including_datetime(), list_between_filters_by_range(), repo_with_habit(), row_to_entry(), Arc, Entry (+8 more)

### Community 67 - "migrations.rs"
Cohesion: 0.45
Nodes (19): DbError, fresh_database_reaches_current_version(), migrate_v1(), migrate_v2(), migrate_v3(), migrate_v4(), migrate_v5(), migrate_v6() (+11 more)

### Community 68 - "HabitId"
Cohesion: 0.13
Nodes (19): Into, explicit_duration_overrides_minutes_per_unit(), habit(), money_spent_scales_with_quantity(), negative_life_minutes_count_as_life_gained(), opportunity_cost_none_without_income(), opportunity_cost_scales_with_spent_time_only(), time_loss_splits_spent_and_shortened() (+11 more)

### Community 70 - "SecretError"
Cohesion: 0.18
Nodes (14): SecretError, keychain_errors_never_echo_stored_bytes(), KeychainSecretStore, map_error(), memory_store_sets_gets_and_deletes(), MemorySecretStore, Entry, Error (+6 more)

### Community 76 - "Option"
Cohesion: 0.27
Nodes (8): HabitPreset, MilestoneStatus, Progress, Option, HabitPresetDto, MilestoneDto, ProgressDto, Option

### Community 79 - "import_workbook.rs"
Cohesion: 0.39
Nodes (11): apply_import(), column_habit_mapping_creates_habits_by_name(), detect_source(), imports_a_csv_with_a_fixed_habit_and_remembers_the_mapping(), preview_import(), Box, Path, PathBuf (+3 more)

### Community 80 - "today_summary.rs"
Cohesion: 0.32
Nodes (14): a_full_thirty_days_of_history_averages_over_thirty_days(), a_net_gain_projects_negative_months_lost(), a_short_history_is_averaged_over_the_days_since_the_first_entry(), a_single_first_entry_is_averaged_over_the_seven_day_floor(), an_empty_window_projects_zero_months(), entries_older_than_thirty_days_are_excluded_from_the_average(), Fixture, life_shortened_does_not_inflate_the_waking_share() (+6 more)

### Community 81 - "compute_timeline"
Cohesion: 0.33
Nodes (7): Duration, compute_timeline(), compute_timeline_includes_entries_from_archived_habits(), compute_timeline_zero_fills_and_compares_against_previous_equal_length_period(), DailyPoint, Date, RepoResult

### Community 82 - "TimelineResult"
Cohesion: 0.23
Nodes (9): DailyPoint, HabitBreakdownItem, PeriodCompareResult, Option, Vec, TimelineResult, DailyPointDto, PeriodCompareDto (+1 more)

### Community 83 - "dto.rs"
Cohesion: 0.26
Nodes (11): blank_fields_become_absent_and_bad_values_are_rejected(), crate::domain::email::EmailSettings, dto(), EmailSettingsDto, habit_preset_dto_carries_source_and_confidence(), habit_preset_dto_serializes_camel_case_with_nulls(), NotificationCategoryDto, NotificationSettingsDto (+3 more)

### Community 84 - "email-settings.ts"
Cohesion: 0.31
Nodes (8): blankToNull(), dtoFromForm(), formFromDto(), PROVIDER_OPTIONS, providerHint(), secretLabel(), SECURITY_OPTIONS, form

### Community 85 - "presets.rs"
Cohesion: 0.26
Nodes (13): derived_values_show_their_arithmetic(), every_estimate_cites_an_https_source(), every_value_is_finite(), gain_presets_are_negative_and_others_are_not(), habit_presets(), HabitPreset, ids_are_unique(), insufficient_presets_are_zero_with_a_note() (+5 more)

### Community 86 - "tray.rs"
Cohesion: 0.27
Nodes (9): App, build(), on_window_event(), AppHandle, R, Result, show_main_window(), Window (+1 more)

### Community 87 - "ImportPreview"
Cohesion: 0.27
Nodes (8): ImportPreview, ImportSummary, Option, String, Vec, RowError, ImportPreviewDto, ImportSummaryDto

### Community 88 - "TimeLoss"
Cohesion: 0.40
Nodes (3): Self, TimeLoss, HabitTotal

### Community 89 - "db/mod.rs"
Cohesion: 0.21
Nodes (11): backup_then_restore_round_trips_data(), migrations_are_idempotent(), reset_clears_notification_tables(), reset_erases_all_data_and_restarts_ids(), AsRef, Connection, MutexGuard, Path (+3 more)

### Community 90 - "Db"
Cohesion: 0.21
Nodes (16): a_failure_mid_transaction_rolls_everything_back(), archived_habits_are_converted_too(), converts_habits_profile_and_currency_together(), costs(), Fixture, habit(), keeps_other_settings_when_creating_the_currency(), refuses_when_stored_currency_is_not_from() (+8 more)

### Community 91 - "SqliteNotificationRepository"
Cohesion: 0.22
Nodes (10): NotificationSettingsRepository, get_before_any_save_returns_defaults(), log_records_each_key_once(), prefs_round_trip(), repo(), Arc, RepoResult, Self (+2 more)

### Community 92 - "domain/notifications.rs"
Cohesion: 0.18
Nodes (11): entry(), event_notices_wait_out_quiet_hours(), habit(), pace_alert_fires_above_threshold_times_average(), pace_alert_needs_a_nonzero_average(), pace_alerts(), PaceAlert, reminder_needed() (+3 more)

### Community 93 - "ColumnMapping"
Cohesion: 0.24
Nodes (12): now(), repo(), Arc, Option, PrimitiveDateTime, RepoResult, Self, sample_mapping() (+4 more)

### Community 94 - "application/notifications.rs"
Cohesion: 0.32
Nodes (13): award(), daily_evaluation_awards_and_announces_once_per_day(), email_ok(), failed_email_is_not_logged_and_waits_before_retrying(), Fixture, progress_is_bundled_and_held_through_quiet_hours(), progress_lines_announce_a_level_up(), progress_unbundled_sends_one_per_award() (+5 more)

### Community 95 - "email_settings_repository.rs"
Cohesion: 0.32
Nodes (7): repo(), Arc, RepoResult, Self, save_then_get_round_trips_and_upserts(), SqliteEmailSettingsRepository, unknown_stored_values_fall_back_to_defaults()

### Community 96 - "NotificationHub"
Cohesion: 0.24
Nodes (7): Condvar, HubInner, NotificationHub, progress_lines(), Mutex, Vec, progress_key()

### Community 97 - "spawn"
Cohesion: 0.33
Nodes (10): PermissionState, DbEmailSender, local_now(), permission_label(), AppHandle, Arc, PrimitiveDateTime, R (+2 more)

### Community 98 - "String"
Cohesion: 0.27
Nodes (9): RefCell, EmailSender, FakeEmail, FakeLocal, LocalNotifier, progress_notice(), Result, String (+1 more)

### Community 99 - "CategoryPrefs"
Cohesion: 0.27
Nodes (4): Category, CategoryPrefs, NotificationPrefs, Self

### Community 100 - "Occurrence"
Cohesion: 0.40
Nodes (9): kinds(), Occurrence, pace_key(), reminder_key(), Date, String, ScheduledKind, streak_key() (+1 more)

### Community 101 - "From"
Cohesion: 0.33
Nodes (6): HabitBreakdownItem, Habit, HabitBreakdownItemDto, HabitDto, From, Habit

### Community 102 - "Sheet"
Cohesion: 0.28
Nodes (7): HabitWeekTrendDto, Vec, Sheet, Vec, SheetPreviewDto, TodaySummaryDto, TodaySummary

### Community 103 - "time-field.ts"
Cohesion: 0.36
Nodes (7): ClockParts, formatClockValue(), fromHour12(), minuteOptions(), parseClockValue(), Period, toHour12()

### Community 104 - "due_occurrences"
Cohesion: 0.53
Nodes (8): all_on(), disabled_category_is_never_due(), due_occurrences(), quiet_hours_delay_until_the_window_ends(), quiet_hours_wrapping_midnight_carry_over_to_the_next_morning(), reminder_is_not_due_before_its_time_and_is_after(), weekly_summary_only_on_its_day(), yesterdays_unsent_reminder_is_not_dug_up_without_quiet_hours()

### Community 105 - "pending_channels"
Cohesion: 0.36
Nodes (6): Fn, already_logged_channels_are_skipped(), Channel, channel_key(), pending_channels(), Vec

### Community 106 - "EmailTransport"
Cohesion: 0.25
Nodes (6): EmailTransport, ProviderTransports, Box, Result, String, SmtpEmailTransport<T>

### Community 107 - "RepositoryError"
Cohesion: 0.38
Nodes (5): RepositoryError, Error, From, Parse, Self

## Knowledge Gaps
- **149 isolated node(s):** `name`, `version`, `description`, `type`, `dev` (+144 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 397 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **17 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Db` connect `Db` to `spawn`, `SqliteEntryRepository`, `commands.rs`, `application/email.rs`, `convert_currency.rs`, `SqliteHabitRepository`, `award_points.rs`, `profile_repository.rs`, `db/mod.rs`, `point_award_repository.rs`, `SqliteNotificationRepository`, `ColumnMapping`, `application/notifications.rs`, `email_settings_repository.rs`?**
  _High betweenness centrality (0.281) - this node is a cross-community bridge._
- **Why does `NotificationHub` connect `NotificationHub` to `spawn`, `commands.rs`, `compute_timeline`, `tick`, `application/notifications.rs`?**
  _High betweenness centrality (0.078) - this node is a cross-community bridge._
- **Why does `AppState` connect `commands.rs` to `NotificationHub`, `Db`, `tray.rs`?**
  _High betweenness centrality (0.036) - this node is a cross-community bridge._
- **Are the 10 inferred relationships involving `HabitId` (e.g. with `explicit_duration_overrides_minutes_per_unit()` and `habit()`) actually correct?**
  _`HabitId` has 10 INFERRED edges - model-reasoned connections that need verification._
- **What connects `name`, `version`, `description` to the rest of the system?**
  _149 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `IwaError` be split into smaller, more focused modules?**
  _Cohesion score 0.05329593267882188 - nodes in this community are weakly interconnected._
- **Should `ipc/index.ts` be split into smaller, more focused modules?**
  _Cohesion score 0.12773109243697478 - nodes in this community are weakly interconnected._