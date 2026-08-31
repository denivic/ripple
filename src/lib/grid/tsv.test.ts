import { describe, expect, it } from "vitest";
import type { HabitLookup } from "./cell-parse";
import { buildFillDownPatches, buildPastePatches, parseTsv, serializeTsv } from "./tsv";
import type { GridRow } from "./types";

const habits: HabitLookup = {
  idByName: (name) => (name === "Cigarettes" ? 1 : name === "Coffee" ? 2 : null),
};

function row(id: number, overrides: Partial<GridRow> = {}): GridRow {
  return {
    id,
    habitId: 1,
    occurredAt: new Date(2026, 0, id),
    quantity: 1,
    durationMinutes: null,
    note: null,
    ...overrides,
  };
}

describe("parseTsv", () => {
  it("splits rows and tab-separated cells", () => {
    expect(parseTsv("a\tb\nc\td")).toEqual([
      ["a", "b"],
      ["c", "d"],
    ]);
  });

  it("drops a single trailing newline", () => {
    expect(parseTsv("a\tb\n")).toEqual([["a", "b"]]);
  });

  it("normalizes CRLF line endings", () => {
    expect(parseTsv("a\tb\r\nc\td")).toEqual([
      ["a", "b"],
      ["c", "d"],
    ]);
  });
});

describe("buildPastePatches", () => {
  it("patches quantity and note across two rows from a 2-row paste", () => {
    const rows = [row(1), row(2)];
    const outcome = buildPastePatches("3\thello\n4\tworld", {
      columns: ["quantity", "note"],
      startColumnIndex: 0,
      rows,
      startRowIndex: 0,
      habits,
    });
    expect(outcome.errors).toEqual([]);
    expect(outcome.overflowRows).toBe(0);
    expect(outcome.patches).toEqual([
      { rowId: 1, before: { quantity: 1, note: null }, after: { quantity: 3, note: "hello" } },
      { rowId: 2, before: { quantity: 1, note: null }, after: { quantity: 4, note: "world" } },
    ]);
  });

  it("reports rows pasted past the end of the grid as overflow", () => {
    const rows = [row(1)];
    const outcome = buildPastePatches("1\n2\n3", {
      columns: ["quantity"],
      startColumnIndex: 0,
      rows,
      startRowIndex: 0,
      habits,
    });
    expect(outcome.patches).toHaveLength(1);
    expect(outcome.overflowRows).toBe(2);
  });

  it("collects a per-cell error without dropping the rest of the row's valid cells", () => {
    const rows = [row(1)];
    const outcome = buildPastePatches("not-a-number\thello", {
      columns: ["quantity", "note"],
      startColumnIndex: 0,
      rows,
      startRowIndex: 0,
      habits,
    });
    expect(outcome.errors).toEqual([{ rowIndex: 0, column: "quantity", message: "Expected a number" }]);
    expect(outcome.patches).toEqual([{ rowId: 1, before: { note: null }, after: { note: "hello" } }]);
  });

  it("resolves a habit-name paste through the habit lookup", () => {
    const rows = [row(1, { habitId: 1 })];
    const outcome = buildPastePatches("Coffee", {
      columns: ["habitId"],
      startColumnIndex: 0,
      rows,
      startRowIndex: 0,
      habits,
    });
    expect(outcome.patches).toEqual([{ rowId: 1, before: { habitId: 1 }, after: { habitId: 2 } }]);
  });
});

describe("buildFillDownPatches", () => {
  it("copies the anchor row's value down to the rest of the selection", () => {
    const rows = [row(1, { quantity: 9 }), row(2, { quantity: 1 }), row(3, { quantity: 2 })];
    const patches = buildFillDownPatches(rows, [1, 2, 3], "quantity");
    expect(patches).toEqual([
      { rowId: 2, before: { quantity: 1 }, after: { quantity: 9 } },
      { rowId: 3, before: { quantity: 2 }, after: { quantity: 9 } },
    ]);
  });

  it("skips rows already equal to the anchor to avoid no-op patches", () => {
    const rows = [row(1, { quantity: 9 }), row(2, { quantity: 9 })];
    expect(buildFillDownPatches(rows, [1, 2], "quantity")).toEqual([]);
  });

  it("is a no-op for a single-row selection", () => {
    const rows = [row(1)];
    expect(buildFillDownPatches(rows, [1], "quantity")).toEqual([]);
  });
});

describe("serializeTsv", () => {
  it("formats rows into tab/newline separated cells, resolving habit names", () => {
    const rows = [row(1, { habitId: 1, quantity: 2 })];
    const text = serializeTsv(rows, ["habitId", "quantity"], (id) => (id === 1 ? "Cigarettes" : null));
    expect(text).toBe("Cigarettes\t2");
  });
});
