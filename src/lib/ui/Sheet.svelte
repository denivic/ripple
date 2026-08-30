<script lang="ts">
  import type { Snippet } from "svelte";
  import { Spring } from "svelte/motion";
  import { toSpringOptions } from "$lib/theme/motion";
  import { themeStore } from "$lib/theme/tokens.svelte";

  interface Props {
    open: boolean;
    title?: string;
    onclose: () => void;
    children: Snippet;
  }

  let { open, title, onclose, children }: Props = $props();

  // Critically damped: a sheet opening on its own isn't a momentum gesture,
  // so no overshoot. Deliberately reads `open` only for its starting value —
  // a sheet mounted already-open shouldn't animate in on load; the $effect
  // below drives every update after that.
  // svelte-ignore state_referenced_locally
  const progress = new Spring(open ? 1 : 0, toSpringOptions(themeStore.current.motion));

  $effect(() => {
    progress.target = open ? 1 : 0;
  });

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === "Escape" && open) onclose();
  }
</script>

<svelte:window onkeydown={handleKeydown} />

{#if open || progress.current > 0.001}
  <div
    class="scrim"
    style:opacity={progress.current}
    onclick={onclose}
    onkeydown={(e) => e.key === "Enter" && onclose()}
    role="button"
    tabindex="-1"
    aria-label="Close"
  ></div>
  <div
    class="sheet"
    style:opacity={progress.current}
    style:transform="translateY({(1 - progress.current) * 12}px) scale({0.98 + progress.current * 0.02})"
    role="dialog"
    aria-modal="true"
    aria-label={title}
  >
    {#if title}
      <div class="sheet-header">
        <h2>{title}</h2>
        <button class="close" onclick={onclose} aria-label="Close">✕</button>
      </div>
    {/if}
    <div class="sheet-body">
      {@render children()}
    </div>
  </div>
{/if}

<style>
  .scrim {
    position: fixed;
    inset: 0;
    z-index: 40;
    background: rgba(0, 0, 0, 0.5);
    backdrop-filter: blur(4px);
    -webkit-backdrop-filter: blur(4px);
    border: none;
    padding: 0;
    cursor: default;
  }

  .sheet {
    position: fixed;
    top: 50%;
    left: 50%;
    z-index: 41;
    width: min(28rem, calc(100vw - 2.5rem));
    max-height: calc(100vh - 4rem);
    overflow: auto;
    translate: -50% -50%;
    background: var(--surface-raised);
    border: 1px solid var(--line-hairline);
    border-radius: 1rem;
    box-shadow: 0 24px 48px rgba(0, 0, 0, 0.4);
    padding: 1.25rem;
  }

  .sheet-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 1rem;
  }
  .sheet-header h2 {
    font-size: 1.0625rem;
    font-weight: 600;
  }

  .close {
    background: none;
    border: none;
    color: var(--text-secondary);
    font-size: 0.875rem;
    cursor: pointer;
    padding: 0.25rem 0.5rem;
    border-radius: 0.375rem;
  }
  .close:hover {
    background: var(--surface-hover);
    color: var(--text-primary);
  }
</style>
