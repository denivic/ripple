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
    delta?: Delta;
    /** A short caveat under the value, for figures that need their assumptions spelled out. */
    hint?: string;
    trend?: Snippet;
  }

  let { label, value, title, delta, hint, trend }: Props = $props();

  // Figures ("DKK 15,302", "4h 20m", "12 clean days") stay on one line and
  // shrink to fit; only a sentence-length value wraps.
  const singleToken = $derived(value.length <= 24 || !/ /.test(value.trim()));

  const MAX_PX = 28;
  const MIN_PX = 16;

  /** Steps the font size down until the value fits the tile's width. */
  function fit(node: HTMLElement, _value: string) {
    const measure = () => {
      if (!singleToken) {
        node.style.fontSize = "";
        return;
      }
      let size = MAX_PX;
      node.style.fontSize = `${size}px`;
      while (node.scrollWidth > node.clientWidth && size > MIN_PX) {
        size -= 1;
        node.style.fontSize = `${size}px`;
      }
    };
    const observer = new ResizeObserver(measure);
    observer.observe(node);
    measure();
    return {
      update: measure,
      destroy: () => observer.disconnect(),
    };
  }

  const deltaTone = $derived(
    !delta || delta.direction === "flat" ? "neutral" : delta.direction === delta.goodDirection ? "good" : "bad",
  );
</script>

<div class="stat-tile">
  <span class="label">{label}</span>
  <span class="value" class:single={singleToken} {title} use:fit={value}>{value}</span>
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
    white-space: nowrap;
    /* Last resort below MIN_PX: clip with an ellipsis; the title has it all. */
    overflow: hidden;
    text-overflow: ellipsis;
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
