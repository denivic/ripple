export interface CellPosition {
  rowIndex: number;
  colIndex: number;
}

export interface CaretState {
  atStart: boolean;
  atEnd: boolean;
}

export interface GridBounds {
  rowCount: number;
  colCount: number;
}

/** Resolves an arrow-key press into the cell it should move focus to, or
 * `null` if the key isn't a navigation key, the target is out of bounds, or
 * (for Left/Right) the caret isn't yet at that edge of the cell's text — a
 * single-line input never uses vertical caret motion, so Up/Down always
 * move a row, but Left/Right only hand off once normal in-cell caret
 * movement has nowhere further to go. */
export function nextCell(key: string, current: CellPosition, caret: CaretState, bounds: GridBounds): CellPosition | null {
  let target: CellPosition;
  if (key === "ArrowUp") target = { rowIndex: current.rowIndex - 1, colIndex: current.colIndex };
  else if (key === "ArrowDown") target = { rowIndex: current.rowIndex + 1, colIndex: current.colIndex };
  else if (key === "ArrowLeft" && caret.atStart) target = { rowIndex: current.rowIndex, colIndex: current.colIndex - 1 };
  else if (key === "ArrowRight" && caret.atEnd) target = { rowIndex: current.rowIndex, colIndex: current.colIndex + 1 };
  else return null;

  if (target.rowIndex < 0 || target.rowIndex >= bounds.rowCount) return null;
  if (target.colIndex < 0 || target.colIndex >= bounds.colCount) return null;
  return target;
}
