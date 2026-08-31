# Graph Report - ripple  (2026-08-31)

## Corpus Check
- Corpus is ~32,904 words - fits in a single context window. You may not need a graph.

## Summary
- 938 nodes · 1897 edges · 79 communities (31 shown, 33 thin omitted)
- Extraction: 96% EXTRACTED · 4% INFERRED · 0% AMBIGUOUS · INFERRED: 69 edges (avg confidence: 0.85)
- Token cost: 0 input · 150,174 output

## Community Hubs (Navigation)
- .numbers IWA Archive Parsing
- Frontend IPC & Datetime Utils
- Timeline Domain Types
- Chart Rendering Components
- Interface DTOs & Codec
- Compute Timeline Use Case
- DB Error Handling
- Export Workbook Use Case
- Today Summary Use Case
- Import Workbook Use Case
- Theme Accessibility System
- UI Motion & Primitives
- Habit Domain Model
- .numbers Cell Storage Decode
- Tauri App Config & Icons
- Profile Domain Model
- SQLite Habit Repository
- CSV/JSON Export Writers
- npm devDependencies
- npm Runtime Dependencies
- Planning Docs & Design Refs
- App Icon Asset Set
- TypeScript Config
- npm Scripts
- Tauri Capability Permissions
- App Sections & Patterns Overview
- package.json Metadata
- Import/Export Format Adapters
- Cost Model & Widmark BAC
- SvelteKit Config
- Client-Side Series Math
- Phase 6 Ledger Plan
- Phase 7 Profile/Settings Plan
- svelte-check Tooling
- SvelteKit Static Adapter
- Svelte Vite Plugin
- d3-scale Types
- d3-shape Types
- d3-time Types
- Phase 8 Polish Plan
- Hexagonal Architecture Boundary
- Repository Pattern & rusqlite
- TypeScript Dependency
- Vite Dependency
- SvelteKit Layout Config
- Project Name Node
- Chart Builder Pattern
- Calendar Heatmap Chart
- d3 Library Bundle
- Daily Bars + MA Chart
- DTO Boundary Pattern
- Rejected: ECharts/Chart.js
- Rejected: egui/iced
- Rejected: Handsontable/AG Grid
- Hour x Weekday Matrix Chart
- Period Compare Chart
- Phase 1 Scaffold
- Phase 2 Domain+Persistence
- Phase 3 Design System
- Phase 4 Charts+Timeline+Today
- Phase 5 Import/Export
- Settings Section
- Streak Counter Chart
- README Template Node

## God Nodes (most connected - your core abstractions)
1. `HabitId` - 37 edges
2. `IwaError` - 23 edges
3. `AppState` - 23 edges
4. `Db` - 21 edges
5. `decode_cell()` - 18 edges
6. `read_table()` - 17 edges
7. `compute_today_summary()` - 16 edges
8. `daily_series()` - 16 edges
9. `compute_timeline()` - 15 edges
10. `Sheet` - 15 edges

## Surprising Connections (you probably didn't know these)
- `Tauri` --semantically_similar_to--> `Tauri 2`  [INFERRED] [semantically similar]
  README.md → plan-v1.md
- `SvelteKit` --semantically_similar_to--> `Svelte 5`  [INFERRED] [semantically similar]
  README.md → plan-v1.md
- `TypeScript` --semantically_similar_to--> `TypeScript`  [INFERRED] [semantically similar]
  README.md → plan-v1.md
- `Vite` --semantically_similar_to--> `Vite`  [INFERRED] [semantically similar]
  README.md → plan-v1.md
- `SvelteKit HTML Shell (app.html)` --implements--> `SvelteKit`  [INFERRED]
  src/app.html → README.md

## Import Cycles
- 2-file cycle: `src-tauri/src/infrastructure/db/mapping_repository.rs -> src-tauri/src/infrastructure/db/mod.rs -> src-tauri/src/infrastructure/db/mapping_repository.rs`

## Hyperedges (group relationships)
- **Hexagonal Architecture Pattern Set** — plan_v1_hexagonal_architecture, plan_v1_repository_pattern, plan_v1_strategy_adapter_pattern, plan_v1_command_pattern, plan_v1_observer_pattern, plan_v1_builder_pattern, plan_v1_dto_boundary_pattern [EXTRACTED 1.00]
- **Timeline Analytics Chart Suite** — plan_v1_cumulative_ripple_chart, plan_v1_daily_weekly_bars_chart, plan_v1_calendar_heatmap_chart, plan_v1_hour_weekday_matrix_chart, plan_v1_habit_breakdown_chart, plan_v1_streak_counter, plan_v1_period_compare_chart [EXTRACTED 1.00]
- **Explicitly Rejected Stack Alternatives** — plan_v1_electron, plan_v1_egui_iced, plan_v1_echarts_chart_js, plan_v1_handsontable_ag_grid [EXTRACTED 1.00]

## Communities (79 total, 33 thin omitted)

### Community 0 - ".numbers IWA Archive Parsing"
Cohesion: 0.05
Nodes (69): Item, Iterator, ArchiveMessage, parse_archives(), parses_a_real_archive_stream(), parses_consecutive_archives(), real_archive_bytes(), Result (+61 more)

### Community 1 - "Frontend IPC & Datetime Utils"
Cohesion: 0.08
Nodes (38): commands, formatLocalDate(), formatLocalDateTime(), pad(), parseLocalDate(), parseLocalDateTime(), DailyPointDto, EntryDto (+30 more)

### Community 2 - "Timeline Domain Types"
Cohesion: 0.08
Nodes (42): DailyPoint, HabitBreakdownItem, HabitPreset, HabitBreakdownItem, PeriodCompareResult, Option, Vec, TimelineResult (+34 more)

### Community 3 - "Chart Rendering Components"
Cohesion: 0.07
Nodes (30): i(), horizontalBarPath(), verticalBarPath(), cells, columnCount, d, last, maxValue (+22 more)

### Community 4 - "Interface DTOs & Codec"
Cohesion: 0.13
Nodes (44): AppHandle, EntryDto, HabitDto, HabitPresetDto, ProfileDto, format_date(), format_datetime(), parse_date() (+36 more)

### Community 5 - "Compute Timeline Use Case"
Cohesion: 0.12
Nodes (38): compute_timeline(), compute_timeline_includes_entries_from_archived_habits(), compute_timeline_zero_fills_and_compares_against_previous_equal_length_period(), DailyPoint, Date, RepoResult, money_spent_for_entry(), Self (+30 more)

### Community 6 - "DB Error Handling"
Cohesion: 0.09
Nodes (32): Mutex, MutexGuard, DbError, RepositoryError, Error, From, Parse, Self (+24 more)

### Community 7 - "Export Workbook Use Case"
Cohesion: 0.10
Nodes (30): export_entries(), export_resolves_habit_names_and_counts_rows(), ExportFormat, Path, Result, Entry, EntryId, Option (+22 more)

### Community 8 - "Today Summary Use Case"
Cohesion: 0.09
Nodes (37): day_range(), Date, PrimitiveDateTime, compute_today_summary(), Date, Option, RepoResult, today_summary_omits_remaining_life_projection_without_a_complete_profile() (+29 more)

### Community 9 - "Import Workbook Use Case"
Cohesion: 0.08
Nodes (32): Box, Data, apply_import(), column_habit_mapping_creates_habits_by_name(), detect_source(), imports_a_csv_with_a_fixed_habit_and_remembers_the_mapping(), preview_import(), Path (+24 more)

### Community 10 - "Theme Accessibility System"
Cohesion: 0.13
Nodes (23): AccessibilityPreferences, applyAccessibilityOverrides(), NO_ACCESSIBILITY_OVERRIDES, ACCESSIBILITY_QUERIES, applyTheme(), readAccessibilityPreferences(), themeToCssVars(), watchAccessibilityPreferences() (+15 more)

### Community 11 - "UI Motion & Primitives"
Cohesion: 0.08
Nodes (9): BOUNCE_DAMPING, SpringOptions, toSpringOptions(), active, addHabitOpen, newHabitCost, newHabitLifeMinutes, newHabitName (+1 more)

### Community 12 - "Habit Domain Model"
Cohesion: 0.16
Nodes (27): Into, Habit, HabitId, Option, Self, String, apply_mapping(), cell_to_datetime() (+19 more)

### Community 13 - ".numbers Cell Storage Decode"
Cohesion: 0.16
Nodes (25): CellValue, Option, String, cell_kind(), CellKind, decode_cell(), DecodedCell, decodes_a_bool_cell() (+17 more)

### Community 14 - "Tauri App Config & Icons"
Cohesion: 0.09
Nodes (22): icons/128x128@2x.png, icons/128x128.png, icons/32x32.png, icons/icon.icns, icons/icon.ico, app, security, windows (+14 more)

### Community 15 - "Profile Domain Model"
Cohesion: 0.15
Nodes (17): Profile, Date, Option, Sex, ProfileRepository, get_before_any_save_returns_default(), repo(), Arc (+9 more)

### Community 16 - "SQLite Habit Repository"
Cohesion: 0.22
Nodes (14): insert_then_get_round_trips(), list_excludes_archived_by_default(), repo(), row_to_habit(), Arc, Habit, Option, RepoResult (+6 more)

### Community 17 - "CSV/JSON Export Writers"
Cohesion: 0.13
Nodes (17): Path, Result, write_csv(), JsonRow, Option, Path, Result, write_json() (+9 more)

### Community 18 - "npm devDependencies"
Cohesion: 0.12
Nodes (17): devDependencies, svelte, @sveltejs/kit, @tauri-apps/cli, @types/d3-array, @types/d3-time-format, typescript, vite (+9 more)

### Community 19 - "npm Runtime Dependencies"
Cohesion: 0.13
Nodes (15): d3-time-format, dependencies, d3-array, d3-scale, d3-shape, d3-time, d3-time-format, @tauri-apps/api (+7 more)

### Community 20 - "Planning Docs & Design Refs"
Cohesion: 0.13
Nodes (15): apple-design skill, plan-v1-continuation.md (Ripple Continuation Plan), Force-scaffold deleted plan-v1.md, Cumulative Ripple Chart, dataviz skill, plan-v1.md (Ripple Implementation Plan), Electron, GitButler (reference implementation) (+7 more)

### Community 22 - "TypeScript Config"
Cohesion: 0.15
Nodes (12): ./.svelte-kit/tsconfig.json, compilerOptions, allowJs, checkJs, esModuleInterop, forceConsistentCasingInFileNames, moduleResolution, resolveJsonModule (+4 more)

### Community 23 - "npm Scripts"
Cohesion: 0.22
Nodes (9): scripts, build, check, check:watch, dev, preview, tauri, test (+1 more)

### Community 24 - "Tauri Capability Permissions"
Cohesion: 0.22
Nodes (8): core:default, main, opener:default, description, identifier, permissions, $schema, windows

### Community 25 - "App Sections & Patterns Overview"
Cohesion: 0.29
Nodes (7): Command Pattern (Grid Undo/Redo), No visual verification gap (Chrome extension), Stores do full refetch, not fine-grained patching, Ledger (Spreadsheet Editor), Observer Pattern (ripple://data-changed), Timeline (Section), Today (Section)

### Community 26 - "package.json Metadata"
Cohesion: 0.33
Nodes (5): description, license, name, type, version

### Community 27 - "Import/Export Format Adapters"
Cohesion: 0.40
Nodes (5): calamine (xlsx reader crate), .numbers reader confidence tiers, .numbers Reader (hand-rolled IWA decoder), rust_xlsxwriter (xlsx writer crate), Strategy/Adapter Pattern (TabularSource)

### Community 28 - "Cost Model & Widmark BAC"
Cohesion: 0.40
Nodes (5): Widmark BAC/caffeine functions left unreachable, Cost Model (time/life split), Profile (Section), Waking-Life Share Formula, Widmark BAC Model

### Community 30 - "Client-Side Series Math"
Cohesion: 0.67
Nodes (3): Client-side cumulative/moving-average/projection math, Metric enum kept with 4 variants, Habit Breakdown Chart

### Community 31 - "Phase 6 Ledger Plan"
Cohesion: 0.67
Nodes (3): @tauri-apps/plugin-dialog, Phase 6 - Ledger (continuation detail), Phase 6: Ledger

### Community 32 - "Phase 7 Profile/Settings Plan"
Cohesion: 0.67
Nodes (3): Phase 7 - Profile + Settings + theme editor (continuation detail), Theme persistence not yet implemented, Phase 7: Profile + Settings + Theme Editor

## Knowledge Gaps
- **129 isolated node(s):** `name`, `version`, `description`, `type`, `dev` (+124 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 284 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **33 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Db` connect `DB Error Handling` to `SQLite Habit Repository`, `Profile Domain Model`, `Interface DTOs & Codec`, `Export Workbook Use Case`?**
  _High betweenness centrality (0.190) - this node is a cross-community bridge._
- **Why does `Sheet` connect `Timeline Domain Types` to `.numbers IWA Archive Parsing`, `Import Workbook Use Case`, `Habit Domain Model`, `.numbers Cell Storage Decode`?**
  _High betweenness centrality (0.122) - this node is a cross-community bridge._
- **Why does `HabitId` connect `Habit Domain Model` to `Timeline Domain Types`, `Compute Timeline Use Case`, `DB Error Handling`, `Export Workbook Use Case`, `Today Summary Use Case`, `SQLite Habit Repository`?**
  _High betweenness centrality (0.090) - this node is a cross-community bridge._
- **Are the 7 inferred relationships involving `HabitId` (e.g. with `habit()` and `money_spent_scales_with_quantity()`) actually correct?**
  _`HabitId` has 7 INFERRED edges - model-reasoned connections that need verification._
- **What connects `name`, `version`, `description` to the rest of the system?**
  _129 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `.numbers IWA Archive Parsing` be split into smaller, more focused modules?**
  _Cohesion score 0.05422838031533684 - nodes in this community are weakly interconnected._
- **Should `Frontend IPC & Datetime Utils` be split into smaller, more focused modules?**
  _Cohesion score 0.07878787878787878 - nodes in this community are weakly interconnected._