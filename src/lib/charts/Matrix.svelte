<script lang="ts">
  import Tooltip from "./Tooltip.svelte";
  import { formatDuration } from "./format";

  interface Props {
    /** [weekday 0=Mon..6=Sun][hour 0..23], matching the Rust domain's
     * hour_weekday_matrix output shape. */
    matrix: number[][];
  }

  let { matrix }: Props = $props();

  const WEEKDAY_LABELS = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
  const CELL = 16;
  const GAP = 2;
  const LABEL_WIDTH = 30;
  const TOP = 16;
  const HOUR_TICKS = [0, 6, 12, 18, 23];

  const maxValue = $derived(matrix.reduce((m, row) => row.reduce((rm, v) => Math.max(rm, v), m), 0) || 1);

  function intensity(value: number): string {
    // Faint but visible: --surface-raised is the card colour, so empty
    // slots drawn with it vanished and the grid lost its shape.
    if (value <= 0) return "var(--chart-empty)";
    const pct = 12 + (value / maxValue) * 78;
    return `color-mix(in oklch, var(--accent-base) ${pct.toFixed(1)}%, var(--surface-raised))`;
  }

  let hovered = $state<{ weekday: number; hour: number; value: number } | null>(null);
</script>

<div class="matrix-wrap">
  <svg width={LABEL_WIDTH + 24 * (CELL + GAP)} height={TOP + 7 * (CELL + GAP)}>
    {#each HOUR_TICKS as hour (hour)}
      <text x={LABEL_WIDTH + hour * (CELL + GAP) + CELL / 2} y={TOP - 5} class="col-label">
        {String(hour).padStart(2, "0")}
      </text>
    {/each}
    {#each WEEKDAY_LABELS as label, row (label)}
      <text x="0" y={TOP + row * (CELL + GAP) + CELL / 2 + 4} class="row-label">{label}</text>
    {/each}
    {#each matrix as row, weekday (weekday)}
      {#each row as value, hour (hour)}
        <rect
          x={LABEL_WIDTH + hour * (CELL + GAP)}
          y={TOP + weekday * (CELL + GAP)}
          width={CELL}
          height={CELL}
          rx="3"
          fill={intensity(value)}
          role="img"
          aria-label="{WEEKDAY_LABELS[weekday]} {String(hour).padStart(2, '0')}:00: {formatDuration(value)}"
          onpointerenter={() => (hovered = { weekday, hour, value })}
          onpointerleave={() => (hovered = null)}
        />
      {/each}
    {/each}
  </svg>
  <Tooltip
    x={LABEL_WIDTH + (hovered?.hour ?? 0) * (CELL + GAP) + CELL / 2}
    y={TOP + (hovered?.weekday ?? 0) * (CELL + GAP)}
    visible={!!hovered}
  >
    {#if hovered}
      <strong>{formatDuration(hovered.value)}</strong>
      <span class="muted">{WEEKDAY_LABELS[hovered.weekday]} {String(hovered.hour).padStart(2, "0")}:00</span>
    {/if}
  </Tooltip>
</div>

<style>
  .matrix-wrap {
    position: relative;
  }
  rect {
    transition: opacity 100ms ease-out;
    cursor: default;
  }
  rect:hover {
    opacity: 0.8;
  }
  .row-label {
    font-size: 0.6875rem;
    fill: var(--text-tertiary);
  }
  .col-label {
    font-size: 0.625rem;
    fill: var(--text-tertiary);
    text-anchor: middle;
  }
  .muted {
    color: var(--text-tertiary);
    margin-left: 0.375rem;
  }
</style>
