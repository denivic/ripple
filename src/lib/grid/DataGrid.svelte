<script lang="ts">
  import { formatCell, parseCell, type HabitLookup } from "./cell-parse";
  import type { RowPatch } from "./commands";
  import { buildFillDownPatches, buildPastePatches } from "./tsv";
  import type { CellValue, EditableColumnKey, GridRow } from "./types";

  const ROW_HEIGHT = 32;
  const OVERSCAN = 8;

  interface ColumnConfig {
    key: EditableColumnKey;
    label: string;
  }

  interface Props {
    rows: GridRow[];
    habits: HabitLookup;
    habitNames: string[];
    habitNameById: (id: number) => string | null;
    showDuration: boolean;
    showNote: boolean;
    onPatch: (patches: RowPatch[], label?: string) => void;
    onDeleteRows: (rowIds: number[]) => void;
    onInsertRow: () => void;
  }

  let { rows, habits, habitNames, habitNameById, showDuration, showNote, onPatch, onDeleteRows, onInsertRow }: Props =
    $props();

  const BASE_COLUMNS: ColumnConfig[] = [
    { key: "habitId", label: "Habit" },
    { key: "occurredAt", label: "When" },
    { key: "quantity", label: "Qty" },
  ];
  const OPTIONAL_COLUMNS: Record<"durationMinutes" | "note", ColumnConfig> = {
    durationMinutes: { key: "durationMinutes", label: "Duration (min)" },
    note: { key: "note", label: "Note" },
  };
  const visibleColumns = $derived([
    ...BASE_COLUMNS,
    ...(showDuration ? [OPTIONAL_COLUMNS.durationMinutes] : []),
    ...(showNote ? [OPTIONAL_COLUMNS.note] : []),
  ]);
  const gridCols = $derived(`1.75rem repeat(${visibleColumns.length}, minmax(6rem, 1fr))`);

  let sortColumn = $state<EditableColumnKey>("occurredAt");
  let sortDirection = $state<1 | -1>(1);
  let filterText = $state("");
  let filterHabit = $state("");

  function cellSortValue(row: GridRow, column: EditableColumnKey): string | number {
    if (column === "habitId") return habitNameById(row.habitId)?.toLowerCase() ?? "";
    const v = row[column];
    if (v instanceof Date) return v.getTime();
    if (typeof v === "number") return v;
    return (v ?? "").toString().toLowerCase();
  }

  const displayRows = $derived.by(() => {
    let list = rows;
    if (filterHabit) list = list.filter((r) => habitNameById(r.habitId) === filterHabit);
    const needle = filterText.trim().toLowerCase();
    if (needle) {
      list = list.filter(
        (r) => (habitNameById(r.habitId) ?? "").toLowerCase().includes(needle) || (r.note ?? "").toLowerCase().includes(needle),
      );
    }
    return [...list].sort((a, b) => {
      const av = cellSortValue(a, sortColumn);
      const bv = cellSortValue(b, sortColumn);
      const cmp = typeof av === "number" && typeof bv === "number" ? av - bv : String(av).localeCompare(String(bv));
      return cmp * sortDirection;
    });
  });

  let viewport: HTMLDivElement | undefined = $state();
  let scrollTop = $state(0);
  let viewportHeight = $state(320);

  $effect(() => {
    if (!viewport) return;
    const el = viewport;
    const observer = new ResizeObserver(() => (viewportHeight = el.clientHeight));
    observer.observe(el);
    viewportHeight = el.clientHeight;
    return () => observer.disconnect();
  });

  const totalHeight = $derived(displayRows.length * ROW_HEIGHT);
  const startIndex = $derived(Math.max(0, Math.floor(scrollTop / ROW_HEIGHT) - OVERSCAN));
  const endIndex = $derived(Math.min(displayRows.length, Math.ceil((scrollTop + viewportHeight) / ROW_HEIGHT) + OVERSCAN));
  const visibleRows = $derived(displayRows.slice(startIndex, endIndex));

  function toggleSort(column: EditableColumnKey): void {
    if (sortColumn === column) sortDirection = sortDirection === 1 ? -1 : 1;
    else {
      sortColumn = column;
      sortDirection = 1;
    }
  }

  let selected = $state<Set<number>>(new Set());
  let anchorId: number | null = $state(null);

  function selectRow(rowId: number, e: MouseEvent): void {
    if (e.shiftKey && anchorId !== null) {
      const ids = displayRows.map((r) => r.id);
      const a = ids.indexOf(anchorId);
      const b = ids.indexOf(rowId);
      if (a !== -1 && b !== -1) {
        const [lo, hi] = a < b ? [a, b] : [b, a];
        selected = new Set(ids.slice(lo, hi + 1));
        return;
      }
    }
    if (e.metaKey || e.ctrlKey) {
      const next = new Set(selected);
      next.has(rowId) ? next.delete(rowId) : next.add(rowId);
      selected = next;
    } else {
      selected = new Set([rowId]);
    }
    anchorId = rowId;
  }

  function deleteSelected(): void {
    if (selected.size === 0) return;
    onDeleteRows([...selected]);
    selected = new Set();
  }

  function fillDownActiveColumn(): void {
    if (selected.size < 2) return;
    const idsInOrder = displayRows.map((r) => r.id).filter((id) => selected.has(id));
    onPatch(buildFillDownPatches(rows, idsInOrder, sortColumn), "Fill down");
  }

  let cellErrors = $state<Map<string, string>>(new Map());

  function cellKey(rowId: number, column: EditableColumnKey): string {
    return `${rowId}:${column}`;
  }

  function setCellError(rowId: number, column: EditableColumnKey, message: string | null): void {
    const next = new Map(cellErrors);
    if (message) next.set(cellKey(rowId, column), message);
    else next.delete(cellKey(rowId, column));
    cellErrors = next;
  }

  function commit(row: GridRow, column: EditableColumnKey, text: string, input: HTMLInputElement): void {
    const result = parseCell(column, text, habits);
    if (!result.ok) {
      setCellError(row.id, column, result.error);
      return;
    }
    setCellError(row.id, column, null);
    const current = row[column] as CellValue;
    const unchanged =
      current === result.value ||
      (current instanceof Date && result.value instanceof Date && current.getTime() === result.value.getTime());
    if (unchanged) {
      input.value = formatCell(column, current, column === "habitId" ? habitNameById(row.habitId) : null);
      return;
    }
    const before: Partial<GridRow> = {};
    const after: Partial<GridRow> = {};
    (before as Record<string, unknown>)[column] = current;
    (after as Record<string, unknown>)[column] = result.value;
    onPatch([{ rowId: row.id, before, after }]);
  }

  function handleKeydown(e: KeyboardEvent, row: GridRow, column: EditableColumnKey): void {
    const input = e.currentTarget as HTMLInputElement;
    if (e.key === "Escape") {
      input.value = formatCell(column, row[column] as CellValue, column === "habitId" ? habitNameById(row.habitId) : null);
      setCellError(row.id, column, null);
      input.blur();
    } else if (e.key === "Enter") {
      input.blur();
    }
  }

  function handlePaste(e: ClipboardEvent, row: GridRow, column: EditableColumnKey): void {
    const text = e.clipboardData?.getData("text/plain") ?? "";
    if (!text.includes("\t") && !text.includes("\n")) return;
    e.preventDefault();
    const rowIndex = displayRows.findIndex((r) => r.id === row.id);
    const columnIndex = visibleColumns.findIndex((c) => c.key === column);
    const outcome = buildPastePatches(text, {
      columns: visibleColumns.map((c) => c.key),
      startColumnIndex: columnIndex,
      rows: displayRows,
      startRowIndex: rowIndex,
      habits,
    });
    if (outcome.patches.length > 0) onPatch(outcome.patches, "Paste");
    const notes: string[] = [];
    if (outcome.errors.length > 0) notes.push(`${outcome.errors.length} cell${outcome.errors.length === 1 ? "" : "s"} rejected`);
    if (outcome.overflowRows > 0) notes.push(`${outcome.overflowRows} row${outcome.overflowRows === 1 ? "" : "s"} past the end ignored`);
    if (notes.length > 0) setCellError(row.id, column, notes.join(", "));
  }
</script>

<div class="toolbar">
  <input class="search" type="search" placeholder="Filter by habit or note…" bind:value={filterText} />
  <select class="habit-filter" bind:value={filterHabit}>
    <option value="">All habits</option>
    {#each habitNames as name (name)}
      <option value={name}>{name}</option>
    {/each}
  </select>
  <div class="spacer"></div>
  <button type="button" onclick={fillDownActiveColumn} disabled={selected.size < 2}>Fill down</button>
  <button type="button" onclick={deleteSelected} disabled={selected.size === 0}>
    Delete{selected.size > 0 ? ` (${selected.size})` : ""}
  </button>
  <button type="button" onclick={onInsertRow}>Insert row</button>
</div>

<div class="header" style:grid-template-columns={gridCols}>
  <span class="handle-header"></span>
  {#each visibleColumns as col (col.key)}
    <button type="button" class="header-cell" onclick={() => toggleSort(col.key)}>
      {col.label}
      {#if sortColumn === col.key}<span class="sort-arrow">{sortDirection === 1 ? "▲" : "▼"}</span>{/if}
    </button>
  {/each}
</div>

<div class="viewport" bind:this={viewport} onscroll={() => (scrollTop = viewport?.scrollTop ?? 0)}>
  <div class="scroll-space" style:height="{totalHeight}px">
    <div class="rows" style:transform="translateY({startIndex * ROW_HEIGHT}px)" style:grid-template-columns={gridCols}>
      {#each visibleRows as row (row.id)}
        <div class="row">
          <button
            type="button"
            class="handle"
            class:selected={selected.has(row.id)}
            onclick={(e) => selectRow(row.id, e)}
            aria-label="Select row"
            aria-pressed={selected.has(row.id)}
          ></button>
          {#each visibleColumns as col (col.key)}
            {@const error = cellErrors.get(cellKey(row.id, col.key))}
            <input
              class="cell"
              class:invalid={!!error}
              class:row-selected={selected.has(row.id)}
              title={error ?? ""}
              list={col.key === "habitId" ? "ledger-habit-names" : undefined}
              value={formatCell(col.key, row[col.key] as CellValue, col.key === "habitId" ? habitNameById(row.habitId) : null)}
              onchange={(e) => commit(row, col.key, (e.currentTarget as HTMLInputElement).value, e.currentTarget)}
              onkeydown={(e) => handleKeydown(e, row, col.key)}
              onpaste={(e) => handlePaste(e, row, col.key)}
            />
          {/each}
        </div>
      {/each}
      {#if displayRows.length === 0}
        <p class="empty">No entries match.</p>
      {/if}
    </div>
  </div>
</div>

<datalist id="ledger-habit-names">
  {#each habitNames as name (name)}
    <option value={name}></option>
  {/each}
</datalist>

<style>
  .toolbar {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.75rem 1rem;
    border-bottom: 1px solid var(--line-hairline);
  }
  .search {
    background: var(--surface-raised);
    border: 1px solid var(--line-hairline);
    border-radius: 0.5rem;
    padding: 0.4rem 0.625rem;
    color: var(--text-primary);
    font-size: 0.875rem;
    width: 14rem;
  }
  .habit-filter {
    background: var(--surface-raised);
    border: 1px solid var(--line-hairline);
    border-radius: 0.5rem;
    padding: 0.4rem 0.625rem;
    color: var(--text-primary);
    font-size: 0.875rem;
  }
  .spacer {
    flex: 1;
  }
  .toolbar button {
    background: var(--surface-raised);
    border: 1px solid var(--line-hairline);
    border-radius: 0.5rem;
    padding: 0.4rem 0.75rem;
    color: var(--text-primary);
    font-size: 0.8125rem;
    cursor: pointer;
  }
  .toolbar button:disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }
  .toolbar button:not(:disabled):hover {
    background: var(--surface-hover);
  }

  .header {
    display: grid;
    gap: 1px;
    padding: 0 1rem;
    border-bottom: 1px solid var(--line-hairline);
  }
  .handle-header {
    width: 1.75rem;
  }
  .header-cell {
    display: flex;
    align-items: center;
    gap: 0.375rem;
    background: none;
    border: none;
    color: var(--text-secondary);
    font-size: 0.75rem;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.03em;
    padding: 0.5rem 0.375rem;
    cursor: pointer;
    text-align: left;
  }
  .header-cell:hover {
    color: var(--text-primary);
  }
  .sort-arrow {
    color: var(--accent-base);
    font-size: 0.625rem;
  }

  .viewport {
    flex: 1;
    min-height: 0;
    overflow-y: auto;
    padding: 0 1rem;
  }
  .scroll-space {
    position: relative;
  }
  .rows {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    display: grid;
    grid-auto-rows: 32px;
    gap: 1px;
  }
  .row {
    display: contents;
  }

  .handle {
    width: 1.75rem;
    height: 100%;
    background: none;
    border: none;
    border-radius: 0.25rem;
    cursor: pointer;
  }
  .handle::before {
    content: "";
    display: block;
    width: 0.5rem;
    height: 0.5rem;
    margin: auto;
    border-radius: 50%;
    border: 1px solid var(--line-hairline);
  }
  .handle.selected::before {
    background: var(--accent-base);
    border-color: var(--accent-base);
  }

  .cell {
    background: var(--surface-raised);
    border: 1px solid transparent;
    color: var(--text-primary);
    font-size: 0.8125rem;
    padding: 0 0.5rem;
    border-radius: 0.25rem;
    min-width: 0;
  }
  .cell.row-selected {
    background: var(--accent-translucent);
  }
  .cell:focus-visible {
    outline: none;
    border-color: var(--accent-base);
  }
  .cell.invalid {
    border-color: #f87171;
  }

  .empty {
    padding: 2rem;
    text-align: center;
    color: var(--text-tertiary);
  }
</style>
