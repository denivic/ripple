<script lang="ts">
  import { settingsStore } from "$lib/stores";
  import { formatClockValue, fromHour12, minuteOptions, parseClockValue, toHour12, type Period } from "./time-field";

  interface Props {
    label: string;
    /** Always 24-hour "HH:MM", whichever clock style the selects show. */
    value: string;
    hint?: string;
    disabled?: boolean;
    oninput?: (value: string) => void;
  }

  let { label, value, hint, disabled = false, oninput }: Props = $props();

  // Replaces `<input type="time">`, whose clock style follows the OS locale
  // rather than the app's time-format setting.
  const id = $props.id();
  const twelveHour = $derived(settingsStore.data.timeFormat === "12h");
  const parts = $derived(parseClockValue(value) ?? { hour: 0, minute: 0 });
  const hour12 = $derived(toHour12(parts.hour));
  const minutes = $derived(minuteOptions(parts.minute));

  const pad = (n: number) => String(n).padStart(2, "0");

  function emit(hour: number, minute: number) {
    oninput?.(formatClockValue(hour, minute));
  }

  function selectValue(e: Event): number {
    return Number((e.currentTarget as HTMLSelectElement).value);
  }
</script>

<div class="field" role="group" aria-labelledby="{id}-label" aria-describedby={hint ? `${id}-hint` : undefined}>
  <span class="field-label" id="{id}-label">{label}</span>
  <div class="selects">
    {#if twelveHour}
      <select
        class="field-input"
        aria-label="Hour"
        value={hour12.hour12}
        {disabled}
        onchange={(e) => emit(fromHour12(selectValue(e), hour12.period), parts.minute)}
      >
        {#each [12, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11] as h (h)}
          <option value={h}>{h}</option>
        {/each}
      </select>
    {:else}
      <select
        class="field-input"
        aria-label="Hour"
        value={parts.hour}
        {disabled}
        onchange={(e) => emit(selectValue(e), parts.minute)}
      >
        {#each Array.from({ length: 24 }, (_, i) => i) as h (h)}
          <option value={h}>{pad(h)}</option>
        {/each}
      </select>
    {/if}
    <span class="colon" aria-hidden="true">:</span>
    <select
      class="field-input"
      aria-label="Minute"
      value={parts.minute}
      {disabled}
      onchange={(e) => emit(parts.hour, selectValue(e))}
    >
      {#each minutes as m (m)}
        <option value={m}>{pad(m)}</option>
      {/each}
    </select>
    {#if twelveHour}
      <select
        class="field-input"
        aria-label="AM or PM"
        value={hour12.period}
        {disabled}
        onchange={(e) =>
          emit(fromHour12(hour12.hour12, (e.currentTarget as HTMLSelectElement).value as Period), parts.minute)}
      >
        <option value="AM">AM</option>
        <option value="PM">PM</option>
      </select>
    {/if}
  </div>
  {#if hint}
    <span class="field-message hint" id="{id}-hint">{hint}</span>
  {/if}
</div>

<style>
  .field {
    display: flex;
    flex-direction: column;
    gap: 0.375rem;
  }

  .field-label {
    font-size: 0.8125rem;
    color: var(--text-secondary);
  }

  .selects {
    display: flex;
    align-items: center;
    gap: 0.375rem;
  }

  .colon {
    color: var(--text-secondary);
  }

  .field-input {
    background: var(--surface-raised);
    border: 1px solid var(--line-hairline);
    border-radius: 0.5rem;
    padding: 0.5rem 0.625rem;
    color: var(--text-primary);
    font-size: 0.9375rem;
    font-variant-numeric: tabular-nums;
    cursor: pointer;
    transition: border-color 120ms ease-out;
  }
  .field-input:focus-visible {
    outline: none;
    border-color: var(--accent-base);
    box-shadow: 0 0 0 2px var(--accent-translucent);
  }
  .field-input:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .field-message {
    font-size: 0.75rem;
  }
  .field-message.hint {
    color: var(--text-tertiary);
  }
</style>
