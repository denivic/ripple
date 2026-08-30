<script lang="ts">
  import { onMount } from "svelte";
  import Toolbar from "$lib/ui/Toolbar.svelte";
  import Button from "$lib/ui/Button.svelte";
  import Field from "$lib/ui/Field.svelte";
  import Sheet from "$lib/ui/Sheet.svelte";
  import { StatTile, formatDuration, formatMoney, formatPercent } from "$lib/charts";
  import { habitsStore, todayStore } from "$lib/stores";
  import type { HabitPreset } from "$lib/ipc";

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

  async function addPreset(preset: HabitPreset) {
    await habitsStore.create({
      name: preset.name,
      unitLabel: preset.unitLabel,
      lifeMinutesPerUnit: preset.lifeMinutesPerUnit,
      costPerUnit: 0,
      color: null,
      archived: false,
    });
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
    addHabitOpen = false;
  }

  async function logOne(habitId: number) {
    await todayStore.log(habitId, 1);
  }
</script>

<Toolbar title="Today">
  <Button variant="secondary" onclick={() => (addHabitOpen = true)}>Add habit</Button>
</Toolbar>

<div class="page">
  {#if todayStore.summary}
    <section class="toll" aria-label="Today's running toll">
      <StatTile label="Time spent" value={formatDuration(todayStore.summary.timeSpentMinutes)} />
      <StatTile label="Life shortened (est.)" value={formatDuration(todayStore.summary.lifeShortenedMinutes)} />
      <StatTile label="Spent" value={formatMoney(todayStore.summary.moneySpent)} />
      <StatTile label="Share of today's waking hours" value={formatPercent(todayStore.summary.wakingLifeShareToday)} />
      {#if todayStore.summary.opportunityCost !== null}
        <StatTile label="Opportunity cost" value={formatMoney(todayStore.summary.opportunityCost)} />
      {/if}
      {#if todayStore.summary.remainingWakingLifeMonthsAtTodaysRate !== null}
        <StatTile
          label="At this rate"
          value="{todayStore.summary.remainingWakingLifeMonthsAtTodaysRate.toFixed(1)} months of remaining waking life"
        />
      {/if}
    </section>
  {/if}

  <section class="targets" aria-label="Log a habit">
    {#if habitsStore.items.length === 0}
      <div class="empty">
        <p>No habits yet. Start with a preset or add your own.</p>
        <div class="presets">
          {#each todayStore.presets as preset (preset.name)}
            <Button variant="secondary" onclick={() => addPreset(preset)}>{preset.name}</Button>
          {/each}
          <Button variant="ghost" onclick={() => (addHabitOpen = true)}>Custom…</Button>
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

<Sheet open={addHabitOpen} title="Add a habit" onclose={() => (addHabitOpen = false)}>
  <div class="form">
    <Field label="Name" bind:value={newHabitName} />
    <Field label="Unit (e.g. cigarette, drink)" bind:value={newHabitUnit} />
    <Field label="Life shortened per unit (minutes)" type="number" bind:value={newHabitLifeMinutes} />
    <Field label="Cost per unit" type="number" bind:value={newHabitCost} />
    <Button variant="primary" onclick={addCustomHabit}>Add habit</Button>
  </div>
</Sheet>

<style>
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
</style>
