# Graph Report - ripple  (2026-08-31)

## Corpus Check
- 120 files · ~31,811 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 951 nodes · 1914 edges · 77 communities (35 shown, 33 thin omitted)
- Extraction: 96% EXTRACTED · 4% INFERRED · 0% AMBIGUOUS · INFERRED: 69 edges (avg confidence: 0.85)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `a42c0caf`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- IwaError
- ipc/index.ts
- dto.rs
- charts/index.ts
- commands.rs
- HabitId
- ColumnMapping
- Db
- Apple Design
- import_workbook.rs
- theme/index.ts
- today/+page.svelte
- mapping.rs
- cell_storage.rs
- tauri.conf.json
- repository.rs
- SqliteHabitRepository
- ExportError
- devDependencies
- dependencies
- plan-v1.md (Ripple Implementation Plan)
- Ripple App Icon/Logo
- compilerOptions
- scripts
- default.json
- No visual verification gap (Chrome extension)
- package.json
- Strategy/Adapter Pattern (TabularSource)
- Cost Model (time/life split)
- svelte.config.js
- Habit Breakdown Chart
- Phase 6 - Ledger (continuation detail)
- Phase 7 - Profile + Settings + theme editor (continuation detail)
- svelte-check
- @sveltejs/adapter-static
- @sveltejs/vite-plugin-svelte
- @types/d3-scale
- @types/d3-shape
- @types/d3-time
- Phase 8 - Polish + README (continuation detail)
- pub(crate) hexagonal boundary
- Repository Pattern
- TypeScript
- Vite
- ssr
- XlsxSource
- .sheets
- CellValue
- map_row
- ripple
- Builder Pattern (Chart Option Assembly)
- Calendar Heatmap Chart
- d3 (d3-scale/d3-shape/d3-array/d3-time-format)
- Daily/Weekly Bars + Moving Average Chart
- DTO Boundary Pattern
- ECharts/Chart.js
- egui/iced
- Handsontable/AG Grid
- Hour x Weekday Matrix Chart
- Period Compare Chart
- Phase 1: Scaffold
- Phase 2: Domain + Persistence
- Phase 3: Design System
- Phase 4: Charts + Timeline + Today
- Phase 5: Import/Export
- Settings (Section)
- Streak + Clean-Days Counter
- README.md (Tauri+SvelteKit+TS template)

## God Nodes (most connected - your core abstractions)
1. `HabitId` - 37 edges
2. `IwaError` - 23 edges
3. `AppState` - 23 edges
4. `Db` - 21 edges
5. `Apple Design` - 20 edges
6. `decode_cell()` - 18 edges
7. `read_table()` - 17 edges
8. `daily_series()` - 16 edges
9. `compute_today_summary()` - 16 edges
10. `parse_fields()` - 15 edges

## Surprising Connections (you probably didn't know these)
- `SvelteKit` --semantically_similar_to--> `Svelte 5`  [INFERRED] [semantically similar]
  README.md → plan-v1.md
- `Tauri` --semantically_similar_to--> `Tauri 2`  [INFERRED] [semantically similar]
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
- **Explicitly Rejected Stack Alternatives** — plan_v1_electron, plan_v1_egui_iced, plan_v1_echarts_chart_js, plan_v1_handsontable_ag_grid [EXTRACTED 1.00]
- **Timeline Analytics Chart Suite** — plan_v1_cumulative_ripple_chart, plan_v1_daily_weekly_bars_chart, plan_v1_calendar_heatmap_chart, plan_v1_hour_weekday_matrix_chart, plan_v1_habit_breakdown_chart, plan_v1_streak_counter, plan_v1_period_compare_chart [EXTRACTED 1.00]

## Communities (77 total, 33 thin omitted)

### Community 0 - "IwaError"
Cohesion: 0.05
Nodes (69): Item, Iterator, ArchiveMessage, parse_archives(), parses_a_real_archive_stream(), parses_consecutive_archives(), real_archive_bytes(), Result (+61 more)

### Community 1 - "ipc/index.ts"
Cohesion: 0.08
Nodes (38): commands, formatLocalDate(), formatLocalDateTime(), pad(), parseLocalDate(), parseLocalDateTime(), DailyPointDto, EntryDto (+30 more)

### Community 2 - "dto.rs"
Cohesion: 0.06
Nodes (56): DailyPoint, HabitBreakdownItem, HabitPreset, compute_timeline_includes_entries_from_archived_habits(), compute_timeline_zero_fills_and_compares_against_previous_equal_length_period(), DailyPoint, HabitBreakdownItem, PeriodCompareResult (+48 more)

### Community 3 - "charts/index.ts"
Cohesion: 0.07
Nodes (30): i(), horizontalBarPath(), verticalBarPath(), cells, columnCount, d, last, maxValue (+22 more)

### Community 4 - "commands.rs"
Cohesion: 0.18
Nodes (35): AppHandle, EntryDto, HabitDto, HabitPresetDto, ProfileDto, apply_import(), archive_habit(), compute_timeline() (+27 more)

### Community 5 - "HabitId"
Cohesion: 0.08
Nodes (61): compute_timeline(), RepoResult, bac_never_negative(), bac_rises_with_drinks_and_falls_with_time(), bac_sex_affects_distribution_ratio(), caffeine_concentration_decays_by_half_at_half_life(), default_habit_presets(), estimated_bac_percent() (+53 more)

### Community 6 - "ColumnMapping"
Cohesion: 0.24
Nodes (12): now(), repo(), Arc, Option, PrimitiveDateTime, RepoResult, Self, sample_mapping() (+4 more)

### Community 7 - "Db"
Cohesion: 0.07
Nodes (42): Mutex, MutexGuard, Entry, EntryId, Option, PrimitiveDateTime, Self, String (+34 more)

### Community 8 - "Apple Design"
Cohesion: 0.10
Nodes (20): 10. Gesture design details (the "feel" checklist), 11. Frame-level smoothness, 12. Materials & depth — translucency conveys hierarchy, 13. Multimodal feedback — motion + sound + haptics, 14. Reduced motion & accessibility, 15. Typography — optical sizing, tracking, leading, 16. Design foundations — the eight principles, 17. Process (+12 more)

### Community 9 - "import_workbook.rs"
Cohesion: 0.39
Nodes (11): Box, apply_import(), column_habit_mapping_creates_habits_by_name(), detect_source(), imports_a_csv_with_a_fixed_habit_and_remembers_the_mapping(), preview_import(), Path, PathBuf (+3 more)

### Community 10 - "theme/index.ts"
Cohesion: 0.13
Nodes (23): AccessibilityPreferences, applyAccessibilityOverrides(), NO_ACCESSIBILITY_OVERRIDES, ACCESSIBILITY_QUERIES, applyTheme(), readAccessibilityPreferences(), themeToCssVars(), watchAccessibilityPreferences() (+15 more)

### Community 11 - "today/+page.svelte"
Cohesion: 0.08
Nodes (9): BOUNCE_DAMPING, SpringOptions, toSpringOptions(), active, addHabitOpen, newHabitCost, newHabitLifeMinutes, newHabitName (+1 more)

### Community 12 - "mapping.rs"
Cohesion: 0.38
Nodes (11): apply_mapping(), column_habit_mapping_resolves_by_name(), HabitMapping, MappingResult, maps_a_well_formed_row_with_fixed_habit(), parses_slash_dates(), quantity_defaults_to_one_unit_when_unmapped(), reports_a_row_error_instead_of_failing_the_whole_import() (+3 more)

### Community 13 - "cell_storage.rs"
Cohesion: 0.20
Nodes (22): cell_kind(), CellKind, decode_cell(), DecodedCell, decodes_a_bool_cell(), decodes_a_date_cell_relative_to_the_2001_epoch(), decodes_a_duration_cell_as_minutes(), decodes_a_negative_fractional_number() (+14 more)

### Community 14 - "tauri.conf.json"
Cohesion: 0.09
Nodes (22): icons/128x128@2x.png, icons/128x128.png, icons/32x32.png, icons/icon.icns, icons/icon.ico, app, security, windows (+14 more)

### Community 15 - "repository.rs"
Cohesion: 0.06
Nodes (42): Into, export_entries(), export_resolves_habit_names_and_counts_rows(), ExportFormat, Path, Result, day_range(), Date (+34 more)

### Community 16 - "SqliteHabitRepository"
Cohesion: 0.22
Nodes (14): insert_then_get_round_trips(), list_excludes_archived_by_default(), repo(), row_to_habit(), Arc, Habit, Option, RepoResult (+6 more)

### Community 17 - "ExportError"
Cohesion: 0.13
Nodes (17): Path, Result, write_csv(), JsonRow, Option, Path, Result, write_json() (+9 more)

### Community 18 - "devDependencies"
Cohesion: 0.12
Nodes (17): devDependencies, svelte, @sveltejs/kit, @tauri-apps/cli, @types/d3-array, @types/d3-time-format, typescript, vite (+9 more)

### Community 19 - "dependencies"
Cohesion: 0.13
Nodes (15): d3-time-format, dependencies, d3-array, d3-scale, d3-shape, d3-time, d3-time-format, @tauri-apps/api (+7 more)

### Community 20 - "plan-v1.md (Ripple Implementation Plan)"
Cohesion: 0.13
Nodes (15): apple-design skill, plan-v1-continuation.md (Ripple Continuation Plan), Force-scaffold deleted plan-v1.md, Cumulative Ripple Chart, dataviz skill, plan-v1.md (Ripple Implementation Plan), Electron, GitButler (reference implementation) (+7 more)

### Community 22 - "compilerOptions"
Cohesion: 0.15
Nodes (12): ./.svelte-kit/tsconfig.json, compilerOptions, allowJs, checkJs, esModuleInterop, forceConsistentCasingInFileNames, moduleResolution, resolveJsonModule (+4 more)

### Community 23 - "scripts"
Cohesion: 0.22
Nodes (9): scripts, build, check, check:watch, dev, preview, tauri, test (+1 more)

### Community 24 - "default.json"
Cohesion: 0.22
Nodes (8): core:default, main, opener:default, description, identifier, permissions, $schema, windows

### Community 25 - "No visual verification gap (Chrome extension)"
Cohesion: 0.29
Nodes (7): Command Pattern (Grid Undo/Redo), No visual verification gap (Chrome extension), Stores do full refetch, not fine-grained patching, Ledger (Spreadsheet Editor), Observer Pattern (ripple://data-changed), Timeline (Section), Today (Section)

### Community 26 - "package.json"
Cohesion: 0.33
Nodes (5): description, license, name, type, version

### Community 27 - "Strategy/Adapter Pattern (TabularSource)"
Cohesion: 0.40
Nodes (5): calamine (xlsx reader crate), .numbers reader confidence tiers, .numbers Reader (hand-rolled IWA decoder), rust_xlsxwriter (xlsx writer crate), Strategy/Adapter Pattern (TabularSource)

### Community 28 - "Cost Model (time/life split)"
Cohesion: 0.40
Nodes (5): Widmark BAC/caffeine functions left unreachable, Cost Model (time/life split), Profile (Section), Waking-Life Share Formula, Widmark BAC Model

### Community 30 - "Habit Breakdown Chart"
Cohesion: 0.67
Nodes (3): Client-side cumulative/moving-average/projection math, Metric enum kept with 4 variants, Habit Breakdown Chart

### Community 31 - "Phase 6 - Ledger (continuation detail)"
Cohesion: 0.67
Nodes (3): @tauri-apps/plugin-dialog, Phase 6 - Ledger (continuation detail), Phase 6: Ledger

### Community 32 - "Phase 7 - Profile + Settings + theme editor (continuation detail)"
Cohesion: 0.67
Nodes (3): Phase 7 - Profile + Settings + theme editor (continuation detail), Theme persistence not yet implemented, Phase 7: Profile + Settings + Theme Editor

### Community 48 - "XlsxSource"
Cohesion: 0.20
Nodes (9): Data, cell_from_calamine(), AsRef, Path, PathBuf, Result, Self, Vec (+1 more)

### Community 49 - ".sheets"
Cohesion: 0.20
Nodes (9): CsvSource, infer_cell(), AsRef, Path, PathBuf, Result, Self, Vec (+1 more)

### Community 50 - "CellValue"
Cohesion: 0.36
Nodes (6): CellValue, ImportError, Error, Option, PrimitiveDateTime, String

### Community 51 - "map_row"
Cohesion: 0.43
Nodes (8): cell_to_datetime(), map_row(), MappedEntry, parse_flexible_datetime(), Option, PrimitiveDateTime, Result, String

## Knowledge Gaps
- **148 isolated node(s):** `The Core Idea`, `1. Response — kill latency`, `2. Direct manipulation — 1:1 tracking`, `3. Interruptibility — the single most important principle`, `4. Behavior over animation — use springs` (+143 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 298 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **33 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Db` connect `Db` to `SqliteHabitRepository`, `commands.rs`, `ColumnMapping`, `repository.rs`?**
  _High betweenness centrality (0.184) - this node is a cross-community bridge._
- **Why does `Sheet` connect `dto.rs` to `IwaError`, `import_workbook.rs`, `mapping.rs`, `XlsxSource`, `.sheets`, `CellValue`?**
  _High betweenness centrality (0.118) - this node is a cross-community bridge._
- **Why does `HabitId` connect `HabitId` to `dto.rs`, `ColumnMapping`, `Db`, `mapping.rs`, `repository.rs`, `SqliteHabitRepository`, `map_row`?**
  _High betweenness centrality (0.088) - this node is a cross-community bridge._
- **Are the 7 inferred relationships involving `HabitId` (e.g. with `habit()` and `money_spent_scales_with_quantity()`) actually correct?**
  _`HabitId` has 7 INFERRED edges - model-reasoned connections that need verification._
- **What connects `The Core Idea`, `1. Response — kill latency`, `2. Direct manipulation — 1:1 tracking` to the rest of the system?**
  _148 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `IwaError` be split into smaller, more focused modules?**
  _Cohesion score 0.05422838031533684 - nodes in this community are weakly interconnected._
- **Should `ipc/index.ts` be split into smaller, more focused modules?**
  _Cohesion score 0.07878787878787878 - nodes in this community are weakly interconnected._