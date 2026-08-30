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

  // Validated with the dataviz skill's validate_palette.js against each
  // theme's own surface color (lightness band, chroma floor, CVD separation,
  // normal-vision floor, contrast — all PASS). Re-run that validator by hand
  // if this array ever changes; this test only guards against silent drift.
  const VALIDATED_CATEGORICAL_ORDER = [
    "#3987E5",
    "#D95926",
    "#199E70",
    "#C98500",
    "#D55181",
    "#008300",
    "#9085E9",
    "#E66767",
  ];

  it.each(builtInThemes)("$name's data series is the validated categorical order", (theme) => {
    expect(theme.data.series).toEqual(VALIDATED_CATEGORICAL_ORDER);
  });
});
