# Graph Report - ripple  (2026-08-31)

## Corpus Check
- 130 files · ~38,662 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 1003 nodes · 2135 edges · 54 communities (24 shown, 15 thin omitted)
- Extraction: 97% EXTRACTED · 3% INFERRED · 0% AMBIGUOUS · INFERRED: 63 edges (avg confidence: 0.85)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `74cd0872`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- IwaError
- ipc/index.ts
- dto.rs
- charts/index.ts
- commands.rs
- cost_model.rs
- grid/index.ts
- Db
- Apple Design
- @types/d3-array
- theme/index.ts
- today/+page.svelte
- HabitId
- cell_storage.rs
- tauri.conf.json
- profile_repository.rs
- repository.rs
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
- import_workbook.rs
- svelte.config.js
- vite
- svelte-check
- Tauri
- TypeScript
- Vite
- ssr
- graphify
- ripple
- README.md (Tauri+SvelteKit+TS template)

## God Nodes (most connected - your core abstractions)
1. `HabitId` - 37 edges
2. `IwaError` - 23 edges
3. `AppState` - 23 edges
4. `Db` - 21 edges
5. `LedgerStore` - 21 edges
6. `Apple Design` - 20 edges
7. `decode_cell()` - 18 edges
8. `read_table()` - 17 edges
9. `compute_today_summary()` - 16 edges
10. `daily_series()` - 16 edges

## Surprising Connections (you probably didn't know these)
- `SvelteKit HTML Shell (app.html)` --implements--> `SvelteKit`  [INFERRED]
  src/app.html → README.md
- `export_entries()` --calls--> `format_datetime()`  [INFERRED]
  src-tauri/src/application/export_workbook.rs → src-tauri/src/infrastructure/db/codec.rs
- `apply_import()` --calls--> `apply_mapping()`  [INFERRED]
  src-tauri/src/application/import_workbook.rs → src-tauri/src/infrastructure/import/mapping.rs
- `get_habit_presets()` --calls--> `default_habit_presets()`  [INFERRED]
  src-tauri/src/interface/commands.rs → src-tauri/src/domain/cost_model.rs
- `habit()` --calls--> `HabitId`  [INFERRED]
  src-tauri/src/domain/cost_model.rs → src-tauri/src/domain/habit.rs

## Import Cycles
- 2-file cycle: `src-tauri/src/infrastructure/db/mapping_repository.rs -> src-tauri/src/infrastructure/db/mod.rs -> src-tauri/src/infrastructure/db/mapping_repository.rs`
- 4-file cycle: `src/lib/grid/ImportWizard.svelte -> src/lib/stores/index.ts -> src/lib/stores/ledger.svelte.ts -> src/lib/grid/index.ts -> src/lib/grid/ImportWizard.svelte`

## Communities (54 total, 15 thin omitted)

### Community 0 - "IwaError"
Cohesion: 0.05
Nodes (70): Item, Iterator, ArchiveMessage, parse_archives(), parses_a_real_archive_stream(), parses_consecutive_archives(), real_archive_bytes(), Result (+62 more)

### Community 1 - "ipc/index.ts"
Cohesion: 0.07
Nodes (46): commands, formatLocalDate(), formatLocalDateTime(), pad(), parseLocalDate(), parseLocalDateTime(), ColumnMappingDto, DailyPointDto (+38 more)

### Community 2 - "dto.rs"
Cohesion: 0.08
Nodes (40): DailyPoint, HabitBreakdownItem, HabitPreset, compute_timeline_includes_entries_from_archived_habits(), compute_timeline_zero_fills_and_compares_against_previous_equal_length_period(), DailyPoint, HabitBreakdownItem, PeriodCompareResult (+32 more)

### Community 3 - "charts/index.ts"
Cohesion: 0.07
Nodes (30): i(), horizontalBarPath(), verticalBarPath(), cells, columnCount, d, last, maxValue (+22 more)

### Community 4 - "commands.rs"
Cohesion: 0.12
Nodes (46): AppHandle, EntryDto, HabitDto, HabitPresetDto, ImportPreviewDto, ImportSummaryDto, ProfileDto, format_date() (+38 more)

### Community 5 - "cost_model.rs"
Cohesion: 0.06
Nodes (71): compute_timeline(), RepoResult, day_range(), Date, PrimitiveDateTime, compute_today_summary(), Date, Option (+63 more)

### Community 6 - "grid/index.ts"
Cohesion: 0.06
Nodes (34): formatCell(), HabitLookup, pad(), parseCell(), parseDateCell(), parseNumberCell(), ParseResult, habits (+26 more)

### Community 7 - "Db"
Cohesion: 0.06
Nodes (46): Mutex, MutexGuard, DbError, RepositoryError, Error, From, Parse, Self (+38 more)

### Community 8 - "Apple Design"
Cohesion: 0.10
Nodes (20): 10. Gesture design details (the "feel" checklist), 11. Frame-level smoothness, 12. Materials & depth — translucency conveys hierarchy, 13. Multimodal feedback — motion + sound + haptics, 14. Reduced motion & accessibility, 15. Typography — optical sizing, tracking, leading, 16. Design foundations — the eight principles, 17. Process (+12 more)

### Community 10 - "theme/index.ts"
Cohesion: 0.13
Nodes (23): AccessibilityPreferences, applyAccessibilityOverrides(), NO_ACCESSIBILITY_OVERRIDES, ACCESSIBILITY_QUERIES, applyTheme(), readAccessibilityPreferences(), themeToCssVars(), watchAccessibilityPreferences() (+15 more)

### Community 11 - "today/+page.svelte"
Cohesion: 0.09
Nodes (9): BOUNCE_DAMPING, SpringOptions, toSpringOptions(), active, addHabitOpen, newHabitCost, newHabitLifeMinutes, newHabitName (+1 more)

### Community 12 - "HabitId"
Cohesion: 0.16
Nodes (28): Into, Habit, HabitId, Option, Self, String, apply_mapping(), cell_to_datetime() (+20 more)

### Community 13 - "cell_storage.rs"
Cohesion: 0.19
Nodes (23): cell_kind(), CellKind, decode_cell(), DecodedCell, decodes_a_bool_cell(), decodes_a_date_cell_relative_to_the_2001_epoch(), decodes_a_duration_cell_as_minutes(), decodes_a_negative_fractional_number() (+15 more)

### Community 14 - "tauri.conf.json"
Cohesion: 0.09
Nodes (22): icons/128x128@2x.png, icons/128x128.png, icons/32x32.png, icons/icon.icns, icons/icon.ico, app, security, windows (+14 more)

### Community 15 - "profile_repository.rs"
Cohesion: 0.15
Nodes (17): Profile, Date, Option, Sex, ProfileRepository, get_before_any_save_returns_default(), repo(), Arc (+9 more)

### Community 16 - "repository.rs"
Cohesion: 0.10
Nodes (30): export_entries(), export_resolves_habit_names_and_counts_rows(), ExportFormat, Path, Result, Entry, EntryId, Option (+22 more)

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

### Community 28 - "import_workbook.rs"
Cohesion: 0.06
Nodes (43): Box, Data, apply_import(), column_habit_mapping_creates_habits_by_name(), detect_source(), ImportPreview, imports_a_csv_with_a_fixed_habit_and_remembers_the_mapping(), preview_import() (+35 more)

## Knowledge Gaps
- **128 isolated node(s):** `name`, `version`, `description`, `type`, `dev` (+123 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 293 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **15 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Db` connect `Db` to `repository.rs`, `commands.rs`, `profile_repository.rs`?**
  _High betweenness centrality (0.260) - this node is a cross-community bridge._
- **Why does `Sheet` connect `import_workbook.rs` to `IwaError`, `dto.rs`, `HabitId`?**
  _High betweenness centrality (0.136) - this node is a cross-community bridge._
- **Why does `read_table()` connect `IwaError` to `import_workbook.rs`, `cell_storage.rs`?**
  _High betweenness centrality (0.093) - this node is a cross-community bridge._
- **Are the 7 inferred relationships involving `HabitId` (e.g. with `habit()` and `money_spent_scales_with_quantity()`) actually correct?**
  _`HabitId` has 7 INFERRED edges - model-reasoned connections that need verification._
- **What connects `name`, `version`, `description` to the rest of the system?**
  _128 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `IwaError` be split into smaller, more focused modules?**
  _Cohesion score 0.05329593267882188 - nodes in this community are weakly interconnected._
- **Should `ipc/index.ts` be split into smaller, more focused modules?**
  _Cohesion score 0.06766917293233082 - nodes in this community are weakly interconnected._