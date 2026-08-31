import { describe, expect, it } from "vitest";
import { formatCell, parseCell, type HabitLookup } from "./cell-parse";

const habits: HabitLookup = {
  idByName: (name) => (name === "Cigarettes" ? 1 : null),
};

describe("parseCell: habitId", () => {
  it("resolves a known habit name to its id", () => {
    const result = parseCell("habitId", "Cigarettes", habits);
    expect(result).toEqual({ ok: true, value: 1 });
  });

  it("errors on an unknown habit name", () => {
    const result = parseCell("habitId", "Nope", habits);
    expect(result.ok).toBe(false);
  });

  it("errors on empty input", () => {
    expect(parseCell("habitId", "  ", habits).ok).toBe(false);
  });
});

describe("parseCell: occurredAt", () => {
  it("parses ISO-like date and time", () => {
    const result = parseCell("occurredAt", "2026-03-14 08:05", habits);
    expect(result.ok).toBe(true);
    if (result.ok) {
      const d = result.value as Date;
      expect([d.getFullYear(), d.getMonth(), d.getDate(), d.getHours(), d.getMinutes()]).toEqual([2026, 2, 14, 8, 5]);
    }
  });

  it("parses a date-only value at local midnight", () => {
    const result = parseCell("occurredAt", "2026-03-14", habits);
    expect(result.ok).toBe(true);
    if (result.ok) expect((result.value as Date).getHours()).toBe(0);
  });

  it("parses slash-style dates", () => {
    const result = parseCell("occurredAt", "3/14/2026", habits);
    expect(result.ok).toBe(true);
    if (result.ok) {
      const d = result.value as Date;
      expect([d.getFullYear(), d.getMonth(), d.getDate()]).toEqual([2026, 2, 14]);
    }
  });

  it("errors on unparseable text", () => {
    expect(parseCell("occurredAt", "not a date", habits).ok).toBe(false);
  });
});

describe("parseCell: quantity", () => {
  it("parses a positive number", () => {
    expect(parseCell("quantity", "2.5", habits)).toEqual({ ok: true, value: 2.5 });
  });

  it("rejects negative quantities", () => {
    expect(parseCell("quantity", "-1", habits).ok).toBe(false);
  });

  it("rejects non-numeric text", () => {
    expect(parseCell("quantity", "abc", habits).ok).toBe(false);
  });
});

describe("parseCell: durationMinutes", () => {
  it("treats blank as null (optional field)", () => {
    expect(parseCell("durationMinutes", "", habits)).toEqual({ ok: true, value: null });
  });

  it("parses a number", () => {
    expect(parseCell("durationMinutes", "15", habits)).toEqual({ ok: true, value: 15 });
  });
});

describe("parseCell: note", () => {
  it("treats blank as null", () => {
    expect(parseCell("note", "   ", habits)).toEqual({ ok: true, value: null });
  });

  it("passes through non-blank text verbatim", () => {
    expect(parseCell("note", "late night", habits)).toEqual({ ok: true, value: "late night" });
  });
});

describe("formatCell round-trips parseCell for canonical values", () => {
  it("occurredAt", () => {
    const date = new Date(2026, 2, 14, 8, 5);
    const text = formatCell("occurredAt", date, null);
    const parsed = parseCell("occurredAt", text, habits);
    expect(parsed).toEqual({ ok: true, value: date });
  });

  it("quantity", () => {
    const text = formatCell("quantity", 3, null);
    expect(parseCell("quantity", text, habits)).toEqual({ ok: true, value: 3 });
  });
});
