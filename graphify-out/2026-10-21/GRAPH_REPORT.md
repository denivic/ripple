# Graph Report - ripple  (2026-10-21)

## Corpus Check
- 142 files · ~50,543 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 1184 nodes · 2603 edges · 67 communities (37 shown, 16 thin omitted)
- Extraction: 97% EXTRACTED · 3% INFERRED · 0% AMBIGUOUS · INFERRED: 74 edges (avg confidence: 0.85)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `a130152a`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- IwaError
- ipc/index.ts
- dto.rs
- charts/index.ts
- commands.rs
- HabitId
- grid/index.ts
- Db
- Apple Design
- @types/d3-array
- theme/index.ts
- convert_currency.rs
- mapping.rs
- cell_storage.rs
- tauri.conf.json
- repository.rs
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
- XlsxSource
- svelte.config.js
- vite
- svelte-check
- Tauri
- navigation.ts
- settings_repository.rs
- .sheets
- TypeScript
- Vite
- ssr
- models.ts
- today/+page.svelte
- import_workbook.rs
- ColumnMapping
- habit-presets.ts
- graphify
- ripple
- Sheet
- onDataChanged
- ImportPreview
- HabitsStore
- DbError
- TimelineStore
- README.md (Tauri+SvelteKit+TS template)

## God Nodes (most connected - your core abstractions)
1. `HabitId` - 41 edges
2. `Db` - 32 edges
3. `AppState` - 30 edges
4. `IwaError` - 23 edges
5. `LedgerStore` - 21 edges
6. `Apple Design` - 20 edges
7. `compute_today_summary()` - 18 edges
8. `decode_cell()` - 18 edges
9. `daily_series()` - 17 edges
10. `read_table()` - 17 edges

## Surprising Connections (you probably didn't know these)
- `SvelteKit HTML Shell (app.html)` --implements--> `SvelteKit`  [INFERRED]
  src/app.html → README.md
- `compute_timeline()` --calls--> `day_range()`  [INFERRED]
  src-tauri/src/application/compute_series.rs → src-tauri/src/application/mod.rs
- `validate()` --calls--> `is_currency_code()`  [INFERRED]
  src-tauri/src/application/convert_currency.rs → src-tauri/src/domain/settings.rs
- `export_entries()` --calls--> `format_datetime()`  [INFERRED]
  src-tauri/src/application/export_workbook.rs → src-tauri/src/infrastructure/db/codec.rs
- `apply_import()` --calls--> `apply_mapping()`  [INFERRED]
  src-tauri/src/application/import_workbook.rs → src-tauri/src/infrastructure/import/mapping.rs

## Import Cycles
- 2-file cycle: `src-tauri/src/infrastructure/db/entry_repository.rs -> src-tauri/src/infrastructure/db/mod.rs -> src-tauri/src/infrastructure/db/entry_repository.rs`
- 2-file cycle: `src-tauri/src/infrastructure/db/mapping_repository.rs -> src-tauri/src/infrastructure/db/mod.rs -> src-tauri/src/infrastructure/db/mapping_repository.rs`
- 2-file cycle: `src-tauri/src/infrastructure/db/mod.rs -> src-tauri/src/infrastructure/db/settings_repository.rs -> src-tauri/src/infrastructure/db/mod.rs`
- 4-file cycle: `src/lib/grid/ImportWizard.svelte -> src/lib/stores/index.ts -> src/lib/stores/ledger.svelte.ts -> src/lib/grid/index.ts -> src/lib/grid/ImportWizard.svelte`

## Communities (67 total, 16 thin omitted)

### Community 0 - "IwaError"
Cohesion: 0.05
Nodes (70): Item, Iterator, ArchiveMessage, parse_archives(), parses_a_real_archive_stream(), parses_consecutive_archives(), real_archive_bytes(), Result (+62 more)

### Community 1 - "ipc/index.ts"
Cohesion: 0.16
Nodes (21): commands, ColumnMappingDto, CurrencyConversionDto, EntryDto, HabitDto, HabitMappingDto, HabitPresetDto, ImportPreviewDto (+13 more)

### Community 2 - "dto.rs"
Cohesion: 0.07
Nodes (49): DailyPoint, HabitBreakdownItem, HabitPreset, PeriodCompareResult, Option, CurrencyConversion, ImportSummary, derived_values_show_their_arithmetic() (+41 more)

### Community 3 - "charts/index.ts"
Cohesion: 0.08
Nodes (24): horizontalBarPath(), verticalBarPath(), formatCalendarDate(), formatDayTick, formatDuration(), formatHourTick, formatMillisecond, formatMinuteTick (+16 more)

### Community 4 - "commands.rs"
Cohesion: 0.10
Nodes (58): AppHandle, CurrencyConversionDto, EntryDto, HabitDto, HabitPresetDto, ImportPreviewDto, ImportSummaryDto, ProfileDto (+50 more)

### Community 5 - "HabitId"
Cohesion: 0.06
Nodes (71): compute_timeline(), compute_timeline_includes_entries_from_archived_habits(), compute_timeline_zero_fills_and_compares_against_previous_equal_length_period(), DailyPoint, HabitBreakdownItem, Date, RepoResult, Vec (+63 more)

### Community 6 - "grid/index.ts"
Cohesion: 0.06
Nodes (36): formatCell(), HabitLookup, pad(), parseCell(), parseDateCell(), parseNumberCell(), ParseResult, habits (+28 more)

### Community 7 - "Db"
Cohesion: 0.05
Nodes (60): Mutex, MutexGuard, EntryId, a_failure_mid_transaction_rolls_everything_back(), archived_habits_are_converted_too(), converts_habits_profile_and_currency_together(), costs(), Fixture (+52 more)

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
Cohesion: 0.21
Nodes (21): apply_mapping(), cell_to_datetime(), column_habit_mapping_resolves_by_name(), HabitMapping, map_row(), MappedEntry, MappingResult, maps_a_well_formed_row_with_fixed_habit() (+13 more)

### Community 13 - "cell_storage.rs"
Cohesion: 0.19
Nodes (23): cell_kind(), CellKind, decode_cell(), DecodedCell, decodes_a_bool_cell(), decodes_a_date_cell_relative_to_the_2001_epoch(), decodes_a_duration_cell_as_minutes(), decodes_a_negative_fractional_number() (+15 more)

### Community 14 - "tauri.conf.json"
Cohesion: 0.09
Nodes (22): icons/128x128@2x.png, icons/128x128.png, icons/32x32.png, icons/icon.icns, icons/icon.ico, app, security, windows (+14 more)

### Community 15 - "repository.rs"
Cohesion: 0.06
Nodes (53): Into, export_entries(), export_resolves_habit_names_and_counts_rows(), ExportFormat, Path, Result, day_range(), Date (+45 more)

### Community 16 - "stores/index.ts"
Cohesion: 0.13
Nodes (7): i(), BOUNCE_DAMPING, SpringOptions, toSpringOptions(), active, announce(), await()

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

### Community 28 - "XlsxSource"
Cohesion: 0.24
Nodes (8): Data, cell_from_calamine(), AsRef, CellValue, Path, PathBuf, Self, XlsxSource

### Community 39 - "navigation.ts"
Cohesion: 0.29
Nodes (6): CaretState, CellPosition, GridBounds, nextCell(), bounds, midCaret

### Community 40 - "settings_repository.rs"
Cohesion: 0.11
Nodes (27): Default, SettingsRepository, is_currency_code(), Option, Self, String, Settings, StartupRoute (+19 more)

### Community 41 - ".sheets"
Cohesion: 0.18
Nodes (10): CsvSource, infer_cell(), AsRef, CellValue, Path, PathBuf, Result, Self (+2 more)

### Community 45 - "models.ts"
Cohesion: 0.16
Nodes (17): formatLocalDate(), formatLocalDateTime(), pad(), parseLocalDate(), parseLocalDateTime(), DailyPointDto, HabitBreakdownItemDto, PeriodCompareDto (+9 more)

### Community 48 - "today/+page.svelte"
Cohesion: 0.12
Nodes (14): addCustomHabit(), addHabitOpen, closeAddHabit(), handlePresetKeydown(), newHabitCost, newHabitLifeMinutes, newHabitName, newHabitUnit (+6 more)

### Community 49 - "import_workbook.rs"
Cohesion: 0.39
Nodes (11): Box, apply_import(), column_habit_mapping_creates_habits_by_name(), detect_source(), imports_a_csv_with_a_fixed_habit_and_remembers_the_mapping(), preview_import(), Path, PathBuf (+3 more)

### Community 50 - "ColumnMapping"
Cohesion: 0.24
Nodes (12): now(), repo(), Arc, Option, PrimitiveDateTime, RepoResult, Self, sample_mapping() (+4 more)

### Community 51 - "habit-presets.ts"
Cohesion: 0.27
Nodes (9): categoryLabel(), filterPresets(), formatPresetSource(), groupPresets(), PRESET_CATEGORIES, PresetGroup, presets, PresetCategory (+1 more)

### Community 54 - "Sheet"
Cohesion: 0.20
Nodes (11): CellValue, ImportError, Error, Option, PrimitiveDateTime, String, Vec, Sheet (+3 more)

### Community 55 - "onDataChanged"
Cohesion: 0.23
Nodes (6): onDataChanged(), Entry, entryFromDto(), entryToDto(), TodaySummary, TodayStore

### Community 56 - "ImportPreview"
Cohesion: 0.40
Nodes (5): ImportPreview, Option, String, Vec, ImportPreviewDto

### Community 57 - "HabitsStore"
Cohesion: 0.29
Nodes (4): Habit, habitFromDto(), habitToDto(), HabitsStore

### Community 58 - "DbError"
Cohesion: 0.53
Nodes (11): DbError, fresh_database_reaches_current_version(), migrate_v1(), migrate_v2(), migrate_v3(), migrate_v4(), Connection, Result (+3 more)

## Knowledge Gaps
- **137 isolated node(s):** `name`, `version`, `description`, `type`, `dev` (+132 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 318 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **16 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Db` connect `Db` to `commands.rs`, `settings_repository.rs`, `convert_currency.rs`, `repository.rs`, `ColumnMapping`?**
  _High betweenness centrality (0.286) - this node is a cross-community bridge._
- **Why does `Sheet` connect `Sheet` to `IwaError`, `dto.rs`, `.sheets`, `mapping.rs`, `import_workbook.rs`, `ImportPreview`?**
  _High betweenness centrality (0.154) - this node is a cross-community bridge._
- **Why does `read_table()` connect `IwaError` to `cell_storage.rs`, `Sheet`?**
  _High betweenness centrality (0.105) - this node is a cross-community bridge._
- **Are the 8 inferred relationships involving `HabitId` (e.g. with `habit()` and `money_spent_scales_with_quantity()`) actually correct?**
  _`HabitId` has 8 INFERRED edges - model-reasoned connections that need verification._
- **What connects `name`, `version`, `description` to the rest of the system?**
  _137 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `IwaError` be split into smaller, more focused modules?**
  _Cohesion score 0.05329593267882188 - nodes in this community are weakly interconnected._
- **Should `dto.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.06512890094979647 - nodes in this community are weakly interconnected._