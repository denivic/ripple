# Graph Report - ripple  (2026-09-12)

## Corpus Check
- 130 files · ~39,172 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 1014 nodes · 2069 edges · 71 communities (40 shown, 16 thin omitted)
- Extraction: 97% EXTRACTED · 3% INFERRED · 0% AMBIGUOUS · INFERRED: 60 edges (avg confidence: 0.85)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `3b33d3c3`
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
- today/+page.svelte
- mapping.rs
- cell_storage.rs
- tauri.conf.json
- profile_repository.rs
- entry_repository.rs
- ExportError
- devDependencies
- dependencies
- SvelteKit
- Ripple App Icon/Logo
- compilerOptions
- scripts
- default.json
- @sveltejs/adapter-static
- package.json
- @types/d3-shape
- XlsxSource
- svelte.config.js
- vite
- svelte-check
- Tauri
- LedgerStore
- SqliteHabitRepository
- .sheets
- TypeScript
- Vite
- ssr
- EntryDto
- import_workbook.rs
- ui/index.ts
- repository.rs
- graphify
- ripple
- Sheet
- compute_timeline
- ImportPreview
- compute_today_summary
- habit.rs
- PeriodCompareDto
- entry.rs
- profile.rs
- motion.ts
- day_range
- README.md (Tauri+SvelteKit+TS template)

## God Nodes (most connected - your core abstractions)
1. `HabitId` - 37 edges
2. `IwaError` - 23 edges
3. `AppState` - 23 edges
4. `Db` - 21 edges
5. `LedgerStore` - 20 edges
6. `Apple Design` - 20 edges
7. `decode_cell()` - 18 edges
8. `read_table()` - 17 edges
9. `compute_today_summary()` - 16 edges
10. `daily_series()` - 16 edges

## Surprising Connections (you probably didn't know these)
- `SvelteKit HTML Shell (app.html)` --implements--> `SvelteKit`  [INFERRED]
  src/app.html → README.md
- `compute_timeline()` --calls--> `day_range()`  [INFERRED]
  src-tauri/src/application/compute_series.rs → src-tauri/src/application/mod.rs
- `compute_timeline()` --calls--> `current_streak_days()`  [INFERRED]
  src-tauri/src/application/compute_series.rs → src-tauri/src/domain/projections.rs
- `compute_timeline()` --calls--> `daily_series()`  [INFERRED]
  src-tauri/src/application/compute_series.rs → src-tauri/src/domain/projections.rs
- `compute_timeline()` --calls--> `hour_weekday_matrix()`  [INFERRED]
  src-tauri/src/application/compute_series.rs → src-tauri/src/domain/projections.rs

## Import Cycles
- 2-file cycle: `src-tauri/src/infrastructure/db/mapping_repository.rs -> src-tauri/src/infrastructure/db/mod.rs -> src-tauri/src/infrastructure/db/mapping_repository.rs`

## Communities (71 total, 16 thin omitted)

### Community 0 - "IwaError"
Cohesion: 0.05
Nodes (70): Item, Iterator, ArchiveMessage, parse_archives(), parses_a_real_archive_stream(), parses_consecutive_archives(), real_archive_bytes(), Result (+62 more)

### Community 1 - "ipc/index.ts"
Cohesion: 0.10
Nodes (41): commands, formatLocalDate(), formatLocalDateTime(), pad(), parseLocalDate(), parseLocalDateTime(), ColumnMappingDto, DailyPointDto (+33 more)

### Community 2 - "dto.rs"
Cohesion: 0.18
Nodes (15): DailyPoint, HabitBreakdownItem, HabitPreset, DailyPointDto, Habit, HabitBreakdownItemDto, HabitDto, HabitPresetDto (+7 more)

### Community 3 - "charts/index.ts"
Cohesion: 0.07
Nodes (30): i(), horizontalBarPath(), verticalBarPath(), cells, columnCount, d, last, maxValue (+22 more)

### Community 4 - "commands.rs"
Cohesion: 0.17
Nodes (37): AppHandle, EntryDto, HabitDto, HabitPresetDto, ImportPreviewDto, ImportSummaryDto, ProfileDto, apply_import() (+29 more)

### Community 5 - "HabitId"
Cohesion: 0.08
Nodes (59): bac_never_negative(), bac_rises_with_drinks_and_falls_with_time(), bac_sex_affects_distribution_ratio(), caffeine_concentration_decays_by_half_at_half_life(), default_habit_presets(), estimated_bac_percent(), estimated_caffeine_mg_per_l(), habit() (+51 more)

### Community 6 - "grid/index.ts"
Cohesion: 0.09
Nodes (30): formatCell(), HabitLookup, pad(), parseCell(), parseDateCell(), parseNumberCell(), ParseResult, habits (+22 more)

### Community 7 - "Db"
Cohesion: 0.09
Nodes (33): Mutex, MutexGuard, DbError, RepositoryError, Error, From, Parse, Self (+25 more)

### Community 8 - "Apple Design"
Cohesion: 0.10
Nodes (20): 10. Gesture design details (the "feel" checklist), 11. Frame-level smoothness, 12. Materials & depth — translucency conveys hierarchy, 13. Multimodal feedback — motion + sound + haptics, 14. Reduced motion & accessibility, 15. Typography — optical sizing, tracking, leading, 16. Design foundations — the eight principles, 17. Process (+12 more)

### Community 10 - "theme/index.ts"
Cohesion: 0.13
Nodes (23): AccessibilityPreferences, applyAccessibilityOverrides(), NO_ACCESSIBILITY_OVERRIDES, ACCESSIBILITY_QUERIES, applyTheme(), readAccessibilityPreferences(), themeToCssVars(), watchAccessibilityPreferences() (+15 more)

### Community 11 - "today/+page.svelte"
Cohesion: 0.10
Nodes (5): addHabitOpen, newHabitCost, newHabitLifeMinutes, newHabitName, newHabitUnit

### Community 12 - "mapping.rs"
Cohesion: 0.22
Nodes (20): apply_mapping(), cell_to_datetime(), column_habit_mapping_resolves_by_name(), HabitMapping, map_row(), MappedEntry, MappingResult, maps_a_well_formed_row_with_fixed_habit() (+12 more)

### Community 13 - "cell_storage.rs"
Cohesion: 0.19
Nodes (23): cell_kind(), CellKind, decode_cell(), DecodedCell, decodes_a_bool_cell(), decodes_a_date_cell_relative_to_the_2001_epoch(), decodes_a_duration_cell_as_minutes(), decodes_a_negative_fractional_number() (+15 more)

### Community 14 - "tauri.conf.json"
Cohesion: 0.09
Nodes (22): icons/128x128@2x.png, icons/128x128.png, icons/32x32.png, icons/icon.icns, icons/icon.ico, app, security, windows (+14 more)

### Community 15 - "profile_repository.rs"
Cohesion: 0.26
Nodes (11): get_before_any_save_returns_default(), repo(), Arc, Option, Profile, RepoResult, Self, save_then_get_round_trips() (+3 more)

### Community 16 - "entry_repository.rs"
Cohesion: 0.13
Nodes (25): EntryId, format_date(), format_datetime(), parse_date(), parse_datetime(), Date, Parse, PrimitiveDateTime (+17 more)

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

### Community 24 - "default.json"
Cohesion: 0.18
Nodes (10): core:default, dialog:allow-open, dialog:allow-save, main, opener:default, description, identifier, permissions (+2 more)

### Community 26 - "package.json"
Cohesion: 0.33
Nodes (5): description, license, name, type, version

### Community 28 - "XlsxSource"
Cohesion: 0.18
Nodes (10): Data, cell_from_calamine(), AsRef, CellValue, Path, PathBuf, Result, Self (+2 more)

### Community 39 - "LedgerStore"
Cohesion: 0.06
Nodes (9): HabitsStore, EntryFields, fieldsEqual(), LedgerStore, SyncedRow, toComparable(), toEntry(), TimelineStore (+1 more)

### Community 40 - "SqliteHabitRepository"
Cohesion: 0.22
Nodes (14): insert_then_get_round_trips(), list_excludes_archived_by_default(), repo(), row_to_habit(), Arc, Habit, Option, RepoResult (+6 more)

### Community 41 - ".sheets"
Cohesion: 0.18
Nodes (10): CsvSource, infer_cell(), AsRef, CellValue, Path, PathBuf, Result, Self (+2 more)

### Community 48 - "EntryDto"
Cohesion: 0.23
Nodes (9): Entry, EntryDto, Profile, ProfileDto, Entry, Error, Profile, Result (+1 more)

### Community 49 - "import_workbook.rs"
Cohesion: 0.39
Nodes (11): Box, apply_import(), column_habit_mapping_creates_habits_by_name(), detect_source(), imports_a_csv_with_a_fixed_habit_and_remembers_the_mapping(), preview_import(), Path, PathBuf (+3 more)

### Community 51 - "repository.rs"
Cohesion: 0.24
Nodes (10): export_entries(), export_resolves_habit_names_and_counts_rows(), ExportFormat, Path, Result, EntryRepository, HabitRepository, ProfileRepository (+2 more)

### Community 54 - "Sheet"
Cohesion: 0.31
Nodes (8): CellValue, ImportError, Error, Option, PrimitiveDateTime, String, Vec, Sheet

### Community 55 - "compute_timeline"
Cohesion: 0.36
Nodes (9): compute_timeline(), compute_timeline_includes_entries_from_archived_habits(), compute_timeline_zero_fills_and_compares_against_previous_equal_length_period(), DailyPoint, HabitBreakdownItem, Date, RepoResult, Vec (+1 more)

### Community 56 - "ImportPreview"
Cohesion: 0.27
Nodes (8): ImportPreview, ImportSummary, Option, String, Vec, RowError, ImportPreviewDto, ImportSummaryDto

### Community 57 - "compute_today_summary"
Cohesion: 0.36
Nodes (8): compute_today_summary(), Date, Option, RepoResult, today_summary_omits_remaining_life_projection_without_a_complete_profile(), today_summary_projects_remaining_life_with_a_complete_profile(), today_summary_sums_only_todays_entries(), TodaySummary

### Community 58 - "habit.rs"
Cohesion: 0.29
Nodes (5): Into, Habit, Option, Self, String

### Community 59 - "PeriodCompareDto"
Cohesion: 0.32
Nodes (6): PeriodCompareResult, Option, PeriodCompareDto, Option, TodaySummaryDto, TodaySummary

### Community 60 - "entry.rs"
Cohesion: 0.38
Nodes (5): Entry, Option, PrimitiveDateTime, Self, String

### Community 61 - "profile.rs"
Cohesion: 0.38
Nodes (5): Profile, Date, Option, Sex, sex_to_str()

### Community 62 - "motion.ts"
Cohesion: 0.50
Nodes (3): BOUNCE_DAMPING, SpringOptions, toSpringOptions()

### Community 63 - "day_range"
Cohesion: 0.50
Nodes (3): day_range(), Date, PrimitiveDateTime

## Knowledge Gaps
- **128 isolated node(s):** `name`, `version`, `description`, `type`, `dev` (+123 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 304 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **16 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Sheet` connect `Sheet` to `IwaError`, `dto.rs`, `.sheets`, `mapping.rs`, `import_workbook.rs`, `ImportPreview`, `XlsxSource`?**
  _High betweenness centrality (0.092) - this node is a cross-community bridge._
- **Why does `read_table()` connect `IwaError` to `cell_storage.rs`, `Sheet`?**
  _High betweenness centrality (0.059) - this node is a cross-community bridge._
- **Why does `HabitId` connect `HabitId` to `Db`, `SqliteHabitRepository`, `mapping.rs`, `entry_repository.rs`, `EntryDto`, `compute_timeline`, `habit.rs`, `entry.rs`?**
  _High betweenness centrality (0.049) - this node is a cross-community bridge._
- **Are the 7 inferred relationships involving `HabitId` (e.g. with `habit()` and `money_spent_scales_with_quantity()`) actually correct?**
  _`HabitId` has 7 INFERRED edges - model-reasoned connections that need verification._
- **What connects `name`, `version`, `description` to the rest of the system?**
  _128 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `IwaError` be split into smaller, more focused modules?**
  _Cohesion score 0.05329593267882188 - nodes in this community are weakly interconnected._
- **Should `ipc/index.ts` be split into smaller, more focused modules?**
  _Cohesion score 0.10460992907801418 - nodes in this community are weakly interconnected._