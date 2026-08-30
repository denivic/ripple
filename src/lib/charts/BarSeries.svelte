<script lang="ts">
  import { horizontalBarPath, verticalBarPath } from "./bar-path";

  export interface Bar {
    key: string;
    /** Center position along the categorical/time axis, in px. */
    position: number;
    /** Pixel length from the baseline to the value; always >= 0. */
    length: number;
    color: string;
    thickness: number;
  }

  interface Props {
    bars: Bar[];
    baseline: number;
    orientation: "vertical" | "horizontal";
    onhover?: (bar: Bar | null, event: PointerEvent) => void;
  }

  let { bars, baseline, orientation, onhover }: Props = $props();
</script>

<g class="bars">
  {#each bars as bar (bar.key)}
    <path
      d={orientation === "vertical"
        ? verticalBarPath(bar.position - bar.thickness / 2, baseline - bar.length, bar.thickness, bar.length)
        : horizontalBarPath(baseline, bar.position - bar.thickness / 2, bar.length, bar.thickness)}
      fill={bar.color}
      role="img"
      aria-label={bar.key}
      onpointerenter={(e) => onhover?.(bar, e)}
      onpointermove={(e) => onhover?.(bar, e)}
      onpointerleave={(e) => onhover?.(null, e)}
    />
  {/each}
</g>
