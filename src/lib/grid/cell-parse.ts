import type { CellValue, EditableColumnKey } from "./types";

/** Parses free-typed or pasted cell text into a typed value, or reports why
 * it can't — the source of the grid's inline validation. Deliberately more
 * lenient on input than the formatters are on output (a pasted "3/14/2026"
 * from a spreadsheet should work, not just this app's own canonical format),
 * mirroring the tolerance of the Rust import mapper's own flexible parser
 * (src-tauri/src/infrastructure/import/mapping.rs::parse_flexible_datetime). */

export interface HabitLookup {
  idByName(name: string): number | null;
}

export type ParseResult = { ok: true; value: CellValue } | { ok: false; error: string };

function pad(n: number): string {
  return String(n).padStart(2, "0");
}

/** Display string for a cell value — the inverse of `parseCell` for the same
 * column, so committing an unedited cell round-trips exactly. */
export function formatCell(column: EditableColumnKey, value: CellValue, habitName: string | null): string {
  switch (column) {
    case "habitId":
      return habitName ?? "";
    case "occurredAt": {
      const d = value as Date;
      return `${d.getFullYear()}-${pad(d.getMonth() + 1)}-${pad(d.getDate())} ${pad(d.getHours())}:${pad(d.getMinutes())}`;
    }
    case "quantity":
      return String(value);
    case "durationMinutes":
      return value === null ? "" : String(value);
    case "note":
      return (value as string | null) ?? "";
  }
}

function parseDateCell(text: string): Date | null {
  const trimmed = text.trim();
  const isoLike = trimmed.match(/^(\d{4})-(\d{2})-(\d{2})(?:[ T](\d{2}):(\d{2})(?::(\d{2}))?)?$/);
  if (isoLike) {
    const [, y, mo, d, h, mi, s] = isoLike;
    return new Date(Number(y), Number(mo) - 1, Number(d), Number(h ?? 0), Number(mi ?? 0), Number(s ?? 0));
  }
  const slashLike = trimmed.match(/^(\d{1,2})\/(\d{1,2})\/(\d{4})(?:\s+(\d{1,2}):(\d{2}))?$/);
  if (slashLike) {
    const [, mo, d, y, h, mi] = slashLike;
    return new Date(Number(y), Number(mo) - 1, Number(d), Number(h ?? 0), Number(mi ?? 0));
  }
  return null;
}

function parseNumberCell(text: string): number | null {
  const trimmed = text.trim();
  if (trimmed === "") return null;
  const n = Number(trimmed);
  return Number.isFinite(n) ? n : null;
}

export function parseCell(
  column: EditableColumnKey,
  text: string,
  habits: HabitLookup,
): ParseResult {
  switch (column) {
    case "habitId": {
      const name = text.trim();
      if (!name) return { ok: false, error: "Habit is required" };
      const id = habits.idByName(name);
      if (id === null) return { ok: false, error: `No habit named "${name}"` };
      return { ok: true, value: id };
    }
    case "occurredAt": {
      const date = parseDateCell(text);
      if (!date) return { ok: false, error: "Expected a date like 2026-03-14 08:05" };
      return { ok: true, value: date };
    }
    case "quantity": {
      const n = parseNumberCell(text);
      if (n === null) return { ok: false, error: "Expected a number" };
      if (n < 0) return { ok: false, error: "Quantity can't be negative" };
      return { ok: true, value: n };
    }
    case "durationMinutes": {
      if (text.trim() === "") return { ok: true, value: null };
      const n = parseNumberCell(text);
      if (n === null) return { ok: false, error: "Expected a number of minutes" };
      if (n < 0) return { ok: false, error: "Duration can't be negative" };
      return { ok: true, value: n };
    }
    case "note":
      return { ok: true, value: text.trim() === "" ? null : text };
  }
}
