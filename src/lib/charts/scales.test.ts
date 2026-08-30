import { describe, expect, it } from "vitest";
import { seriesColorVar } from "./scales";

describe("seriesColorVar", () => {
  it("maps the first 8 indices to slots 1-8", () => {
    const vars = Array.from({ length: 8 }, (_, i) => seriesColorVar(i));
    expect(vars).toEqual([1, 2, 3, 4, 5, 6, 7, 8].map((n) => `var(--data-series-${n})`));
  });

  it("wraps back to slot 1 after 8 series, never generating a new hue", () => {
    expect(seriesColorVar(8)).toBe("var(--data-series-1)");
    expect(seriesColorVar(9)).toBe("var(--data-series-2)");
  });

  it("never produces a negative or zero slot number", () => {
    for (let i = -5; i < 20; i++) {
      expect(seriesColorVar(i)).toMatch(/^var\(--data-series-[1-8]\)$/);
    }
  });
});
