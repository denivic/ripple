import { describe, expect, it } from "vitest";
import { formatClockValue, fromHour12, minuteOptions, parseClockValue, toHour12 } from "./time-field";

describe("parseClockValue", () => {
  it("splits a 24-hour value", () => {
    expect(parseClockValue("20:05")).toEqual({ hour: 20, minute: 5 });
    expect(parseClockValue("00:00")).toEqual({ hour: 0, minute: 0 });
  });

  it("rejects malformed or out-of-range values", () => {
    expect(parseClockValue("")).toBeNull();
    expect(parseClockValue("24:00")).toBeNull();
    expect(parseClockValue("08:60")).toBeNull();
    expect(parseClockValue("8 PM")).toBeNull();
  });
});

describe("formatClockValue", () => {
  it("zero-pads to HH:MM", () => {
    expect(formatClockValue(7, 5)).toBe("07:05");
    expect(formatClockValue(23, 55)).toBe("23:55");
  });
});

describe("12-hour conversion", () => {
  it("maps midnight and noon", () => {
    expect(toHour12(0)).toEqual({ hour12: 12, period: "AM" });
    expect(toHour12(12)).toEqual({ hour12: 12, period: "PM" });
    expect(fromHour12(12, "AM")).toBe(0);
    expect(fromHour12(12, "PM")).toBe(12);
  });

  it("round-trips every hour", () => {
    for (let hour = 0; hour < 24; hour++) {
      const { hour12, period } = toHour12(hour);
      expect(hour12).toBeGreaterThanOrEqual(1);
      expect(hour12).toBeLessThanOrEqual(12);
      expect(fromHour12(hour12, period)).toBe(hour);
    }
  });
});

describe("minuteOptions", () => {
  it("offers five-minute steps", () => {
    expect(minuteOptions(30)).toEqual([0, 5, 10, 15, 20, 25, 30, 35, 40, 45, 50, 55]);
  });

  it("keeps an off-step stored minute as an extra option, in order", () => {
    const options = minuteOptions(7);
    expect(options).toHaveLength(13);
    expect(options.slice(0, 4)).toEqual([0, 5, 7, 10]);
  });
});
