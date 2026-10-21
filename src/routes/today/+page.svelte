<script lang="ts">
  import { onMount } from "svelte";
  import Toolbar from "$lib/ui/Toolbar.svelte";
  import Button from "$lib/ui/Button.svelte";
  import Field from "$lib/ui/Field.svelte";
  import Sheet from "$lib/ui/Sheet.svelte";
  import { StatTile, formatDuration, formatMoney, formatMoneyCompactParts, formatPercent } from "$lib/charts";
  import { habitsStore, settingsStore, todayStore } from "$lib/stores";
  import type { HabitPreset } from "$lib/ipc";
  import { filterPresets, formatPresetSource, groupPresets } from "$lib/habit-presets";
  import { openUrl } from "@tauri-apps/plugin-opener";

  onMount(() => {
    void habitsStore.mount();
    void todayStore.mount();
    return () => {
      habitsStore.unmount();
      todayStore.unmount();
    };
  });

  let addHabitOpen = $state(false);
  let newHabitName = $state("");
  let newHabitUnit = $state("");
  let newHabitLifeMinutes = $state(0);
  let newHabitCost = $state(0);

  let presetQuery = $state("");
  let selectedPreset = $state<HabitPreset | null>(null);
  let presetListEl: HTMLDivElement | undefined = $state();
  const presetGroups = $derived(groupPresets(filterPresets(todayStore.presets, presetQuery)));
  const selectedPresetSource = $derived(selectedPreset ? formatPresetSource(selectedPreset) : null);
  // Roving tabindex: one Tab stop for the whole list — the chosen preset if
  // it's still visible, else the first match.
  const visiblePresets = $derived(presetGroups.flatMap((g) => g.presets));
  const presetTabStopId = $derived(
    visiblePresets.find((p) => p.id === selectedPreset?.id)?.id ?? visiblePresets[0]?.id,
  );

  // The preset only pre-fills the form; every field stays editable, and cost
  // is always the user's own figure.
  function choosePreset(preset: HabitPreset) {
    selectedPreset = preset;
    newHabitName = preset.name;
    newHabitUnit = preset.unitLabel;
    newHabitLifeMinutes = preset.lifeMinutesPerUnit;
  }

  function presetOptions(): HTMLButtonElement[] {
    return presetListEl ? Array.from(presetListEl.querySelectorAll<HTMLButtonElement>("button[data-preset]")) : [];
  }

  // Arrow keys move between preset options (and down from the search box
  // into the list), so the list doesn't need one Tab stop per preset.
  function handlePresetKeydown(e: KeyboardEvent) {
    const options = presetOptions();
    if (options.length === 0) return;
    const index = options.indexOf(document.activeElement as HTMLButtonElement);
    let next: number;
    if (e.key === "ArrowDown") next = index < 0 ? 0 : Math.min(index + 1, options.length - 1);
    else if (e.key === "ArrowUp") next = Math.max(index - 1, 0);
    else if (e.key === "Home" && index >= 0) next = 0;
    else if (e.key === "End" && index >= 0) next = options.length - 1;
    else return;
    e.preventDefault();
    options[next].focus();
  }

  function closeAddHabit() {
    addHabitOpen = false;
    presetQuery = "";
    selectedPreset = null;
  }

  async function addCustomHabit() {
    if (!newHabitName.trim() || !newHabitUnit.trim()) return;
    await habitsStore.create({
      name: newHabitName.trim(),
      unitLabel: newHabitUnit.trim(),
      lifeMinutesPerUnit: Number(newHabitLifeMinutes) || 0,
      costPerUnit: Number(newHabitCost) || 0,
      color: null,
      archived: false,
    });
    newHabitName = "";
    newHabitUnit = "";
    newHabitLifeMinutes = 0;
    newHabitCost = 0;
    closeAddHabit();
  }

  // Life expectancy can be fractional (e.g. 81.5); whole ages print bare.
  // Split so the tile can size the figure and wrap the currency separately.
  const spentParts = $derived(formatMoneyCompactParts(todayStore.summary?.moneySpent ?? 0));

  function formatAge(years: number): string {
    return String(Number(years.toFixed(1)));
  }

  async function logOne(habitId: number) {
    await todayStore.log(habitId, 1);
  }
</script>

<Toolbar title="Today">
  <Button variant="secondary" onclick={() => (addHabitOpen = true)}>Add habit</Button>
</Toolbar>

{#if habitsStore.error || todayStore.error}
  <div class="banner error">{habitsStore.error ?? todayStore.error}</div>
{/if}

<div class="page">
  {#if todayStore.summary}
    <section class="toll" aria-label="Today's running toll">
      <StatTile label="Time spent" value={formatDuration(todayStore.summary.timeSpentMinutes)} />
      {#if todayStore.summary.lifeShortenedMinutes < 0}
        <StatTile label="Life gained (est.)" value={formatDuration(-todayStore.summary.lifeShortenedMinutes)} />
      {:else}
        <StatTile label="Life shortened (est.)" value={formatDuration(todayStore.summary.lifeShortenedMinutes)} />
      {/if}
      <StatTile
        label="Spent"
        value={spentParts.figure}
        unit={spentParts.unit}
        unitPosition={spentParts.unitPosition}
        title={formatMoney(todayStore.summary.moneySpent)}
      />
      <StatTile label="Share of today's waking hours" value={formatPercent(todayStore.summary.wakingLifeShareToday)} />
      {#if todayStore.summary.opportunityCost !== null}
        {@const cost = formatMoneyCompactParts(todayStore.summary.opportunityCost)}
        <StatTile
          label="Opportunity cost"
          value={cost.figure}
          unit={cost.unit}
          unitPosition={cost.unitPosition}
          title={formatMoney(todayStore.summary.opportunityCost)}
        />
      {/if}
      {#if todayStore.summary.projectedWakingMonthsLost !== null && todayStore.summary.lifeExpectancyAge !== null}
        {@const months = todayStore.summary.projectedWakingMonthsLost}
        {@const age = formatAge(todayStore.summary.lifeExpectancyAge)}
        <StatTile
          label="At your {todayStore.summary.projectionWindowDays}-day pace"
          value="~{Math.abs(months).toFixed(1)} months of waking life {months < 0 ? 'gained' : 'lost'} by age {age}"
          hint="An estimate assuming you keep this average daily pace until age {age}, counted in waking hours (24 − your sleep hours from Profile). It is not how long you have left."
        />
      {/if}
    </section>
  {/if}

  <section class="targets" aria-label="Log a habit">
    {#if habitsStore.loading}
      <p class="muted">Loading…</p>
    {:else if habitsStore.items.length === 0}
      <div class="empty">
        <p>No habits yet. Start from a cited preset or add your own.</p>
        <div class="presets">
          <Button variant="secondary" onclick={() => (addHabitOpen = true)}>Add a habit…</Button>
        </div>
      </div>
    {:else}
      <div class="target-grid">
        {#each habitsStore.items as habit (habit.id)}
          <button class="tap-target" onclick={() => logOne(habit.id)}>
            <span class="name">{habit.name}</span>
            <span class="unit">+1 {habit.unitLabel}</span>
          </button>
        {/each}
      </div>
    {/if}
  </section>

  <section class="log" aria-label="Today's entries">
    <h2>Logged today</h2>
    {#if todayStore.entries.length === 0}
      <p class="muted">Nothing logged yet today.</p>
    {:else}
      <ul class="entry-list">
        {#each todayStore.entries as entry (entry.id)}
          {@const habit = habitsStore.byId(entry.habitId)}
          <li>
            <span class="entry-name">{habit?.name ?? "Unknown habit"}</span>
            <span class="muted">{entry.quantity} {habit?.unitLabel ?? ""}</span>
            <button class="remove" onclick={() => todayStore.remove(entry.id)} aria-label="Remove entry">✕</button>
          </li>
        {/each}
      </ul>
    {/if}
  </section>
</div>

<Sheet open={addHabitOpen} title="Add a habit" onclose={closeAddHabit}>
  <div class="form">
    <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
    <div class="preset-picker" role="group" aria-label="Start from a preset" onkeydown={handlePresetKeydown}>
      <Field label="Start from a preset (optional)" placeholder="Search presets" bind:value={presetQuery} />
      <div class="preset-list" bind:this={presetListEl}>
        {#each presetGroups as group (group.category)}
          <div class="preset-group" role="group" aria-labelledby="preset-group-{group.category}">
            <h3 id="preset-group-{group.category}">{group.label}</h3>
            {#each group.presets as preset (preset.id)}
              <button
                type="button"
                class="preset-option"
                data-preset
                aria-pressed={selectedPreset?.id === preset.id}
                tabindex={preset.id === presetTabStopId ? 0 : -1}
                onclick={() => choosePreset(preset)}
              >
                <span>{preset.name}</span>
                <span class="preset-meta">
                  {#if preset.confidence === "insufficient"}
                    no estimate
                  {:else}
                    {preset.lifeMinutesPerUnit < 0 ? "+" : "−"}{Math.abs(preset.lifeMinutesPerUnit)} min / {preset.unitLabel}
                  {/if}
                </span>
              </button>
            {/each}
          </div>
        {:else}
          <p class="muted">No presets match “{presetQuery}”.</p>
        {/each}
      </div>
    </div>

    <Field label="Name" bind:value={newHabitName} />
    <Field label="Unit (e.g. cigarette, drink)" bind:value={newHabitUnit} />
    <Field label="Life effect per unit (minutes; negative = gained)" type="number" bind:value={newHabitLifeMinutes} />
    <Field label="Cost per unit ({settingsStore.data.currency})" type="number" bind:value={newHabitCost} />

    {#if selectedPreset}
      <div class="preset-source" aria-live="polite">
        {#if selectedPreset.confidence === "insufficient"}
          <p class="insufficient">No reliable estimate — set your own value.</p>
        {/if}
        <p>{selectedPreset.note}</p>
        {#if selectedPreset.derivation}
          <p class="muted">Derived: {selectedPreset.derivation}</p>
        {/if}
        {#if selectedPresetSource}
          <p class="muted">
            {selectedPresetSource}
            {#if selectedPreset.sourceUrl}
              {@const url = selectedPreset.sourceUrl}
              · <button type="button" class="link" onclick={() => void openUrl(url)}>Open source</button>
            {/if}
          </p>
        {/if}
      </div>
    {/if}

    <Button variant="primary" onclick={addCustomHabit}>Add habit</Button>
  </div>
</Sheet>

<style>
  .banner {
    margin: 0 1.5rem;
    padding: 0.5rem 0.875rem;
    border-radius: 0.5rem;
    font-size: 0.8125rem;
  }
  .banner.error {
    background: rgba(248, 113, 113, 0.15);
    color: #f87171;
  }

  .page {
    padding: 1.5rem;
    display: flex;
    flex-direction: column;
    gap: 2rem;
  }

  .toll {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(11rem, 1fr));
    gap: 0.75rem;
  }

  .target-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(10rem, 1fr));
    gap: 0.75rem;
  }

  .tap-target {
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    gap: 0.375rem;
    min-height: 6rem;
    padding: 1.125rem;
    border-radius: 1rem;
    border: 1px solid var(--line-hairline);
    background: var(--surface-raised);
    color: var(--text-primary);
    cursor: pointer;
    text-align: left;
    transition: transform 100ms ease-out, background-color 120ms ease-out;
  }
  .tap-target:hover {
    background: var(--surface-hover);
  }
  .tap-target:active {
    transform: scale(0.97);
    background: var(--accent-translucent);
  }

  .tap-target .name {
    font-size: 1rem;
    font-weight: 600;
  }
  .tap-target .unit {
    font-size: 0.8125rem;
    color: var(--accent-base);
  }

  .empty {
    padding: 2rem;
    text-align: center;
    color: var(--text-secondary);
  }
  .presets {
    display: flex;
    gap: 0.5rem;
    justify-content: center;
    margin-top: 0.75rem;
  }

  h2 {
    font-size: 0.9375rem;
    font-weight: 600;
    color: var(--text-secondary);
    margin-bottom: 0.75rem;
  }

  .muted {
    color: var(--text-tertiary);
    font-size: 0.875rem;
  }

  .entry-list {
    list-style: none;
    margin: 0;
    padding: 0;
    display: flex;
    flex-direction: column;
    gap: 0.375rem;
  }
  .entry-list li {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 0.625rem 0.875rem;
    background: var(--surface-raised);
    border: 1px solid var(--line-hairline);
    border-radius: 0.625rem;
  }
  .entry-name {
    font-weight: 500;
    flex: 1;
  }
  .remove {
    background: none;
    border: none;
    color: var(--text-tertiary);
    cursor: pointer;
    padding: 0.25rem 0.5rem;
    border-radius: 0.375rem;
  }
  .remove:hover {
    background: var(--surface-hover);
    color: var(--text-primary);
  }

  .form {
    display: flex;
    flex-direction: column;
    gap: 0.875rem;
  }

  .preset-picker {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }
  .preset-list {
    max-height: 14rem;
    overflow-y: auto;
    border: 1px solid var(--line-hairline);
    border-radius: 0.5rem;
    padding: 0.25rem;
  }
  .preset-group h3 {
    font-size: 0.6875rem;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.04em;
    color: var(--text-tertiary);
    padding: 0.5rem 0.5rem 0.25rem;
  }
  .preset-option {
    display: flex;
    width: 100%;
    justify-content: space-between;
    gap: 0.75rem;
    padding: 0.375rem 0.5rem;
    border: none;
    border-radius: 0.375rem;
    background: none;
    color: var(--text-primary);
    font-size: 0.875rem;
    text-align: left;
    cursor: pointer;
  }
  .preset-option:hover {
    background: var(--surface-hover);
  }
  .preset-option:focus-visible {
    outline: none;
    box-shadow: 0 0 0 2px var(--accent-translucent);
  }
  .preset-option[aria-pressed="true"] {
    background: var(--accent-translucent);
  }
  .preset-meta {
    color: var(--text-tertiary);
    font-size: 0.75rem;
    white-space: nowrap;
  }

  .preset-source {
    display: flex;
    flex-direction: column;
    gap: 0.375rem;
    font-size: 0.8125rem;
    color: var(--text-secondary);
  }
  .preset-source .muted {
    font-size: 0.75rem;
  }
  .insufficient {
    color: var(--accent-base);
    font-weight: 600;
  }
  .link {
    background: none;
    border: none;
    padding: 0;
    color: var(--accent-base);
    font: inherit;
    text-decoration: underline;
    cursor: pointer;
  }
</style>
