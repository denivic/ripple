<script lang="ts">
  import { curveMonotoneX, line as d3line } from "d3-shape";

  interface Point {
    x: number;
    y: number;
  }

  interface Props {
    points: Point[];
    color?: string;
    strokeWidth?: number;
    dashed?: boolean;
  }

  let { points, color = "var(--accent-base)", strokeWidth = 2, dashed = false }: Props = $props();

  const path = $derived(
    d3line<Point>()
      .x((d) => d.x)
      .y((d) => d.y)
      .curve(curveMonotoneX)(points) ?? "",
  );
</script>

<path
  d={path}
  fill="none"
  stroke={color}
  stroke-width={strokeWidth}
  stroke-linecap="round"
  stroke-linejoin="round"
  stroke-dasharray={dashed ? "5 4" : undefined}
/>
