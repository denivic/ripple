<script lang="ts">
  import { onMount } from "svelte";
  import { bisector } from "d3-array";
  import { scaleBand, scaleLinear, scaleTime } from "d3-scale";
  import Toolbar from "$lib/ui/Toolbar.svelte";
  import {
    AreaSeries,
    Axis,
    BarSeries,
    CalendarHeatmap,
    Grid,
    LineSeries,
    Matrix,
    StatTile,
    Tooltip,
    cumulativeSum,
    forwardProjection,
    formatCalendarDate,
    formatDuration,
    formatSignedPercent,
    movingAverage,
    multiScaleTimeFormat,
    recentDailyRate,
    seriesColorVar,
  } from "$lib/charts";
  import { habitsStore, timelineStore } from "$lib/stores";

  onMount(() => {
    void habitsStore.mount();
    void timelineStore.mount();
    return () => {
      habitsStore.unmount();
      timelineStore.unmount();
    };
  });

  const MARGIN = { top: 10, right: 16, bottom: 24, left: 48 };

  function habitValue(item: { timeSpentMinutes: number; lifeShortenedMinutes: number; money: number }): number {
    return breakdownMetric === "life"
      ? item.lifeShortenedMinutes
      : breakdownMetric === "time"
        ? item.timeSpentMinutes
        : item.money;
  }

  // --- 1. Cumulative Ripple (hero) --------------------------------------
  let heroWidth = $state(700);
  const HERO_HEIGHT = 240;
  const PROJECTION_DAYS = 30;
  const RATE_WINDOW_DAYS = 30;

  const dailyTotals = $derived((timelineStore.data?.daily ?? []).map((d) => d.timeSpentMinutes + d.lifeShortenedMinutes));
  const cumulativeValues = $derived(cumulativeSum(dailyTotals));
  const heroRate = $derived(recentDailyRate(dailyTotals, RATE_WINDOW_DAYS));
  const heroLastDate = $derived(timelineStore.data?.daily.at(-1)?.date ?? new Date());
  const heroLastValue = $derived(cumulativeValues.at(-1) ?? 0);
  const heroProjection = $derived(forwardProjection(heroLastDate, heroLastValue, heroRate, PROJECTION_DAYS));

  const heroX = $derived(
    scaleTime()
      .domain([timelineStore.data?.daily[0]?.date ?? new Date(), heroProjection.at(-1)?.date ?? new Date()])
      .range([MARGIN.left, heroWidth - MARGIN.right]),
  );
  const heroYMax = $derived(Math.max(1, ...cumulativeValues, ...heroProjection.map((p) => p.value)));
  const heroY = $derived(
    scaleLinear()
      .domain([0, heroYMax])
      .nice()
      .range([HERO_HEIGHT - MARGIN.bottom, MARGIN.top]),
  );
  const heroBaseline = $derived(heroY(0));
  const heroAreaPoints = $derived(
    (timelineStore.data?.daily ?? []).map((d, i) => ({ x: heroX(d.date), y: heroY(cumulativeValues[i]) })),
  );
  const heroProjectionPoints = $derived(heroProjection.map((p) => ({ x: heroX(p.date), y: heroY(p.value) })));

  let heroHover = $state<{ x: number; date: Date; value: number } | null>(null);
  const dateBisect = bisector<Date, Date>((d) => d).left;

  function onHeroHover(e: PointerEvent) {
    const daily = timelineStore.data?.daily ?? [];
    if (daily.length === 0) return;
    const svg = e.currentTarget as SVGElement;
    const rect = svg.getBoundingClientRect();
    const px = e.clientX - rect.left;
    const hoveredDate = heroX.invert(px);
    const dates = daily.map((d) => d.date);
    let i = dateBisect(dates, hoveredDate);
    i = Math.min(Math.max(i, 0), dates.length - 1);
    heroHover = { x: heroX(dates[i]), date: dates[i], value: cumulativeValues[i] };
  }

  // --- 2. Daily bars + 7-day moving average -----------------------------
  const RECENT_WINDOW_DAYS = 60;
  let barsWidth = $state(700);
  const BARS_HEIGHT = 180;

  const recentDaily = $derived((timelineStore.data?.daily ?? []).slice(-RECENT_WINDOW_DAYS));
  const recentValues = $derived(recentDaily.map((d) => d.timeSpentMinutes + d.lifeShortenedMinutes));
  const recentMA = $derived(movingAverage(recentValues, 7));

  const barsX = $derived(
    scaleBand<number>()
      .domain(recentDaily.map((_, i) => i))
      .range([MARGIN.left, barsWidth - MARGIN.right])
      .padding(0.3),
  );
  const barsYMax = $derived(Math.max(1, ...recentValues));
  const barsY = $derived(
    scaleLinear()
      .domain([0, barsYMax])
      .nice()
      .range([BARS_HEIGHT - MARGIN.bottom, MARGIN.top]),
  );
  const barsBaseline = $derived(barsY(0));
  const barThickness = $derived(Math.min(24, barsX.bandwidth()));
  const bars = $derived(
    recentDaily.map((d, i) => ({
      key: d.date.toISOString(),
      position: (barsX(i) ?? 0) + barsX.bandwidth() / 2,
      length: barsBaseline - barsY(recentValues[i]),
      color: "var(--accent-base)",
      thickness: barThickness,
    })),
  );
  const maPoints = $derived(recentDaily.map((d, i) => ({ x: (barsX(i) ?? 0) + barsX.bandwidth() / 2, y: barsY(recentMA[i]) })));

  let barsHover = $state<{ date: Date; value: number; x: number } | null>(null);

  // --- 3. Calendar heatmap ------------------------------------------------
  const heatmapData = $derived(
    (timelineStore.data?.daily ?? []).map((d) => ({ date: d.date, value: d.timeSpentMinutes + d.lifeShortenedMinutes })),
  );

  // --- 5. Habit breakdown --------------------------------------------------
  let breakdownMetric = $state<"life" | "time" | "money">("life");
  let breakdownWidth = $state(700);
  const BREAKDOWN_ROW_HEIGHT = 32;
  const BREAKDOWN_LABEL_WIDTH = 130;

  const breakdownItems = $derived([...(timelineStore.data?.habitBreakdown ?? [])].sort((a, b) => habitValue(b) - habitValue(a)));
  const breakdownMax = $derived(Math.max(1, ...breakdownItems.map(habitValue)));
  const breakdownX = $derived(scaleLinear().domain([0, breakdownMax]).range([0, breakdownWidth - BREAKDOWN_LABEL_WIDTH - 70]));
  const breakdownBars = $derived(
    breakdownItems.map((item, i) => ({
      key: String(item.habitId),
      position: i * BREAKDOWN_ROW_HEIGHT + BREAKDOWN_ROW_HEIGHT / 2,
      length: breakdownX(habitValue(item)),
      color: seriesColorVar(i),
      thickness: 18,
    })),
  );

  function breakdownValueLabel(value: number): string {
    return breakdownMetric === "money" ? `$${value.toFixed(2)}` : formatDuration(value);
  }

  // --- 6. Streak sparkline --------------------------------------------------
  const SPARK_WINDOW_DAYS = 30;
  let sparkWidth = $state(240);
  const SPARK_HEIGHT = 40;
  const sparkValues = $derived(dailyTotals.slice(-SPARK_WINDOW_DAYS));
  const sparkX = $derived(scaleLinear().domain([0, Math.max(1, sparkValues.length - 1)]).range([2, sparkWidth - 2]));
  const sparkYMax = $derived(Math.max(1, ...sparkValues));
  const sparkY = $derived(scaleLinear().domain([0, sparkYMax]).range([SPARK_HEIGHT - 2, 2]));
  const sparkPoints = $derived(sparkValues.map((v, i) => ({ x: sparkX(i), y: sparkY(v) })));

  // --- 7. Period compare ------------------------------------------------
  const periodDeltaText = $derived.by(() => {
    const pc = timelineStore.data?.periodCompare;
    if (!pc) return "";
    const sign = pc.deltaMinutes >= 0 ? "+" : "";
    const pct = pc.percentChange !== null ? ` (${formatSignedPercent(pc.percentChange / 100)})` : "";
    return `${sign}${formatDuration(pc.deltaMinutes)}${pct}`;
  });
  const periodDirection = $derived.by(() => {
    const pc = timelineStore.data?.periodCompare;
    if (!pc || pc.deltaMinutes === 0) return "flat" as const;
    return pc.deltaMinutes > 0 ? ("up" as const) : ("down" as const);
  });
</script>

<Toolbar title="Timeline" />

<div class="page">
  {#if timelineStore.loading && !timelineStore.data}
    <p class="muted">Loading…</p>
  {:else if timelineStore.data}
    <section class="chart-card">
      <h2>Cumulative Ripple</h2>
      <div class="chart-frame" bind:clientWidth={heroWidth}>
        <svg
          width={heroWidth}
          height={HERO_HEIGHT}
          role="img"
          aria-label="Cumulative time and life lost over time, with a forward projection"
          onpointermove={onHeroHover}
          onpointerleave={() => (heroHover = null)}
        >
          <Grid scale={heroY} width={heroWidth - MARGIN.right} />
          <AreaSeries points={heroAreaPoints} baseline={heroBaseline} />
          <LineSeries points={heroProjectionPoints} dashed color="var(--text-tertiary)" />
          <Axis scale={heroX} orientation="bottom" length={heroWidth} format={(d) => multiScaleTimeFormat(d as unknown as Date)} />
          <Axis scale={heroY} orientation="left" length={HERO_HEIGHT} format={(n) => formatDuration(n as unknown as number)} />
          {#if heroHover}
            <line x1={heroHover.x} x2={heroHover.x} y1={MARGIN.top} y2={HERO_HEIGHT - MARGIN.bottom} class="crosshair" />
            <circle cx={heroHover.x} cy={heroY(heroHover.value)} r="4" fill="var(--accent-base)" stroke="var(--surface-base)" stroke-width="2" />
          {/if}
        </svg>
        <Tooltip x={heroHover?.x ?? 0} y={heroHover ? heroY(heroHover.value) : 0} visible={!!heroHover}>
          {#if heroHover}
            <strong>{formatDuration(heroHover.value)}</strong>
            <span class="muted">{formatCalendarDate(heroHover.date)}</span>
          {/if}
        </Tooltip>
      </div>
      <p class="caption">Dashed line projects forward at the last {RATE_WINDOW_DAYS}-day daily rate.</p>
    </section>

    <section class="chart-card">
      <h2>Daily total, last {RECENT_WINDOW_DAYS} days</h2>
      <div class="chart-frame" bind:clientWidth={barsWidth}>
        <svg width={barsWidth} height={BARS_HEIGHT}>
          <Grid scale={barsY} width={barsWidth - MARGIN.right} />
          <BarSeries
            {bars}
            baseline={barsBaseline}
            orientation="vertical"
            onhover={(bar) => (barsHover = bar ? { date: new Date(bar.key), value: recentValues[bars.indexOf(bar)], x: bar.position } : null)}
          />
          <LineSeries points={maPoints} color="var(--text-primary)" />
          <Axis scale={barsY} orientation="left" length={BARS_HEIGHT} format={(n) => formatDuration(n as unknown as number)} />
        </svg>
        <Tooltip x={barsHover?.x ?? 0} y={barsHover ? barsY(barsHover.value) : 0} visible={!!barsHover}>
          {#if barsHover}
            <strong>{formatDuration(barsHover.value)}</strong>
            <span class="muted">{formatCalendarDate(barsHover.date)}</span>
          {/if}
        </Tooltip>
      </div>
      <p class="caption">Bars: daily total. Line: 7-day moving average.</p>
    </section>

    <section class="chart-card">
      <h2>Calendar</h2>
      <CalendarHeatmap data={heatmapData} start={timelineStore.start} end={timelineStore.end} />
    </section>

    <section class="chart-card">
      <h2>When it happens</h2>
      <Matrix matrix={timelineStore.data.hourWeekdayMatrix} />
    </section>

    <section class="chart-card">
      <div class="card-header">
        <h2>Habit breakdown</h2>
        <div class="metric-toggle">
          <button class:active={breakdownMetric === "life"} onclick={() => (breakdownMetric = "life")}>Life</button>
          <button class:active={breakdownMetric === "time"} onclick={() => (breakdownMetric = "time")}>Time</button>
          <button class:active={breakdownMetric === "money"} onclick={() => (breakdownMetric = "money")}>Money</button>
        </div>
      </div>
      <div class="chart-frame" bind:clientWidth={breakdownWidth}>
        <svg width={breakdownWidth} height={Math.max(1, breakdownItems.length) * BREAKDOWN_ROW_HEIGHT}>
          {#each breakdownItems as item, i (item.habitId)}
            <text x={0} y={i * BREAKDOWN_ROW_HEIGHT + BREAKDOWN_ROW_HEIGHT / 2 + 4} class="row-label">
              {habitsStore.byId(item.habitId)?.name ?? "Unknown"}
            </text>
            <text
              x={BREAKDOWN_LABEL_WIDTH + breakdownX(habitValue(item)) + 8}
              y={i * BREAKDOWN_ROW_HEIGHT + BREAKDOWN_ROW_HEIGHT / 2 + 4}
              class="row-value"
            >
              {breakdownValueLabel(habitValue(item))}
            </text>
          {/each}
          <g transform="translate({BREAKDOWN_LABEL_WIDTH}, 0)">
            <BarSeries bars={breakdownBars} baseline={0} orientation="horizontal" />
          </g>
        </svg>
      </div>
    </section>

    <section class="chart-card stat-row">
      <StatTile label="Current streak" value="{timelineStore.data.currentStreakDays} clean days">
        {#snippet trend()}
          <svg width={sparkWidth} height={SPARK_HEIGHT}>
            <LineSeries points={sparkPoints} color="var(--text-tertiary)" strokeWidth={1.5} />
          </svg>
        {/snippet}
      </StatTile>
      <StatTile label="Longest streak" value="{timelineStore.data.longestStreakDays} clean days" />
      <StatTile
        label="This period vs. last"
        value={formatDuration(timelineStore.data.periodCompare.currentTotalMinutes)}
        delta={{ text: periodDeltaText, direction: periodDirection, goodDirection: "down" }}
      />
    </section>
  {/if}
</div>

<style>
  .page {
    padding: 1.5rem;
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .chart-card {
    background: var(--surface-raised);
    border: 1px solid var(--line-hairline);
    border-radius: 1rem;
    padding: 1.25rem;
  }

  .chart-card h2 {
    font-size: 0.9375rem;
    font-weight: 600;
    color: var(--text-secondary);
    margin: 0 0 0.75rem;
  }

  .card-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }
  .card-header h2 {
    margin: 0;
  }

  .metric-toggle {
    display: flex;
    gap: 0.25rem;
  }
  .metric-toggle button {
    background: none;
    border: 1px solid var(--line-hairline);
    color: var(--text-secondary);
    padding: 0.25rem 0.625rem;
    border-radius: 0.375rem;
    font-size: 0.8125rem;
    cursor: pointer;
  }
  .metric-toggle button.active {
    background: var(--accent-translucent);
    color: var(--text-primary);
    border-color: var(--accent-base);
  }

  .chart-frame {
    position: relative;
    width: 100%;
  }
  .chart-frame svg {
    width: 100%;
    display: block;
  }

  .crosshair {
    stroke: var(--line-hairline);
    stroke-width: 1;
  }

  .row-label {
    font-size: 0.8125rem;
    fill: var(--text-secondary);
  }
  .row-value {
    font-size: 0.75rem;
    fill: var(--text-tertiary);
  }

  .caption {
    margin: 0.75rem 0 0;
    font-size: 0.75rem;
    color: var(--text-tertiary);
  }

  .muted {
    color: var(--text-tertiary);
  }

  .stat-row {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(14rem, 1fr));
    gap: 0.75rem;
    background: none;
    border: none;
    padding: 0;
  }
</style>
