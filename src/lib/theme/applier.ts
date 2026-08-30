import { applyAccessibilityOverrides, type AccessibilityPreferences } from "./accessibility";
import type { Theme } from "./schema";

export function themeToCssVars(theme: Theme): Record<string, string> {
  const vars: Record<string, string> = {
    "--surface-base": theme.surface.base,
    "--surface-sidebar-from": theme.surface.sidebarFrom,
    "--surface-sidebar-to": theme.surface.sidebarTo,
    "--surface-raised": theme.surface.raised,
    "--text-primary": theme.text.primary,
    "--text-secondary": theme.text.secondary,
    "--text-tertiary": theme.text.tertiary,
    "--accent-base": theme.accent.base,
    "--accent-muted": theme.accent.muted,
    "--line-hairline": theme.line.hairline,
    "--material-blur": theme.material.blur,
    "--material-saturate": theme.material.saturate,
    "--material-alpha": String(theme.material.alpha),
    "--motion-damping": String(theme.motion.damping),
    "--motion-response": String(theme.motion.response),
  };
  theme.data.series.forEach((color, i) => {
    vars[`--data-series-${i + 1}`] = color;
  });
  return vars;
}

const ACCESSIBILITY_QUERIES = [
  "(prefers-reduced-motion: reduce)",
  "(prefers-reduced-transparency: reduce)",
  "(prefers-contrast: more)",
] as const;

export function readAccessibilityPreferences(): AccessibilityPreferences {
  if (typeof window === "undefined") {
    return { reducedMotion: false, reducedTransparency: false, increasedContrast: false };
  }
  const [motion, transparency, contrast] = ACCESSIBILITY_QUERIES.map((q) => window.matchMedia(q).matches);
  return { reducedMotion: motion, reducedTransparency: transparency, increasedContrast: contrast };
}

/** Applies a theme (plus live accessibility overrides) as CSS custom
 * properties on `root`. Charts and the grid read these back via
 * `getComputedStyle` rather than duplicating the palette in JS — see
 * `tokens.svelte.ts`. */
export function applyTheme(theme: Theme, root: HTMLElement = document.documentElement): void {
  const resolved = applyAccessibilityOverrides(theme, readAccessibilityPreferences());
  for (const [name, value] of Object.entries(themeToCssVars(resolved))) {
    root.style.setProperty(name, value);
  }
}

export function watchAccessibilityPreferences(onChange: () => void): () => void {
  if (typeof window === "undefined") return () => {};
  const queries = ACCESSIBILITY_QUERIES.map((q) => window.matchMedia(q));
  queries.forEach((mq) => mq.addEventListener("change", onChange));
  return () => queries.forEach((mq) => mq.removeEventListener("change", onChange));
}
