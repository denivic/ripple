import { describe, expect, it } from "vitest";
import { cumulativeSum, forwardProjection, movingAverage, recentDailyRate } from "./series-math";

describe("cumulativeSum", () => {
  it("runs a total forward", () => {
    expect(cumulativeSum([1, 2, 3])).toEqual([1, 3, 6]);
  });

  it("returns an empty array for an empty input", () => {
    expect(cumulativeSum([])).toEqual([]);
  });
});

describe("movingAverage", () => {
  it("uses a partial window at the start of the series", () => {
    expect(movingAverage([10, 20], 7)).toEqual([10, 15]);
  });

  it("uses the full window once enough history exists", () => {
    expect(movingAverage([10, 20, 30, 40], 2)).toEqual([10, 15, 25, 35]);
  });
});

describe("recentDailyRate", () => {
  it("averages the trailing window", () => {
    expect(recentDailyRate([10, 20, 30], 2)).toBe(25);
  });

  it("is zero for an empty series", () => {
    expect(recentDailyRate([], 7)).toBe(0);
  });
});

describe("forwardProjection", () => {
  it("extrapolates linearly from the starting point", () => {
    const points = forwardProjection(new Date(2026, 0, 1), 100, 10, 3);
    expect(points).toHaveLength(4);
    expect(points[0].value).toBe(100);
    expect(points[3].value).toBe(130);
    expect(points[3].date.getDate()).toBe(4);
  });
});
