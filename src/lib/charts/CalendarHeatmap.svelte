<script lang="ts">
  import { timeDay, timeWeek } from "d3-time";
  import Tooltip from "./Tooltip.svelte";
  import { formatCalendarDate, formatDuration } from "./format";

  interface DayValue {
    date: Date;
    value: number;
  }

  interface Props {
    data: DayValue[];
    start: Date;
    end: Date;
  }

  let { data, start, end }: Props = $props();

  const CELL = 12;
  const GAP = 3;

  const byDate = $derived(new Map(data.map((d) => [timeDay(d.date).getTime(), d.value])));
  const weekStart = $derived(timeWeek(start));

  const cells = $derived.by(() => {
    const out: { date: Date; value: number; column: number; row: number }[] = [];
    let d = timeDay(start);
    const last = timeDay(end);
    while (d.getTime() <= last.getTime()) {
      const column = Math.round((timeWeek(d).getTime() - weekStart.getTime()) / (7 * 86400000));
      out.push({ date: d, value: byDate.get(d.getTime()) ?? 0, column, row: d.getDay() });
      d = timeDay.offset(d, 1);
    }
    return out;
  });

  const maxValue = $derived(cells.reduce((m, c) => Math.max(m, c.value), 0) || 1);
  const columnCount = $derived(cells.reduce((m, c) => Math.max(m, c.column), 0) + 1);

  function intensity(value: number): string {
    if (value <= 0) return "var(--surface-raised)";
    const pct = 12 + (value / maxValue) * 78;
    return `color-mix(in oklch, var(--accent-base) ${pct.toFixed(1)}%, var(--surface-raised))`;
  }

  let hovered = $state<{ date: Date; value: number; column: number; row: number } | null>(null);
</script>

<div class="heatmap-wrap" style:width="{columnCount * (CELL + GAP)}px" style:height="{7 * (CELL + GAP)}px">
  <svg width={columnCount * (CELL + GAP)} height={7 * (CELL + GAP)}>
    {#each cells as cell (cell.date.getTime())}
      <rect
        x={cell.column * (CELL + GAP)}
        y={cell.row * (CELL + GAP)}
        width={CELL}
        height={CELL}
        rx="3"
        fill={intensity(cell.value)}
        role="img"
        aria-label="{formatCalendarDate(cell.date)}: {formatDuration(cell.value)}"
        onpointerenter={() => (hovered = cell)}
        onpointerleave={() => (hovered = null)}
      />
    {/each}
  </svg>
  <Tooltip
    x={(hovered?.column ?? 0) * (CELL + GAP) + CELL / 2}
    y={(hovered?.row ?? 0) * (CELL + GAP)}
    visible={!!hovered}
  >
    {#if hovered}
      <strong>{formatDuration(hovered.value)}</strong>
      <span class="muted">{formatCalendarDate(hovered.date)}</span>
    {/if}
  </Tooltip>
</div>

<style>
  .heatmap-wrap {
    position: relative;
  }
  rect {
    transition: opacity 100ms ease-out;
    cursor: default;
  }
  rect:hover {
    opacity: 0.8;
  }
  .muted {
    color: var(--text-tertiary);
    margin-left: 0.375rem;
  }
</style>
