import { describe, expect, it } from "vitest";
import { contrastRatio, meetsAA, relativeLuminance } from "./contrast";

describe("relativeLuminance", () => {
  it("is 0 for black and 1 for white", () => {
    expect(relativeLuminance("#000000")).toBeCloseTo(0, 5);
    expect(relativeLuminance("#FFFFFF")).toBeCloseTo(1, 5);
  });

  it("expands 3-digit hex the same as its 6-digit equivalent", () => {
    expect(relativeLuminance("#fff")).toBeCloseTo(relativeLuminance("#ffffff"), 10);
  });
});

describe("contrastRatio", () => {
  it("is 21:1 for black on white, the WCAG maximum", () => {
    expect(contrastRatio("#000000", "#FFFFFF")).toBeCloseTo(21, 1);
  });

  it("is 1:1 for identical colors", () => {
    expect(contrastRatio("#2DD4BF", "#2DD4BF")).toBeCloseTo(1, 5);
  });

  it("is symmetric", () => {
    expect(contrastRatio("#0B0D0C", "#FFFFFF")).toBeCloseTo(contrastRatio("#FFFFFF", "#0B0D0C"), 10);
  });
});

describe("meetsAA", () => {
  it("passes normal text at the 4.5:1 threshold", () => {
    expect(meetsAA("#FFFFFF", "#0B0D0C")).toBe(true);
    expect(meetsAA("#5E6664", "#0B0D0C")).toBe(false);
  });

  it("uses the lower 3:1 threshold for large text", () => {
    // #858585 on white sits at ~3.7:1 — comfortably between the two thresholds.
    const midRange = "#858585";
    expect(meetsAA(midRange, "#FFFFFF", false)).toBe(false);
    expect(meetsAA(midRange, "#FFFFFF", true)).toBe(true);
  });
});
