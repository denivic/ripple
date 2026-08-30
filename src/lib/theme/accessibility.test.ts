import { describe, expect, it } from "vitest";
import { applyAccessibilityOverrides, NO_ACCESSIBILITY_OVERRIDES } from "./accessibility";
import { deepCurrent } from "./presets";

describe("applyAccessibilityOverrides", () => {
  it("returns the theme unchanged with no preferences active", () => {
    expect(applyAccessibilityOverrides(deepCurrent, NO_ACCESSIBILITY_OVERRIDES)).toEqual(deepCurrent);
  });

  it("forces critical damping and a near-instant response under reduced motion", () => {
    const result = applyAccessibilityOverrides(deepCurrent, { ...NO_ACCESSIBILITY_OVERRIDES, reducedMotion: true });
    expect(result.motion.damping).toBe(1);
    expect(result.motion.response).toBeLessThanOrEqual(0.01);
  });

  it("goes fully opaque with no blur under reduced transparency", () => {
    const result = applyAccessibilityOverrides(deepCurrent, {
      ...NO_ACCESSIBILITY_OVERRIDES,
      reducedTransparency: true,
    });
    expect(result.material.alpha).toBe(1);
    expect(result.material.blur).toBe("0px");
  });

  it("strengthens the hairline under increased contrast", () => {
    const result = applyAccessibilityOverrides(deepCurrent, {
      ...NO_ACCESSIBILITY_OVERRIDES,
      increasedContrast: true,
    });
    expect(result.line.hairline).toBe(deepCurrent.text.secondary);
  });

  it("composes independently when multiple preferences are active", () => {
    const result = applyAccessibilityOverrides(deepCurrent, {
      reducedMotion: true,
      reducedTransparency: true,
      increasedContrast: true,
    });
    expect(result.motion.damping).toBe(1);
    expect(result.material.alpha).toBe(1);
    expect(result.line.hairline).toBe(deepCurrent.text.secondary);
  });

  it("never mutates the input theme", () => {
    const before = JSON.parse(JSON.stringify(deepCurrent));
    applyAccessibilityOverrides(deepCurrent, { reducedMotion: true, reducedTransparency: true, increasedContrast: true });
    expect(deepCurrent).toEqual(before);
  });
});
