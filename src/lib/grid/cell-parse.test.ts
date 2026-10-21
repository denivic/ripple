import { afterEach, describe, expect, it } from "vitest";
import { setTimeFormatSource } from "../charts/format";
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

  function hourMinute(text: string): [number, number] | null {
    const result = parseCell("occurredAt", text, habits);
    if (!result.ok) return null;
    const d = result.value as Date;
    return [d.getHours(), d.getMinutes()];
  }

  it("accepts 24-hour and 12-hour times regardless of the setting", () => {
    for (const format of ["24h", "12h"] as const) {
      setTimeFormatSource(() => format);
      expect(hourMinute("2026-03-14 13:05")).toEqual([13, 5]);
      expect(hourMinute("2026-03-14 1:05 PM")).toEqual([13, 5]);
      expect(hourMinute("2026-03-14 1:05pm")).toEqual([13, 5]);
      expect(hourMinute("3/14/2026 1:05 PM")).toEqual([13, 5]);
    }
    setTimeFormatSource(() => "24h");
  });

  it("maps 12 AM to midnight and 12 PM to noon", () => {
    expect(hourMinute("2026-03-14 12:00 AM")).toEqual([0, 0]);
    expect(hourMinute("2026-03-14 12:30 PM")).toEqual([12, 30]);
    expect(hourMinute("2026-03-14 9:15 a.m.")).toEqual([9, 15]);
  });

  it("rejects a 12-hour time with an out-of-range hour", () => {
    expect(parseCell("occurredAt", "2026-03-14 13:05 PM", habits).ok).toBe(false);
    expect(parseCell("occurredAt", "2026-03-14 0:05 AM", habits).ok).toBe(false);
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
  afterEach(() => setTimeFormatSource(() => "24h"));

  it("occurredAt", () => {
    const date = new Date(2026, 2, 14, 8, 5);
    const text = formatCell("occurredAt", date, null);
    expect(text).toBe("2026-03-14 08:05");
    const parsed = parseCell("occurredAt", text, habits);
    expect(parsed).toEqual({ ok: true, value: date });
  });

  it("occurredAt in 12-hour mode", () => {
    setTimeFormatSource(() => "12h");
    const date = new Date(2026, 2, 14, 20, 5);
    const text = formatCell("occurredAt", date, null);
    expect(text).toBe("2026-03-14 8:05 PM");
    expect(parseCell("occurredAt", text, habits)).toEqual({ ok: true, value: date });
  });

  it("quantity", () => {
    const text = formatCell("quantity", 3, null);
    expect(parseCell("quantity", text, habits)).toEqual({ ok: true, value: 3 });
  });
});
