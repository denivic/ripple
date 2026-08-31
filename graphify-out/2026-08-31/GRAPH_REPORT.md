# Graph Report - ripple  (2026-08-31)

## Corpus Check
- 130 files · ~38,662 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 996 nodes · 2124 edges · 59 communities (33 shown, 15 thin omitted)
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
- repository.rs
- entry_repository.rs
- ExportError
- devDependencies
- dependencies
- SvelteKit
- Ripple App Icon/Logo
- compilerOptions
- scripts
- default.json
- formatLocalDate
- package.json
- HabitsStore
- .sheets
- svelte.config.js
- EntryDto
- import_workbook.rs
- timeline.svelte.ts
- svelte-check
- Sheet
- Tauri
- ImportPreview
- PeriodCompareDto
- @tauri-apps/cli
- @types/d3-scale
- @types/d3-time
- TypeScript
- Vite
- ssr
- XlsxSource
- graphify
- ripple
- README.md (Tauri+SvelteKit+TS template)

## God Nodes (most connected - your core abstractions)
1. `HabitId` - 37 edges
2. `IwaError` - 23 edges
3. `AppState` - 22 edges
4. `LedgerStore` - 21 edges
5. `Db` - 21 edges
6. `Apple Design` - 20 edges
7. `decode_cell()` - 18 edges
8. `read_table()` - 17 edges
9. `compute_today_summary()` - 16 edges
10. `daily_series()` - 16 edges

## Surprising Connections (you probably didn't know these)
- `SvelteKit HTML Shell (app.html)` --implements--> `SvelteKit`  [INFERRED]
  src/app.html → README.md
- `read_table()` --calls--> `decode_cell()`  [INFERRED]
  src-tauri/src/infrastructure/import/numbers/source.rs → src-tauri/src/infrastructure/import/numbers/cell_storage.rs
- `sample_mapping()` --calls--> `HabitId`  [INFERRED]
  src-tauri/src/infrastructure/db/mapping_repository.rs → src-tauri/src/domain/habit.rs
- `apply_import()` --calls--> `apply_mapping()`  [INFERRED]
  src-tauri/src/application/import_workbook.rs → src-tauri/src/infrastructure/import/mapping.rs
- `export_entries()` --calls--> `format_datetime()`  [INFERRED]
  src-tauri/src/application/export_workbook.rs → src-tauri/src/infrastructure/db/codec.rs

## Import Cycles
- 2-file cycle: `src-tauri/src/infrastructure/db/mapping_repository.rs -> src-tauri/src/infrastructure/db/mod.rs -> src-tauri/src/infrastructure/db/mapping_repository.rs`
- 4-file cycle: `src/lib/grid/ImportWizard.svelte -> src/lib/stores/index.ts -> src/lib/stores/ledger.svelte.ts -> src/lib/grid/index.ts -> src/lib/grid/ImportWizard.svelte`

## Communities (59 total, 15 thin omitted)

### Community 0 - "IwaError"
Cohesion: 0.05
Nodes (70): Item, Iterator, ArchiveMessage, parse_archives(), parses_a_real_archive_stream(), parses_consecutive_archives(), real_archive_bytes(), Result (+62 more)

### Community 1 - "ipc/index.ts"
Cohesion: 0.19
Nodes (23): parseLocalDate(), ColumnMappingDto, DailyPointDto, EntryDto, HabitBreakdownItemDto, HabitDto, HabitMappingDto, HabitPresetDto (+15 more)

### Community 2 - "dto.rs"
Cohesion: 0.18
Nodes (15): DailyPoint, HabitBreakdownItem, HabitPreset, DailyPointDto, Habit, HabitBreakdownItemDto, HabitDto, HabitPresetDto (+7 more)

### Community 3 - "charts/index.ts"
Cohesion: 0.07
Nodes (30): i(), horizontalBarPath(), verticalBarPath(), cells, columnCount, d, last, maxValue (+22 more)

### Community 4 - "commands.rs"
Cohesion: 0.13
Nodes (46): AppHandle, EntryDto, HabitDto, HabitPresetDto, ImportPreviewDto, ImportSummaryDto, ProfileDto, format_date() (+38 more)

### Community 5 - "HabitId"
Cohesion: 0.07
Nodes (68): compute_timeline(), compute_timeline_includes_entries_from_archived_habits(), compute_timeline_zero_fills_and_compares_against_previous_equal_length_period(), DailyPoint, HabitBreakdownItem, Date, RepoResult, Vec (+60 more)

### Community 6 - "grid/index.ts"
Cohesion: 0.06
Nodes (36): formatCell(), HabitLookup, pad(), parseCell(), parseDateCell(), parseNumberCell(), ParseResult, habits (+28 more)

### Community 7 - "Db"
Cohesion: 0.09
Nodes (35): Mutex, MutexGuard, DbError, RepositoryError, Error, From, Parse, Self (+27 more)

### Community 8 - "Apple Design"
Cohesion: 0.10
Nodes (20): 10. Gesture design details (the "feel" checklist), 11. Frame-level smoothness, 12. Materials & depth — translucency conveys hierarchy, 13. Multimodal feedback — motion + sound + haptics, 14. Reduced motion & accessibility, 15. Typography — optical sizing, tracking, leading, 16. Design foundations — the eight principles, 17. Process (+12 more)

### Community 10 - "theme/index.ts"
Cohesion: 0.13
Nodes (23): AccessibilityPreferences, applyAccessibilityOverrides(), NO_ACCESSIBILITY_OVERRIDES, ACCESSIBILITY_QUERIES, applyTheme(), readAccessibilityPreferences(), themeToCssVars(), watchAccessibilityPreferences() (+15 more)

### Community 11 - "today/+page.svelte"
Cohesion: 0.09
Nodes (9): BOUNCE_DAMPING, SpringOptions, toSpringOptions(), active, addHabitOpen, newHabitCost, newHabitLifeMinutes, newHabitName (+1 more)

### Community 12 - "mapping.rs"
Cohesion: 0.12
Nodes (32): now(), repo(), Arc, Option, PrimitiveDateTime, RepoResult, Self, sample_mapping() (+24 more)

### Community 13 - "cell_storage.rs"
Cohesion: 0.19
Nodes (23): cell_kind(), CellKind, decode_cell(), DecodedCell, decodes_a_bool_cell(), decodes_a_date_cell_relative_to_the_2001_epoch(), decodes_a_duration_cell_as_minutes(), decodes_a_negative_fractional_number() (+15 more)

### Community 14 - "tauri.conf.json"
Cohesion: 0.09
Nodes (22): icons/128x128@2x.png, icons/128x128.png, icons/32x32.png, icons/icon.icns, icons/icon.ico, app, security, windows (+14 more)

### Community 15 - "repository.rs"
Cohesion: 0.06
Nodes (42): Into, export_entries(), export_resolves_habit_names_and_counts_rows(), ExportFormat, Path, Result, day_range(), Date (+34 more)

### Community 16 - "entry_repository.rs"
Cohesion: 0.14
Nodes (21): Entry, EntryId, Option, PrimitiveDateTime, Self, String, deleting_a_habit_cascades_to_its_entries(), insert_then_get_round_trips_including_datetime() (+13 more)

### Community 17 - "ExportError"
Cohesion: 0.13
Nodes (17): Path, Result, write_csv(), JsonRow, Option, Path, Result, write_json() (+9 more)

### Community 18 - "devDependencies"
Cohesion: 0.11
Nodes (19): devDependencies, svelte, @sveltejs/adapter-static, @sveltejs/kit, @sveltejs/vite-plugin-svelte, @types/d3-shape, @types/d3-time-format, typescript (+11 more)

### Community 19 - "dependencies"
Cohesion: 0.12
Nodes (17): d3-array, d3-scale, d3-shape, d3-time, d3-time-format, dependencies, d3-array, d3-scale (+9 more)

### Community 22 - "compilerOptions"
Cohesion: 0.15
Nodes (12): ./.svelte-kit/tsconfig.json, compilerOptions, allowJs, checkJs, esModuleInterop, forceConsistentCasingInFileNames, moduleResolution, resolveJsonModule (+4 more)

### Community 23 - "scripts"
Cohesion: 0.22
Nodes (9): scripts, build, check, check:watch, dev, preview, tauri, test (+1 more)

### Community 24 - "default.json"
Cohesion: 0.18
Nodes (10): core:default, dialog:allow-open, dialog:allow-save, main, opener:default, description, identifier, permissions (+2 more)

### Community 25 - "formatLocalDate"
Cohesion: 0.18
Nodes (11): formatLocalDate(), formatLocalDateTime(), pad(), parseLocalDateTime(), Entry, entryFromDto(), entryToDto(), HabitPreset (+3 more)

### Community 26 - "package.json"
Cohesion: 0.33
Nodes (5): description, license, name, type, version

### Community 27 - "HabitsStore"
Cohesion: 0.26
Nodes (5): commands, Habit, habitFromDto(), habitToDto(), HabitsStore

### Community 28 - ".sheets"
Cohesion: 0.18
Nodes (10): CsvSource, infer_cell(), AsRef, CellValue, Path, PathBuf, Result, Self (+2 more)

### Community 30 - "EntryDto"
Cohesion: 0.23
Nodes (9): Entry, EntryDto, Profile, ProfileDto, Entry, Error, Profile, Result (+1 more)

### Community 31 - "import_workbook.rs"
Cohesion: 0.39
Nodes (11): Box, apply_import(), column_habit_mapping_creates_habits_by_name(), detect_source(), imports_a_csv_with_a_fixed_habit_and_remembers_the_mapping(), preview_import(), Path, PathBuf (+3 more)

### Community 32 - "timeline.svelte.ts"
Cohesion: 0.21
Nodes (5): DataChangedEvent, DataChangedScope, onDataChanged(), Timeline, TimelineStore

### Community 34 - "Sheet"
Cohesion: 0.31
Nodes (8): CellValue, ImportError, Error, Option, PrimitiveDateTime, String, Vec, Sheet

### Community 37 - "ImportPreview"
Cohesion: 0.27
Nodes (8): ImportPreview, ImportSummary, Option, String, Vec, RowError, ImportPreviewDto, ImportSummaryDto

### Community 38 - "PeriodCompareDto"
Cohesion: 0.32
Nodes (6): PeriodCompareResult, Option, PeriodCompareDto, Option, TodaySummaryDto, TodaySummary

### Community 48 - "XlsxSource"
Cohesion: 0.18
Nodes (10): Data, cell_from_calamine(), AsRef, CellValue, Path, PathBuf, Result, Self (+2 more)

## Knowledge Gaps
- **128 isolated node(s):** `name`, `version`, `description`, `type`, `dev` (+123 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 289 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **15 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Db` connect `Db` to `entry_repository.rs`, `mapping.rs`, `commands.rs`, `repository.rs`?**
  _High betweenness centrality (0.264) - this node is a cross-community bridge._
- **Why does `Sheet` connect `Sheet` to `IwaError`, `dto.rs`, `ImportPreview`, `mapping.rs`, `XlsxSource`, `.sheets`, `import_workbook.rs`?**
  _High betweenness centrality (0.163) - this node is a cross-community bridge._
- **Why does `read_table()` connect `IwaError` to `Sheet`, `cell_storage.rs`?**
  _High betweenness centrality (0.107) - this node is a cross-community bridge._
- **Are the 7 inferred relationships involving `HabitId` (e.g. with `habit()` and `money_spent_scales_with_quantity()`) actually correct?**
  _`HabitId` has 7 INFERRED edges - model-reasoned connections that need verification._
- **What connects `name`, `version`, `description` to the rest of the system?**
  _128 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `IwaError` be split into smaller, more focused modules?**
  _Cohesion score 0.05329593267882188 - nodes in this community are weakly interconnected._
- **Should `charts/index.ts` be split into smaller, more focused modules?**
  _Cohesion score 0.0653061224489796 - nodes in this community are weakly interconnected._