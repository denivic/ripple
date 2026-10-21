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
    delta?: Delta;
    /** A short caveat under the value, for figures that need their assumptions spelled out. */
    hint?: string;
    trend?: Snippet;
  }

  let { label, value, delta, hint, trend }: Props = $props();

  const deltaTone = $derived(
    !delta || delta.direction === "flat" ? "neutral" : delta.direction === delta.goodDirection ? "good" : "bad",
  );
</script>

<div class="stat-tile">
  <span class="label">{label}</span>
  <span class="value">{value}</span>
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
    color: var(--text-primary);
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
