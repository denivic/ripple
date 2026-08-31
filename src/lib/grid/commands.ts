import type { GridRow } from "./types";

export interface GridModel {
  rows: GridRow[];
}

/** Command pattern for the grid's undo/redo stack (see CommandStack) — kept
 * framework-agnostic so it can be unit tested without a Svelte runtime, per
 * plan-v1.md's "lives in the frontend so keystrokes never round-trip IPC". */
export interface GridCommand {
  readonly label: string;
  do(model: GridModel): void;
  undo(model: GridModel): void;
}

function findRow(model: GridModel, rowId: number): GridRow | undefined {
  return model.rows.find((r) => r.id === rowId);
}

export interface RowPatch {
  rowId: number;
  before: Partial<GridRow>;
  after: Partial<GridRow>;
}

/** Covers a single cell edit, a fill-down, and a multi-cell paste alike —
 * all three are "some rows had some fields change," just with different
 * patch counts. */
export class PatchRowsCommand implements GridCommand {
  readonly label: string;
  #patches: RowPatch[];

  constructor(patches: RowPatch[], label = "Edit") {
    this.#patches = patches;
    this.label = label;
  }

  do(model: GridModel): void {
    for (const { rowId, after } of this.#patches) {
      const row = findRow(model, rowId);
      if (row) Object.assign(row, after);
    }
  }

  undo(model: GridModel): void {
    for (const { rowId, before } of this.#patches) {
      const row = findRow(model, rowId);
      if (row) Object.assign(row, before);
    }
  }
}

export class InsertRowCommand implements GridCommand {
  readonly label = "Insert row";
  #index: number;
  #row: GridRow;

  constructor(index: number, row: GridRow) {
    this.#index = index;
    this.#row = row;
  }

  do(model: GridModel): void {
    model.rows.splice(this.#index, 0, this.#row);
  }

  undo(model: GridModel): void {
    const i = model.rows.findIndex((r) => r.id === this.#row.id);
    if (i !== -1) model.rows.splice(i, 1);
  }
}

interface RemovedRow {
  index: number;
  row: GridRow;
}

export class DeleteRowsCommand implements GridCommand {
  readonly label = "Delete rows";
  #rowIds: number[];
  #removed: RemovedRow[] = [];

  constructor(rowIds: number[]) {
    this.#rowIds = rowIds;
  }

  do(model: GridModel): void {
    // Recomputed on every do() (including redo) rather than cached once, so
    // a redo after an intervening undo still finds the rows at their
    // current — not their original — indices.
    this.#removed = this.#rowIds
      .map((id) => {
        const index = model.rows.findIndex((r) => r.id === id);
        return index === -1 ? null : { index, row: model.rows[index] };
      })
      .filter((r): r is RemovedRow => r !== null)
      .sort((a, b) => b.index - a.index);
    for (const { index } of this.#removed) model.rows.splice(index, 1);
  }

  undo(model: GridModel): void {
    const ascending = [...this.#removed].sort((a, b) => a.index - b.index);
    for (const { index, row } of ascending) model.rows.splice(index, 0, row);
  }
}
