import { describe, expect, it } from "vitest";
import { formatLocalDate, formatLocalDateTime, parseLocalDate, parseLocalDateTime } from "./datetime";

describe("local date round-trip", () => {
  it("formats a date as YYYY-MM-DD with zero padding", () => {
    expect(formatLocalDate(new Date(2026, 2, 4))).toBe("2026-03-04");
  });

  it("parses YYYY-MM-DD at local midnight, not UTC", () => {
    const date = parseLocalDate("2026-03-14");
    expect(date.getFullYear()).toBe(2026);
    expect(date.getMonth()).toBe(2);
    expect(date.getDate()).toBe(14);
    expect(date.getHours()).toBe(0);
  });

  it("round-trips through format then parse", () => {
    const original = new Date(2026, 11, 31);
    expect(parseLocalDate(formatLocalDate(original))).toEqual(original);
  });
});

describe("local datetime round-trip", () => {
  it("formats with zero-padded time components", () => {
    expect(formatLocalDateTime(new Date(2026, 2, 4, 8, 5, 9))).toBe("2026-03-04T08:05:09");
  });

  it("parses date and time components exactly, not via UTC", () => {
    const date = parseLocalDateTime("2026-03-14T23:59:01");
    expect(date.getHours()).toBe(23);
    expect(date.getMinutes()).toBe(59);
    expect(date.getSeconds()).toBe(1);
  });

  it("round-trips through format then parse", () => {
    const original = new Date(2026, 5, 15, 14, 22, 33);
    expect(parseLocalDateTime(formatLocalDateTime(original))).toEqual(original);
  });
});
