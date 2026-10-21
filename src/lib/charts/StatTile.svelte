<script lang="ts">
  import type { Snippet } from "svelte";

  interface Delta {
    text: string;
    direction: "up" | "down" | "flat";
    goodDirection: "up" | "down";
  }

  interface Props {
    label: string;
    value: string;
    /** Exact figure for the tooltip when `value` is rounded or compacted. */
    title?: string;
    /** A currency or unit shown smaller beside `value`; it wraps onto its own
        line when the tile is too narrow, so the figure keeps the full width. */
    unit?: string;
    unitPosition?: "before" | "after";
    delta?: Delta;
    /** A short caveat under the value, for figures that need their assumptions spelled out. */
    hint?: string;
    trend?: Snippet;
  }

  let { label, value, title, unit, unitPosition = "before", delta, hint, trend }: Props = $props();

  // Figures ("DKK 15,302", "4h 20m", "12 clean days") stay on one line and
  // shrink to fit the tile — never clipped; only a sentence-length value wraps.
  const singleToken = $derived(value.length <= 24 || !/ /.test(value.trim()));

  const deltaTone = $derived(
    !delta || delta.direction === "flat" ? "neutral" : delta.direction === delta.goodDirection ? "good" : "bad",
  );
</script>

<div class="stat-tile">
  <span class="label">{label}</span>
  <span class="value" class:single={singleToken} class:unit-after={unitPosition === "after"} {title}>
    {#if unit}<span class="unit">{unit}</span>{/if}
    <span class="figure" style:--chars={value.length}>{value}</span>
  </span>
  {#if delta}
    <span class="delta {deltaTone}">{delta.text}</span>
  {/if}
  {#if hint}
    <span class="hint">{hint}</span>
  {/if}
  {#if trend}
    <div class="trend">{@render trend()}</div>
  {/if}
</div>

<style>
  .stat-tile {
    min-width: 0;
    /* Lets .value size itself against the tile's width (cqi units). */
    container-type: inline-size;
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
    padding: 1rem 1.125rem;
    background: var(--surface-raised);
    border: 1px solid var(--line-hairline);
    border-radius: 0.875rem;
  }

  .label {
    font-size: 0.8125rem;
    color: var(--text-secondary);
  }

  .value {
    /* Proportional figures at display size — tabular-nums is for aligned
       columns, not a standalone hero-ish number. */
    font-size: 1.75rem;
    font-weight: 600;
    letter-spacing: -0.02em;
    line-height: 1.2;
    color: var(--text-primary);
    min-width: 0;
    overflow-wrap: break-word;
  }

  .value.single {
    display: flex;
    flex-wrap: wrap;
    align-items: baseline;
    column-gap: 0.3em;
  }

  .value.single .figure {
    white-space: nowrap;
    /* Pure CSS, so it holds on first paint and on every resize: cap at the
       display size, otherwise give each character an equal share of the
       tile's width (~0.7em covers a bold digit or capital, with headroom). */
    font-size: min(1.75rem, calc(100cqi / (var(--chars) * 0.7)));
  }

  .unit {
    font-size: 1rem;
    font-weight: 500;
    color: var(--text-secondary);
    white-space: nowrap;
  }

  .unit-after .unit {
    order: 1;
  }

  /* Sentences read better a step down from the hero size. */
  .value:not(.single) {
    font-size: 1.125rem;
  }

  .delta {
    font-size: 0.8125rem;
    font-weight: 500;
  }
  /* Fixed status hues (dataviz skill palette.md) — never themed. */
  .delta.good {
    color: #0ca30c;
  }
  .delta.bad {
    color: #d03b3b;
  }
  .delta.neutral {
    color: var(--text-tertiary);
  }

  .hint {
    font-size: 0.75rem;
    line-height: 1.4;
    color: var(--text-tertiary);
  }

  .trend {
    margin-top: 0.25rem;
  }
</style>
