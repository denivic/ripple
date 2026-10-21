import { describe, expect, it } from "vitest";
import type { HabitPreset } from "$lib/ipc";
import { filterPresets, formatPresetSource, groupPresets } from "./habit-presets";

function preset(overrides: Partial<HabitPreset>): HabitPreset {
  return {
    id: "x",
    name: "X",
    unitLabel: "unit",
    lifeMinutesPerUnit: 0,
    minutesPerUnit: 0,
    category: "food",
    sourceTitle: "A study",
    sourceUrl: "https://doi.org/10/x",
    year: 2021,
    confidence: "medium",
    derived: false,
    derivation: null,
    note: "",
    ...overrides,
  };
}

const presets = [
  preset({ id: "cigarette", name: "Cigarettes", unitLabel: "cigarette", category: "tobacco" }),
  preset({ id: "red-meat", name: "Red meat", unitLabel: "portion (85 g)", category: "food" }),
  preset({ id: "nuts", name: "Nuts", unitLabel: "handful", category: "gain", lifeMinutesPerUnit: -25.5 }),
  preset({ id: "hot-dog", name: "Hot dogs", unitLabel: "hot dog in bun", category: "food" }),
];

describe("filterPresets", () => {
  it("returns everything for a blank query", () => {
    expect(filterPresets(presets, "  ")).toEqual(presets);
  });

  it("matches case-insensitively on name", () => {
    expect(filterPresets(presets, "CIGAR").map((p) => p.id)).toEqual(["cigarette"]);
  });

  it("requires every term, in any order", () => {
    expect(filterPresets(presets, "meat red").map((p) => p.id)).toEqual(["red-meat"]);
  });

  it("matches the unit and the category label", () => {
    expect(filterPresets(presets, "handful").map((p) => p.id)).toEqual(["nuts"]);
    expect(filterPresets(presets, "life-gaining").map((p) => p.id)).toEqual(["nuts"]);
  });
});

describe("groupPresets", () => {
  it("orders groups by category, keeps table order, and drops empty groups", () => {
    const groups = groupPresets(presets);
    expect(groups.map((g) => g.category)).toEqual(["tobacco", "food", "gain"]);
    expect(groups[1].presets.map((p) => p.id)).toEqual(["red-meat", "hot-dog"]);
  });
});

describe("formatPresetSource", () => {
  it("formats title, year and confidence", () => {
    expect(formatPresetSource(preset({ sourceTitle: "BMJ", year: 2012, confidence: "low" }))).toBe(
      "Source: BMJ (2012) · confidence: low",
    );
  });

  it("omits a missing year and returns null without a source", () => {
    expect(formatPresetSource(preset({ sourceTitle: "BMJ", year: null }))).toBe("Source: BMJ · confidence: medium");
    expect(formatPresetSource(preset({ sourceTitle: null }))).toBeNull();
  });
});
