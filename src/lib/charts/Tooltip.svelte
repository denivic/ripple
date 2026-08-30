<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    x: number;
    y: number;
    visible: boolean;
    children: Snippet;
  }

  let { x, y, visible, children }: Props = $props();
</script>

{#if visible}
  <div class="tooltip" style:left="{x}px" style:top="{y}px">
    {@render children()}
  </div>
{/if}

<style>
  .tooltip {
    position: absolute;
    pointer-events: none;
    transform: translate(-50%, calc(-100% - 10px));
    background: var(--surface-raised);
    border: 1px solid var(--line-hairline);
    border-radius: 0.5rem;
    padding: 0.5rem 0.625rem;
    font-size: 0.8125rem;
    color: var(--text-primary);
    box-shadow: 0 8px 20px rgba(0, 0, 0, 0.35);
    white-space: nowrap;
    z-index: 20;
  }
</style>
