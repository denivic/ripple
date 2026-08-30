<script lang="ts">
  interface Props {
    scale: { (value: number): number; ticks: (count?: number) => number[] };
    tickCount?: number;
    width: number;
  }

  let { scale, tickCount = 5, width }: Props = $props();

  const tickValues = $derived(scale.ticks(tickCount));
</script>

<g class="grid">
  {#each tickValues as value (value)}
    <line x1={0} x2={width} y1={scale(value)} y2={scale(value)} />
  {/each}
</g>

<style>
  .grid line {
    stroke: var(--line-hairline);
    stroke-width: 1;
    opacity: 0.6;
  }
</style>
