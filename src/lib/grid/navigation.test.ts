import { describe, expect, it } from "vitest";
import { nextCell } from "./navigation";

const bounds = { rowCount: 3, colCount: 3 };
const midCaret = { atStart: false, atEnd: false };

describe("nextCell", () => {
  it("moves up or down regardless of caret position", () => {
    expect(nextCell("ArrowDown", { rowIndex: 0, colIndex: 1 }, midCaret, bounds)).toEqual({ rowIndex: 1, colIndex: 1 });
    expect(nextCell("ArrowUp", { rowIndex: 1, colIndex: 1 }, midCaret, bounds)).toEqual({ rowIndex: 0, colIndex: 1 });
  });

  it("clamps vertical movement at the first and last row", () => {
    expect(nextCell("ArrowUp", { rowIndex: 0, colIndex: 0 }, midCaret, bounds)).toBeNull();
    expect(nextCell("ArrowDown", { rowIndex: 2, colIndex: 0 }, midCaret, bounds)).toBeNull();
  });

  it("moves left or right only when the caret is at that edge of the cell text", () => {
    expect(nextCell("ArrowLeft", { rowIndex: 1, colIndex: 1 }, midCaret, bounds)).toBeNull();
    expect(nextCell("ArrowLeft", { rowIndex: 1, colIndex: 1 }, { atStart: true, atEnd: false }, bounds)).toEqual({
      rowIndex: 1,
      colIndex: 0,
    });
    expect(nextCell("ArrowRight", { rowIndex: 1, colIndex: 1 }, midCaret, bounds)).toBeNull();
    expect(nextCell("ArrowRight", { rowIndex: 1, colIndex: 1 }, { atStart: false, atEnd: true }, bounds)).toEqual({
      rowIndex: 1,
      colIndex: 2,
    });
  });

  it("clamps horizontal movement at the first and last column", () => {
    expect(nextCell("ArrowLeft", { rowIndex: 0, colIndex: 0 }, { atStart: true, atEnd: false }, bounds)).toBeNull();
    expect(nextCell("ArrowRight", { rowIndex: 0, colIndex: 2 }, { atStart: false, atEnd: true }, bounds)).toBeNull();
  });

  it("ignores keys other than the four arrow keys", () => {
    expect(nextCell("Enter", { rowIndex: 1, colIndex: 1 }, midCaret, bounds)).toBeNull();
    expect(nextCell("Tab", { rowIndex: 1, colIndex: 1 }, { atStart: true, atEnd: true }, bounds)).toBeNull();
  });
});
