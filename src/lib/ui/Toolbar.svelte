<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    title?: string;
    children?: Snippet;
  }

  let { title, children }: Props = $props();
</script>

<header class="toolbar">
  {#if title}
    <h1 class="title">{title}</h1>
  {/if}
  {#if children}
    <div class="actions">
      {@render children()}
    </div>
  {/if}
</header>

<style>
  .toolbar {
    position: sticky;
    top: 0;
    z-index: 10;
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 1rem;
    padding: 1rem 1.5rem;
    background: color-mix(in oklch, var(--surface-base) calc(var(--material-alpha) * 100%), transparent);
    backdrop-filter: blur(var(--material-blur)) saturate(var(--material-saturate));
    -webkit-backdrop-filter: blur(var(--material-blur)) saturate(var(--material-saturate));
  }

  /* Edge fade where floating chrome meets scrolling content, in place of a
     hard 1px divider. */
  .toolbar::after {
    content: "";
    position: absolute;
    left: 0;
    right: 0;
    bottom: -1.5rem;
    height: 1.5rem;
    pointer-events: none;
    background: linear-gradient(to bottom, color-mix(in oklch, var(--surface-base) 55%, transparent), transparent);
  }

  .title {
    font-size: 1.25rem;
    font-weight: 600;
  }

  .actions {
    display: flex;
    align-items: center;
    gap: 0.625rem;
  }
</style>
