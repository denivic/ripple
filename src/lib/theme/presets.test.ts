import { describe, expect, it } from "vitest";
import { meetsAA } from "./contrast";
import { builtInThemes } from "./presets";
import { validateTheme } from "./schema";

describe("built-in themes", () => {
  it.each(builtInThemes)("$name validates against the theme schema", (theme) => {
    expect(validateTheme(theme).valid).toBe(true);
  });

  it.each(builtInThemes)("$name's primary text passes AA against the base surface", (theme) => {
    expect(meetsAA(theme.text.primary, theme.surface.base)).toBe(true);
  });

  it.each(builtInThemes)("$name's secondary text passes AA for large text against the base surface", (theme) => {
    expect(meetsAA(theme.text.secondary, theme.surface.base, true)).toBe(true);
  });

  it("ships more than one theme", () => {
    expect(builtInThemes.length).toBeGreaterThan(1);
  });

  it("has unique names", () => {
    const names = builtInThemes.map((t) => t.name);
    expect(new Set(names).size).toBe(names.length);
  });
});
