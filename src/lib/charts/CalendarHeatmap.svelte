<script lang="ts">
  import { timeDay, timeMonday, timeSunday } from "d3-time";
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
    weekStart?: "monday" | "sunday";
  }

  let { data, start, end, weekStart = "monday" }: Props = $props();

  const CELL = 12;
  const GAP = 3;
  // Room for the month labels above and the weekday labels to the left.
  const TOP = 16;
  const LEFT = 28;

  const byDate = $derived(new Map(data.map((d) => [timeDay(d.date).getTime(), d.value])));
  const weekInterval = $derived(weekStart === "sunday" ? timeSunday : timeMonday);
  const firstWeekStart = $derived(weekInterval(start));

  const cells = $derived.by(() => {
    const out: { date: Date; value: number; column: number; row: number }[] = [];
    let d = timeDay(start);
    const last = timeDay(end);
    while (d.getTime() <= last.getTime()) {
      const column = Math.round((weekInterval(d).getTime() - firstWeekStart.getTime()) / (7 * 86400000));
      const row = weekStart === "sunday" ? d.getDay() : (d.getDay() + 6) % 7;
      out.push({ date: d, value: byDate.get(d.getTime()) ?? 0, column, row });
      d = timeDay.offset(d, 1);
    }
    return out;
  });

  const maxValue = $derived(cells.reduce((m, c) => Math.max(m, c.value), 0) || 1);
  const columnCount = $derived(cells.reduce((m, c) => Math.max(m, c.column), 0) + 1);

  // Month label over the first column that contains the 1st of that month
  // (or the range start), so labels line up with the weeks they name.
  const monthLabels = $derived.by(() => {
    const out: { column: number; text: string }[] = [];
    let lastMonth = -1;
    for (const c of cells) {
      const month = c.date.getMonth();
      if (month !== lastMonth && (c.date.getDate() <= 7 || out.length === 0)) {
        // Skip a label that would collide with the previous one.
        if (out.length === 0 || c.column - out[out.length - 1].column >= 3) {
          out.push({ column: c.column, text: MONTH_FORMAT.format(c.date) });
        }
        lastMonth = month;
      }
    }
    return out;
  });

  const MONTH_FORMAT = new Intl.DateTimeFormat(undefined, { month: "short" });
  const WEEKDAY_FORMAT = new Intl.DateTimeFormat(undefined, { weekday: "short" });
  // Label every other row (Mon/Wed/Fri/Sun, or Sun/Tue/Thu/Sat when weeks
  // start on Sunday) — all seven would crowd a 12px grid.
  const weekdayLabels = $derived(
    [0, 2, 4, 6].map((row) => {
      // 2024-01-01 was a Monday; offset to the row's weekday.
      const date = new Date(2024, 0, 1 + ((weekStart === "sunday" ? row - 1 : row) + 7) % 7);
      return { row, text: WEEKDAY_FORMAT.format(date) };
    }),
  );

  function intensity(value: number): string {
    // Empty days get a faint but visible swatch: the card is --surface-raised
    // too, so using it here made every empty day disappear.
    if (value <= 0) return "var(--chart-empty)";
    const pct = 12 + (value / maxValue) * 78;
    return `color-mix(in oklch, var(--accent-base) ${pct.toFixed(1)}%, var(--surface-raised))`;
  }

  let hovered = $state<{ date: Date; value: number; column: number; row: number } | null>(null);
</script>

<div
  class="heatmap-wrap"
  style:width="{LEFT + columnCount * (CELL + GAP)}px"
  style:height="{TOP + 7 * (CELL + GAP)}px"
>
  <svg width={LEFT + columnCount * (CELL + GAP)} height={TOP + 7 * (CELL + GAP)}>
    {#each monthLabels as label (label.column)}
      <text x={LEFT + label.column * (CELL + GAP)} y={TOP - 5} class="axis-label">{label.text}</text>
    {/each}
    {#each weekdayLabels as label (label.row)}
      <text x="0" y={TOP + label.row * (CELL + GAP) + CELL - 2} class="axis-label">{label.text}</text>
    {/each}
    {#each cells as cell (cell.date.getTime())}
      <rect
        x={LEFT + cell.column * (CELL + GAP)}
        y={TOP + cell.row * (CELL + GAP)}
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
    x={LEFT + (hovered?.column ?? 0) * (CELL + GAP) + CELL / 2}
    y={TOP + (hovered?.row ?? 0) * (CELL + GAP)}
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
  .axis-label {
    font-size: 0.625rem;
    fill: var(--text-tertiary);
  }
  .muted {
    color: var(--text-tertiary);
    margin-left: 0.375rem;
  }
</style>
