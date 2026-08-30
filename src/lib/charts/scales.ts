const CATEGORICAL_SLOT_COUNT = 8;

/** Assigns a stable habit index to one of the theme's 8 validated
 * categorical slots. The palette itself is never regenerated per-habit —
 * only which fixed slot a habit lands on cycles, which is what the dataviz
 * skill's "assign in fixed order, never cycled" rule actually protects. */
export function seriesColorVar(index: number): string {
  return `var(--data-series-${(((index % CATEGORICAL_SLOT_COUNT) + CATEGORICAL_SLOT_COUNT) % CATEGORICAL_SLOT_COUNT) + 1})`;
}
