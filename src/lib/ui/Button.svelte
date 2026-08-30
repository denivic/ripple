<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    variant?: "primary" | "secondary" | "ghost";
    disabled?: boolean;
    type?: "button" | "submit";
    onclick?: (e: MouseEvent) => void;
    children: Snippet;
  }

  let { variant = "secondary", disabled = false, type = "button", onclick, children }: Props = $props();
</script>

<button class="btn btn-{variant}" {type} {disabled} {onclick}>
  {@render children()}
</button>

<style>
  .btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
    padding: 0.625rem 1rem;
    border-radius: 0.625rem;
    border: 1px solid transparent;
    font-size: 0.9375rem;
    font-weight: 500;
    cursor: pointer;
    /* Discrete press feedback, not a grabbable gesture: a short CSS
       transition is the right tool here (apple-design §1's own example). */
    transition:
      transform 100ms ease-out,
      background-color 120ms ease-out,
      border-color 120ms ease-out;
  }

  /* Feedback fires on pointer-down (:active), never waiting for release. */
  .btn:active:not(:disabled) {
    transform: scale(0.97);
  }

  .btn:disabled {
    opacity: 0.45;
    cursor: not-allowed;
  }

  .btn:focus-visible {
    outline: 2px solid var(--focus-ring);
    outline-offset: 2px;
  }

  .btn-primary {
    background: var(--accent-base);
    color: var(--surface-base);
  }
  .btn-primary:hover:not(:disabled) {
    background: color-mix(in oklch, var(--accent-base) 90%, white 10%);
  }

  .btn-secondary {
    background: var(--surface-raised);
    color: var(--text-primary);
    border-color: var(--line-hairline);
  }
  .btn-secondary:hover:not(:disabled) {
    background: var(--surface-hover);
  }

  .btn-ghost {
    background: transparent;
    color: var(--text-secondary);
  }
  .btn-ghost:hover:not(:disabled) {
    background: var(--surface-hover);
    color: var(--text-primary);
  }
</style>
