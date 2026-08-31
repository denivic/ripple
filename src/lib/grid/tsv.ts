import { formatCell, parseCell, type HabitLookup } from "./cell-parse";
import type { RowPatch } from "./commands";
import type { CellValue, EditableColumnKey, GridRow } from "./types";

/** Splits clipboard text into a rectangular grid of cell strings. A trailing
 * newline (the common case when copying from a real spreadsheet) would
 * otherwise produce a spurious empty final row, so it's dropped. */
export function parseTsv(text: string): string[][] {
  const normalized = text.replace(/\r\n/g, "\n").replace(/\r/g, "\n");
  const lines = normalized.split("\n");
  if (lines.length > 1 && lines[lines.length - 1] === "") lines.pop();
  return lines.map((line) => line.split("\t"));
}

export function serializeTsv(
  rows: GridRow[],
  columns: EditableColumnKey[],
  habitNameById: (id: number) => string | null,
): string {
  return rows
    .map((row) =>
      columns
        .map((column) => formatCell(column, row[column] as CellValue, column === "habitId" ? habitNameById(row.habitId) : null))
        .join("\t"),
    )
    .join("\n");
}

export interface PasteContext {
  /** The editable columns in on-screen left-to-right order. */
  columns: EditableColumnKey[];
  startColumnIndex: number;
  /** Canonical rows in on-screen top-to-bottom order at paste time. */
  rows: GridRow[];
  startRowIndex: number;
  habits: HabitLookup;
}

export interface PasteError {
  rowIndex: number;
  column: EditableColumnKey;
  message: string;
}

export interface PasteOutcome {
  patches: RowPatch[];
  errors: PasteError[];
  /** Pasted rows past the end of the grid — silently extending the grid on
   * paste would blur "insert row" and "paste" into one operation, so these
   * are reported and dropped instead; the caller can surface a count. */
  overflowRows: number;
}

function setField(patch: Partial<GridRow>, column: EditableColumnKey, value: CellValue): void {
  (patch as Record<string, unknown>)[column] = value;
}

export function buildPastePatches(tsvText: string, ctx: PasteContext): PasteOutcome {
  const block = parseTsv(tsvText);
  const patches: RowPatch[] = [];
  const errors: PasteError[] = [];
  let overflowRows = 0;

  block.forEach((cells, r) => {
    const rowIndex = ctx.startRowIndex + r;
    const row = ctx.rows[rowIndex];
    if (!row) {
      overflowRows++;
      return;
    }
    const before: Partial<GridRow> = {};
    const after: Partial<GridRow> = {};
    let changed = false;
    cells.forEach((cellText, c) => {
      const column = ctx.columns[ctx.startColumnIndex + c];
      if (!column) return;
      const result = parseCell(column, cellText, ctx.habits);
      if (!result.ok) {
        errors.push({ rowIndex, column, message: result.error });
        return;
      }
      setField(before, column, row[column] as CellValue);
      setField(after, column, result.value);
      changed = true;
    });
    if (changed) patches.push({ rowId: row.id, before, after });
  });

  return { patches, errors, overflowRows };
}

/** Fills every row after the first (the anchor) with the anchor's value for
 * `column`. Rows already equal to the anchor are skipped so a fill-down over
 * an already-uniform range doesn't push a no-op command onto the undo stack. */
export function buildFillDownPatches(rows: GridRow[], rowIdsTopToBottom: number[], column: EditableColumnKey): RowPatch[] {
  if (rowIdsTopToBottom.length < 2) return [];
  const [anchorId, ...restIds] = rowIdsTopToBottom;
  const anchor = rows.find((r) => r.id === anchorId);
  if (!anchor) return [];
  const value = anchor[column] as CellValue;

  const patches: RowPatch[] = [];
  for (const id of restIds) {
    const row = rows.find((r) => r.id === id);
    if (!row || row[column] === value) continue;
    const before: Partial<GridRow> = {};
    const after: Partial<GridRow> = {};
    setField(before, column, row[column] as CellValue);
    setField(after, column, value);
    patches.push({ rowId: row.id, before, after });
  }
  return patches;
}
