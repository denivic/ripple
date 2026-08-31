/** In-memory row shape the Ledger grid edits directly. Mirrors `Entry`
 * (see $lib/ipc/models.ts) plus a display-only `habitName` — the grid never
 * stores a second source of truth for habit data, it just reads through. */
export interface GridRow {
  /** Negative for a row created in the grid but not yet persisted. */
  id: number;
  habitId: number;
  occurredAt: Date;
  quantity: number;
  durationMinutes: number | null;
  note: string | null;
}

export type EditableColumnKey = "habitId" | "occurredAt" | "quantity" | "durationMinutes" | "note";

export type CellValue = GridRow[EditableColumnKey];

/** The columns a cell edit, paste, or fill-down can target. `occurredAt` and
 * `duration`/`note` visibility toggles are handled by the grid component;
 * this list is the full editable surface regardless of what's shown. */
export const EDITABLE_COLUMNS: readonly EditableColumnKey[] = [
  "habitId",
  "occurredAt",
  "quantity",
  "durationMinutes",
  "note",
];
