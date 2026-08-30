import { applyTheme, watchAccessibilityPreferences } from "./applier";
import { deepCurrent } from "./presets";
import type { Theme } from "./schema";

/**
 * Single reactive source of truth for the active theme. CSS reads it via the
 * custom properties `applyTheme` sets on `:root`; chart/d3 code that needs a
 * real color string (not a `var()` reference) calls `resolve()`, which is
 * the one `getComputedStyle` call per theme change the plan calls for.
 */
class ThemeStore {
  current = $state<Theme>(deepCurrent);
  #stopWatchingAccessibility: (() => void) | null = null;

  set(theme: Theme) {
    this.current = theme;
    if (typeof document !== "undefined") {
      applyTheme(theme);
    }
  }

  mount() {
    if (typeof document === "undefined" || this.#stopWatchingAccessibility) return;
    applyTheme(this.current);
    this.#stopWatchingAccessibility = watchAccessibilityPreferences(() => applyTheme(this.current));
  }

  unmount() {
    this.#stopWatchingAccessibility?.();
    this.#stopWatchingAccessibility = null;
  }

  resolve(cssVarName: string): string {
    if (typeof document === "undefined") return "";
    return getComputedStyle(document.documentElement).getPropertyValue(cssVarName).trim();
  }
}

export const themeStore = new ThemeStore();
