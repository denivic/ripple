export interface Theme {
  schemaVersion: 1;
  name: string;
  surface: { base: string; sidebarFrom: string; sidebarTo: string; raised: string };
  text: { primary: string; secondary: string; tertiary: string };
  accent: { base: string; muted: string };
  line: { hairline: string };
  data: { series: string[] };
  material: { blur: string; saturate: string; alpha: number };
  motion: { damping: number; response: number };
}

export interface ThemeValidationError {
  path: string;
  message: string;
}

export type ThemeValidationResult =
  | { valid: true; theme: Theme }
  | { valid: false; errors: ThemeValidationError[] };

const HEX_COLOR_RE = /^#([0-9a-fA-F]{3}|[0-9a-fA-F]{6})$/;

function isHexColor(v: unknown): v is string {
  return typeof v === "string" && HEX_COLOR_RE.test(v);
}

function check(errors: ThemeValidationError[], path: string, ok: boolean, message: string) {
  if (!ok) errors.push({ path, message });
}

/** Hand-rolled rather than a schema library: the shape is small and fixed
 * (see plan-v1.md's theme format), so a dependency buys nothing here. */
export function validateTheme(input: unknown): ThemeValidationResult {
  const errors: ThemeValidationError[] = [];
  if (typeof input !== "object" || input === null) {
    return { valid: false, errors: [{ path: "$", message: "theme must be an object" }] };
  }
  const t = input as Record<string, unknown>;

  check(errors, "schemaVersion", t.schemaVersion === 1, "schemaVersion must be 1");
  check(errors, "name", typeof t.name === "string" && t.name.length > 0, "name must be a non-empty string");

  const surface = t.surface as Record<string, unknown> | undefined;
  for (const key of ["base", "sidebarFrom", "sidebarTo", "raised"] as const) {
    check(errors, `surface.${key}`, isHexColor(surface?.[key]), `surface.${key} must be a hex color`);
  }

  const text = t.text as Record<string, unknown> | undefined;
  for (const key of ["primary", "secondary", "tertiary"] as const) {
    check(errors, `text.${key}`, isHexColor(text?.[key]), `text.${key} must be a hex color`);
  }

  const accent = t.accent as Record<string, unknown> | undefined;
  for (const key of ["base", "muted"] as const) {
    check(errors, `accent.${key}`, isHexColor(accent?.[key]), `accent.${key} must be a hex color`);
  }

  const line = t.line as Record<string, unknown> | undefined;
  check(errors, "line.hairline", isHexColor(line?.hairline), "line.hairline must be a hex color");

  const data = t.data as Record<string, unknown> | undefined;
  const series = data?.series;
  check(
    errors,
    "data.series",
    Array.isArray(series) && series.length > 0 && series.every(isHexColor),
    "data.series must be a non-empty array of hex colors",
  );

  const material = t.material as Record<string, unknown> | undefined;
  check(errors, "material.blur", typeof material?.blur === "string", "material.blur must be a CSS length string");
  check(
    errors,
    "material.saturate",
    typeof material?.saturate === "string",
    "material.saturate must be a CSS percentage string",
  );
  check(
    errors,
    "material.alpha",
    typeof material?.alpha === "number" && material.alpha >= 0 && material.alpha <= 1,
    "material.alpha must be a number between 0 and 1",
  );

  const motion = t.motion as Record<string, unknown> | undefined;
  check(
    errors,
    "motion.damping",
    typeof motion?.damping === "number" && motion.damping > 0,
    "motion.damping must be a positive number",
  );
  check(
    errors,
    "motion.response",
    typeof motion?.response === "number" && motion.response > 0,
    "motion.response must be a positive number",
  );

  if (errors.length > 0) return { valid: false, errors };
  return { valid: true, theme: input as Theme };
}
