import { describe, expect, it } from "vitest";
import { deepCurrent } from "./presets";
import { validateTheme } from "./schema";

describe("validateTheme", () => {
  it("accepts a well-formed theme", () => {
    const result = validateTheme(deepCurrent);
    expect(result.valid).toBe(true);
  });

  it("rejects a non-object", () => {
    const result = validateTheme("not a theme");
    expect(result.valid).toBe(false);
  });

  it("rejects the wrong schema version", () => {
    const result = validateTheme({ ...deepCurrent, schemaVersion: 2 });
    expect(result.valid).toBe(false);
    if (!result.valid) {
      expect(result.errors.some((e) => e.path === "schemaVersion")).toBe(true);
    }
  });

  it("rejects a non-hex color", () => {
    const result = validateTheme({ ...deepCurrent, accent: { base: "teal", muted: "#134E4A" } });
    expect(result.valid).toBe(false);
    if (!result.valid) {
      expect(result.errors.some((e) => e.path === "accent.base")).toBe(true);
    }
  });

  it("rejects an empty data series", () => {
    const result = validateTheme({ ...deepCurrent, data: { series: [] } });
    expect(result.valid).toBe(false);
  });

  it("rejects material.alpha out of range", () => {
    const result = validateTheme({ ...deepCurrent, material: { ...deepCurrent.material, alpha: 1.5 } });
    expect(result.valid).toBe(false);
  });

  it("collects every error, not just the first", () => {
    const result = validateTheme({ schemaVersion: 1, name: "" });
    expect(result.valid).toBe(false);
    if (!result.valid) {
      expect(result.errors.length).toBeGreaterThan(1);
    }
  });
});
