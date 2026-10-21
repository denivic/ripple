# Graph Report - ripple  (2026-09-13)

## Corpus Check
- 135 files · ~43,726 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 1069 nodes · 2324 edges · 61 communities (32 shown, 15 thin omitted)
- Extraction: 97% EXTRACTED · 3% INFERRED · 0% AMBIGUOUS · INFERRED: 65 edges (avg confidence: 0.85)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `e8e76ccf`
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
- SqliteHabitRepository
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
- Self
- import_workbook.rs
- graphify
- ripple
- Sheet
- TimelineResult
- ImportPreview
- README.md (Tauri+SvelteKit+TS template)

## God Nodes (most connected - your core abstractions)
1. `HabitId` - 37 edges
2. `AppState` - 29 edges
3. `Db` - 26 edges
4. `IwaError` - 23 edges
5. `LedgerStore` - 21 edges
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
- `export_entries()` --calls--> `format_datetime()`  [INFERRED]
  src-tauri/src/application/export_workbook.rs → src-tauri/src/infrastructure/db/codec.rs
- `apply_import()` --calls--> `apply_mapping()`  [INFERRED]
  src-tauri/src/application/import_workbook.rs → src-tauri/src/infrastructure/import/mapping.rs
- `compute_today_summary()` --calls--> `money_spent_for_entry()`  [INFERRED]
  src-tauri/src/application/today_summary.rs → src-tauri/src/domain/cost_model.rs

## Import Cycles
- 2-file cycle: `src-tauri/src/infrastructure/db/mapping_repository.rs -> src-tauri/src/infrastructure/db/mod.rs -> src-tauri/src/infrastructure/db/mapping_repository.rs`
- 4-file cycle: `src/lib/grid/ImportWizard.svelte -> src/lib/stores/index.ts -> src/lib/stores/ledger.svelte.ts -> src/lib/grid/index.ts -> src/lib/grid/ImportWizard.svelte`

## Communities (61 total, 15 thin omitted)

### Community 0 - "IwaError"
Cohesion: 0.05
Nodes (70): Item, Iterator, ArchiveMessage, parse_archives(), parses_a_real_archive_stream(), parses_consecutive_archives(), real_archive_bytes(), Result (+62 more)

### Community 1 - "ipc/index.ts"
Cohesion: 0.06
Nodes (48): commands, formatLocalDate(), formatLocalDateTime(), pad(), parseLocalDate(), parseLocalDateTime(), ColumnMappingDto, DailyPointDto (+40 more)

### Community 2 - "dto.rs"
Cohesion: 0.20
Nodes (15): DailyPoint, HabitPreset, DailyPointDto, Habit, HabitDto, HabitPresetDto, ImportPreviewDto, From (+7 more)

### Community 3 - "charts/index.ts"
Cohesion: 0.08
Nodes (24): i(), horizontalBarPath(), verticalBarPath(), formatCalendarDate(), formatDayTick, formatDuration(), formatHourTick, formatMillisecond (+16 more)

### Community 4 - "commands.rs"
Cohesion: 0.14
Nodes (46): AppHandle, EntryDto, HabitDto, HabitPresetDto, ImportPreviewDto, ImportSummaryDto, ProfileDto, SettingsDto (+38 more)

### Community 5 - "HabitId"
Cohesion: 0.07
Nodes (66): compute_timeline(), compute_timeline_includes_entries_from_archived_habits(), compute_timeline_zero_fills_and_compares_against_previous_equal_length_period(), DailyPoint, HabitBreakdownItem, Date, RepoResult, bac_never_negative() (+58 more)

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
Cohesion: 0.05
Nodes (35): SettingsStore, AccessibilityPreferences, applyAccessibilityOverrides(), NO_ACCESSIBILITY_OVERRIDES, ACCESSIBILITY_QUERIES, applyTheme(), readAccessibilityPreferences(), themeToCssVars() (+27 more)

### Community 11 - "SqliteHabitRepository"
Cohesion: 0.22
Nodes (14): insert_then_get_round_trips(), list_excludes_archived_by_default(), repo(), row_to_habit(), Arc, Habit, Option, RepoResult (+6 more)

### Community 12 - "mapping.rs"
Cohesion: 0.22
Nodes (20): apply_mapping(), cell_to_datetime(), column_habit_mapping_resolves_by_name(), HabitMapping, map_row(), MappedEntry, MappingResult, maps_a_well_formed_row_with_fixed_habit() (+12 more)

### Community 13 - "cell_storage.rs"
Cohesion: 0.19
Nodes (23): cell_kind(), CellKind, decode_cell(), DecodedCell, decodes_a_bool_cell(), decodes_a_date_cell_relative_to_the_2001_epoch(), decodes_a_duration_cell_as_minutes(), decodes_a_negative_fractional_number() (+15 more)

### Community 14 - "tauri.conf.json"
Cohesion: 0.09
Nodes (22): icons/128x128@2x.png, icons/128x128.png, icons/32x32.png, icons/icon.icns, icons/icon.ico, app, security, windows (+14 more)

### Community 15 - "repository.rs"
Cohesion: 0.05
Nodes (47): Into, export_entries(), export_resolves_habit_names_and_counts_rows(), ExportFormat, Path, Result, day_range(), Date (+39 more)

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

### Community 24 - "permissions"
Cohesion: 0.17
Nodes (11): core:default, dialog:allow-message, dialog:allow-open, dialog:allow-save, main, opener:default, description, identifier (+3 more)

### Community 26 - "package.json"
Cohesion: 0.33
Nodes (5): description, license, name, type, version

### Community 28 - "XlsxSource"
Cohesion: 0.18
Nodes (10): Data, cell_from_calamine(), AsRef, CellValue, Path, PathBuf, Result, Self (+2 more)

### Community 39 - "navigation.ts"
Cohesion: 0.29
Nodes (6): CaretState, CellPosition, GridBounds, nextCell(), bounds, midCaret

### Community 40 - "settings_repository.rs"
Cohesion: 0.11
Nodes (25): Default, SettingsRepository, Option, Self, String, Settings, StartupRoute, WeekStart (+17 more)

### Community 41 - ".sheets"
Cohesion: 0.18
Nodes (10): CsvSource, infer_cell(), AsRef, CellValue, Path, PathBuf, Result, Self (+2 more)

### Community 48 - "Self"
Cohesion: 0.19
Nodes (13): Entry, EntryDto, Profile, ProfileDto, Entry, Error, Profile, Result (+5 more)

### Community 49 - "import_workbook.rs"
Cohesion: 0.37
Nodes (12): Box, apply_import(), column_habit_mapping_creates_habits_by_name(), detect_source(), imports_a_csv_with_a_fixed_habit_and_remembers_the_mapping(), preview_import(), Path, PathBuf (+4 more)

### Community 54 - "Sheet"
Cohesion: 0.27
Nodes (7): CellValue, Error, Option, PrimitiveDateTime, String, Vec, Sheet

### Community 55 - "TimelineResult"
Cohesion: 0.25
Nodes (8): HabitBreakdownItem, PeriodCompareResult, Option, Vec, TimelineResult, HabitBreakdownItemDto, PeriodCompareDto, TimelineDto

### Community 56 - "ImportPreview"
Cohesion: 0.28
Nodes (7): ImportPreview, ImportSummary, Option, String, Vec, RowError, ImportSummaryDto

## Knowledge Gaps
- **130 isolated node(s):** `name`, `version`, `description`, `type`, `dev` (+125 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 295 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **15 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Db` connect `Db` to `commands.rs`, `settings_repository.rs`, `SqliteHabitRepository`, `repository.rs`, `entry_repository.rs`?**
  _High betweenness centrality (0.299) - this node is a cross-community bridge._
- **Why does `Sheet` connect `Sheet` to `IwaError`, `dto.rs`, `.sheets`, `mapping.rs`, `import_workbook.rs`, `ImportPreview`, `XlsxSource`?**
  _High betweenness centrality (0.188) - this node is a cross-community bridge._
- **Why does `read_table()` connect `IwaError` to `cell_storage.rs`, `Sheet`?**
  _High betweenness centrality (0.133) - this node is a cross-community bridge._
- **Are the 7 inferred relationships involving `HabitId` (e.g. with `habit()` and `money_spent_scales_with_quantity()`) actually correct?**
  _`HabitId` has 7 INFERRED edges - model-reasoned connections that need verification._
- **What connects `name`, `version`, `description` to the rest of the system?**
  _130 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `IwaError` be split into smaller, more focused modules?**
  _Cohesion score 0.05329593267882188 - nodes in this community are weakly interconnected._
- **Should `ipc/index.ts` be split into smaller, more focused modules?**
  _Cohesion score 0.06426906426906427 - nodes in this community are weakly interconnected._