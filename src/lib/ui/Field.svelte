<script lang="ts">
  interface Props {
    label: string;
    type?: "text" | "number" | "date" | "time" | "email" | "password" | "select";
    value?: string | number;
    placeholder?: string;
    error?: string;
    hint?: string;
    disabled?: boolean;
    options?: { value: string; label: string }[];
    oninput?: (value: string) => void;
  }

  let {
    label,
    type = "text",
    value = $bindable(""),
    placeholder,
    error,
    hint,
    disabled = false,
    options,
    oninput,
  }: Props = $props();

  function handleInput(e: Event) {
    const target = e.currentTarget as HTMLInputElement;
    value = type === "number" ? target.valueAsNumber : target.value;
    oninput?.(target.value);
  }

  function handleSelect(e: Event) {
    const target = e.currentTarget as HTMLSelectElement;
    value = target.value;
    oninput?.(target.value);
  }
</script>

<label class="field">
  <span class="field-label">{label}</span>
  {#if type === "select"}
    <select class="field-input" class:invalid={!!error} {value} {disabled} onchange={handleSelect}>
      {#each options ?? [] as option (option.value)}
        <option value={option.value}>{option.label}</option>
      {/each}
    </select>
  {:else}
    <input class="field-input" class:invalid={!!error} {type} {value} {placeholder} {disabled} oninput={handleInput} />
  {/if}
  {#if error}
    <span class="field-message error">{error}</span>
  {:else if hint}
    <span class="field-message hint">{hint}</span>
  {/if}
</label>

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

  .field-input {
    background: var(--surface-raised);
    border: 1px solid var(--line-hairline);
    border-radius: 0.5rem;
    padding: 0.5rem 0.625rem;
    color: var(--text-primary);
    font-size: 0.9375rem;
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
  .field-input.invalid {
    border-color: #f87171;
  }

  select.field-input {
    cursor: pointer;
  }

  .field-message {
    font-size: 0.75rem;
  }
  .field-message.error {
    color: #f87171;
  }
  .field-message.hint {
    color: var(--text-tertiary);
  }
</style>
