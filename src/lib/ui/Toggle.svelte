<script lang="ts">
  import { Spring } from "svelte/motion";
  import { toSpringOptions } from "$lib/theme/motion";
  import { themeStore } from "$lib/theme/tokens.svelte";

  interface Props {
    checked?: boolean;
    disabled?: boolean;
    label?: string;
    onchange?: (checked: boolean) => void;
  }

  let { checked = $bindable(false), disabled = false, label, onchange }: Props = $props();

  const TRAVEL_PX = 20;
  // A toggle flip is a reposition, not a flick — critically damped, no bounce.
  const knob = new Spring(checked ? 1 : 0, toSpringOptions(themeStore.current.motion));

  $effect(() => {
    knob.target = checked ? 1 : 0;
  });

  function toggle() {
    if (disabled) return;
    checked = !checked;
    onchange?.(checked);
  }
</script>

<button
  type="button"
  class="toggle"
  role="switch"
  aria-checked={checked}
  aria-label={label ? undefined : "Toggle"}
  {disabled}
  onclick={toggle}
>
  <span class="track" class:on={checked}>
    <span class="knob" style:transform="translateX({knob.current * TRAVEL_PX}px)"></span>
  </span>
  {#if label}
    <span class="label">{label}</span>
  {/if}
</button>

<style>
  .toggle {
    display: inline-flex;
    align-items: center;
    gap: 0.625rem;
    background: none;
    border: none;
    padding: 0;
    cursor: pointer;
    color: var(--text-primary);
    font-size: 0.9375rem;
  }
  .toggle:disabled {
    opacity: 0.45;
    cursor: not-allowed;
  }
  .toggle:focus-visible .track {
    outline: 2px solid var(--focus-ring);
    outline-offset: 2px;
  }

  .track {
    position: relative;
    width: 2.75rem;
    height: 1.5rem;
    border-radius: 999px;
    background: var(--surface-raised);
    border: 1px solid var(--line-hairline);
    transition: background-color 150ms ease-out;
    flex-shrink: 0;
  }
  .track.on {
    background: var(--accent-base);
    border-color: var(--accent-base);
  }

  .knob {
    position: absolute;
    top: 1px;
    left: 1px;
    width: 1.25rem;
    height: 1.25rem;
    border-radius: 50%;
    background: var(--text-primary);
    box-shadow: 0 1px 2px rgba(0, 0, 0, 0.3);
  }

  .label {
    color: var(--text-secondary);
  }
</style>
