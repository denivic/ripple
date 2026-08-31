import { describe, expect, it } from "vitest";
import { CommandStack } from "./command-stack";
import { DeleteRowsCommand, InsertRowCommand, PatchRowsCommand, type GridModel } from "./commands";
import type { GridRow } from "./types";

function row(id: number, quantity = 1): GridRow {
  return { id, habitId: 1, occurredAt: new Date(2026, 0, 1), quantity, durationMinutes: null, note: null };
}

function snapshot(model: GridModel): GridRow[] {
  return model.rows.map((r) => ({ ...r }));
}

describe("CommandStack", () => {
  it("undo undoes the last command and redo replays it (undo then redo = identity)", () => {
    const model: GridModel = { rows: [row(1, 5)] };
    const before = snapshot(model);
    const stack = new CommandStack();

    stack.do(model, new PatchRowsCommand([{ rowId: 1, before: { quantity: 5 }, after: { quantity: 9 } }]));
    expect(model.rows[0].quantity).toBe(9);

    stack.undo(model);
    expect(model.rows).toEqual(before);

    stack.redo(model);
    expect(model.rows[0].quantity).toBe(9);
  });

  it("clears the redo stack once a new command is done", () => {
    const model: GridModel = { rows: [row(1, 1)] };
    const stack = new CommandStack();

    stack.do(model, new PatchRowsCommand([{ rowId: 1, before: { quantity: 1 }, after: { quantity: 2 } }]));
    stack.undo(model);
    expect(stack.canRedo).toBe(true);

    stack.do(model, new PatchRowsCommand([{ rowId: 1, before: { quantity: 1 }, after: { quantity: 3 } }]));
    expect(stack.canRedo).toBe(false);
    expect(model.rows[0].quantity).toBe(3);
  });

  it("undo/redo on an empty stack is a no-op", () => {
    const model: GridModel = { rows: [row(1)] };
    const stack = new CommandStack();
    expect(() => stack.undo(model)).not.toThrow();
    expect(() => stack.redo(model)).not.toThrow();
    expect(stack.canUndo).toBe(false);
    expect(stack.canRedo).toBe(false);
  });

  it("round-trips an insert through undo and redo", () => {
    const model: GridModel = { rows: [row(1)] };
    const before = snapshot(model);
    const stack = new CommandStack();

    stack.do(model, new InsertRowCommand(1, row(2)));
    expect(model.rows.map((r) => r.id)).toEqual([1, 2]);

    stack.undo(model);
    expect(model.rows).toEqual(before);

    stack.redo(model);
    expect(model.rows.map((r) => r.id)).toEqual([1, 2]);
  });

  it("round-trips a multi-row delete through undo, preserving original positions", () => {
    const model: GridModel = { rows: [row(1), row(2), row(3)] };
    const before = snapshot(model);
    const stack = new CommandStack();

    stack.do(model, new DeleteRowsCommand([1, 3]));
    expect(model.rows.map((r) => r.id)).toEqual([2]);

    stack.undo(model);
    expect(model.rows).toEqual(before);

    stack.redo(model);
    expect(model.rows.map((r) => r.id)).toEqual([2]);
  });

  it("redoes a delete correctly after an intervening undo shifts indices back", () => {
    // Regression case for recomputing indices on every do() rather than
    // caching them once: delete row 2, undo (row 2 is back at index 1),
    // redo must find it there again, not at whatever index it had — if any —
    // from a stale first computation.
    const model: GridModel = { rows: [row(1), row(2), row(3)] };
    const stack = new CommandStack();

    stack.do(model, new DeleteRowsCommand([2]));
    stack.undo(model);
    stack.redo(model);
    expect(model.rows.map((r) => r.id)).toEqual([1, 3]);
  });
});
