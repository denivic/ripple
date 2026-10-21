import type { HabitPreset, PresetCategory, SettingsDto } from "$lib/ipc";

/** Display order and labels for the add-habit preset picker's groups. */
export const PRESET_CATEGORIES: { category: PresetCategory; label: string }[] = [
  { category: "tobacco", label: "Tobacco" },
  { category: "alcohol", label: "Alcohol" },
  { category: "cannabis", label: "Cannabis" },
  { category: "food", label: "Food" },
  { category: "drink", label: "Drinks" },
  { category: "sedentary", label: "Sitting & screens" },
  { category: "gain", label: "Life-gaining habits" },
];

export interface PresetGroup {
  category: PresetCategory;
  label: string;
  presets: HabitPreset[];
}

function categoryLabel(category: PresetCategory): string {
  return PRESET_CATEGORIES.find((c) => c.category === category)?.label ?? category;
}

/** Every whitespace-separated term must appear (case-insensitively) in the
 * preset's name, unit or category label, so "red meat" and "meat red" agree. */
export function filterPresets(presets: HabitPreset[], query: string): HabitPreset[] {
  const terms = query.toLowerCase().split(/\s+/).filter(Boolean);
  if (terms.length === 0) return presets;
  return presets.filter((p) => {
    const haystack = `${p.name} ${p.unitLabel} ${categoryLabel(p.category)}`.toLowerCase();
    return terms.every((t) => haystack.includes(t));
  });
}

/** Groups in `PRESET_CATEGORIES` order, keeping table order within a group
 * and dropping empty groups. */
export function groupPresets(presets: HabitPreset[]): PresetGroup[] {
  return PRESET_CATEGORIES.map(({ category, label }) => ({
    category,
    label,
    presets: presets.filter((p) => p.category === category),
  })).filter((g) => g.presets.length > 0);
}

/** "Source: {title} ({year}) · confidence: {confidence}", or null when the
 * preset has no source at all. */
export function formatPresetSource(preset: HabitPreset): string | null {
  if (!preset.sourceTitle) return null;
  const year = preset.year !== null ? ` (${preset.year})` : "";
  return `Source: ${preset.sourceTitle}${year} · confidence: ${preset.confidence}`;
}

/** The unit label matching the Units setting: the imperial variant when the
 * user chose imperial and the preset has one, otherwise the metric label. */
export function presetUnitLabel(preset: HabitPreset, units: SettingsDto["weightUnit"]): string {
  return units === "lb" && preset.unitLabelImperial ? preset.unitLabelImperial : preset.unitLabel;
}
