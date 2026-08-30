<script lang="ts">
  import { area as d3area, curveMonotoneX } from "d3-shape";

  interface Point {
    x: number;
    y: number;
  }

  interface Props {
    points: Point[];
    baseline: number;
    color?: string;
  }

  let { points, baseline, color = "var(--accent-base)" }: Props = $props();

  const path = $derived(
    d3area<Point>()
      .x((d) => d.x)
      .y0(baseline)
      .y1((d) => d.y)
      .curve(curveMonotoneX)(points) ?? "",
  );

  const linePath = $derived(
    d3area<Point>()
      .x((d) => d.x)
      .y0(baseline)
      .y1((d) => d.y)
      .curve(curveMonotoneX)
      .lineY1()(points) ?? "",
  );
</script>

<!-- Area fill: series hue at ~10% opacity, a wash rather than a solid block. -->
<path d={path} fill={color} opacity="0.1" stroke="none" />
<path
  d={linePath}
  fill="none"
  stroke={color}
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
/>
