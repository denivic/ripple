import type { Theme } from "./schema";

export interface AccessibilityPreferences {
  reducedMotion: boolean;
  reducedTransparency: boolean;
  increasedContrast: boolean;
}

export const NO_ACCESSIBILITY_OVERRIDES: AccessibilityPreferences = {
  reducedMotion: false,
  reducedTransparency: false,
  increasedContrast: false,
};

/**
 * A pure token transform, kept separate from DOM/matchMedia wiring so it's
 * directly unit-testable. Mirrors the apple-design fallbacks: reduced motion
 * drops to a near-instant, non-bouncing response; reduced transparency goes
 * fully opaque with no blur; increased contrast strengthens hairlines.
 */
export function applyAccessibilityOverrides(theme: Theme, prefs: AccessibilityPreferences): Theme {
  let result = theme;

  if (prefs.reducedMotion) {
    result = { ...result, motion: { damping: 1, response: Math.min(result.motion.response, 0.01) } };
  }

  if (prefs.reducedTransparency) {
    result = { ...result, material: { ...result.material, blur: "0px", alpha: 1 } };
  }

  if (prefs.increasedContrast) {
    result = { ...result, line: { hairline: result.text.secondary } };
  }

  return result;
}
