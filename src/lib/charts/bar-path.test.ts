import { describe, expect, it } from "vitest";
import { horizontalBarPath, verticalBarPath } from "./bar-path";

describe("verticalBarPath", () => {
  it("returns empty for zero or negative height", () => {
    expect(verticalBarPath(0, 0, 10, 0)).toBe("");
    expect(verticalBarPath(0, 0, 10, -5)).toBe("");
  });

  it("starts at the square baseline corner", () => {
    const path = verticalBarPath(10, 20, 24, 50, 4);
    expect(path.startsWith("M10,70")).toBe(true); // top(20) + height(50) = 70
  });

  it("clamps radius to half the width when the bar is thinner than 2*radius", () => {
    const path = verticalBarPath(0, 0, 4, 100, 4);
    // width 4 -> max radius 2, so the horizontal arc-to point is at x=2, not x=4.
    expect(path).toContain("A2,2");
  });
});

describe("horizontalBarPath", () => {
  it("returns empty for zero or negative width", () => {
    expect(horizontalBarPath(0, 0, 0, 10)).toBe("");
  });

  it("starts at the square baseline corner", () => {
    const path = horizontalBarPath(5, 15, 80, 20, 4);
    expect(path.startsWith("M5,15")).toBe(true);
  });
});
