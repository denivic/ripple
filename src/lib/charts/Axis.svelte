<script lang="ts">
  interface Props {
    scale: { (value: never): number; ticks?: (count?: number) => unknown[]; domain: () => unknown[] };
    orientation: "bottom" | "left";
    tickCount?: number;
    format: (value: never) => string;
    length: number;
  }

  let { scale, orientation, tickCount = 5, format, length }: Props = $props();

  const tickValues = $derived((scale.ticks ? scale.ticks(tickCount) : scale.domain()) as never[]);
</script>

<g class="axis axis-{orientation}">
  {#if orientation === "bottom"}
    <line x1={0} y1={0} x2={length} y2={0} class="baseline" />
  {:else}
    <line x1={0} y1={0} x2={0} y2={length} class="baseline" />
  {/if}
  {#each tickValues as value, i (i)}
    {@const offset = scale(value)}
    {#if orientation === "bottom"}
      <text x={offset} y="18" text-anchor="middle">{format(value)}</text>
    {:else}
      <text x="-10" y={offset} text-anchor="end" dy="0.32em">{format(value)}</text>
    {/if}
  {/each}
</g>

<style>
  text {
    font-size: 0.6875rem;
    fill: var(--text-tertiary);
  }
  .baseline {
    stroke: var(--line-hairline);
    stroke-width: 1;
  }
</style>
