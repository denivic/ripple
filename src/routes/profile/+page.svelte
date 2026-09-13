<script lang="ts">
  import { onMount } from "svelte";
  import Toolbar from "$lib/ui/Toolbar.svelte";
  import Button from "$lib/ui/Button.svelte";
  import Field from "$lib/ui/Field.svelte";
  import { commands, formatLocalDate, parseLocalDate, profileFromDto, profileToDto, type Profile } from "$lib/ipc";
  import { settingsStore } from "$lib/stores";

  const KG_PER_LB = 0.45359237;

  let profile = $state<Profile>({
    birthDate: null,
    sex: null,
    lifeExpectancyYears: null,
    typicalSleepHours: null,
    netHourlyIncome: null,
    weightKg: null,
  });
  let loading = $state(true);
  let saved = $state(false);
  let error = $state<string | null>(null);

  onMount(() => {
    void settingsStore.mount();
    void load();
    return () => settingsStore.unmount();
  });

  async function load(): Promise<void> {
    loading = true;
    try {
      profile = profileFromDto(await commands.getProfile());
    } catch (e) {
      error = String(e);
    } finally {
      loading = false;
    }
  }

  const weightUnit = $derived(settingsStore.data.weightUnit);
  const weightDisplay = $derived.by(() => {
    if (profile.weightKg === null) return "";
    const value = weightUnit === "lb" ? profile.weightKg / KG_PER_LB : profile.weightKg;
    return value.toFixed(1);
  });

  function setWeight(v: string): void {
    const n = Number(v);
    profile.weightKg = v === "" || Number.isNaN(n) ? null : weightUnit === "lb" ? n * KG_PER_LB : n;
  }

  async function save(): Promise<void> {
    saved = false;
    error = null;
    try {
      await commands.saveProfile(profileToDto(profile));
      saved = true;
    } catch (e) {
      error = String(e);
    }
  }
</script>

<Toolbar title="Profile">
  <Button variant="primary" onclick={save}>Save</Button>
</Toolbar>

{#if saved}
  <div class="banner status">Saved.</div>
{/if}
{#if error}
  <div class="banner error">{error}</div>
{/if}

<div class="page">
  {#if loading}
    <p class="muted">Loading…</p>
  {:else}
    <p class="intro">
      Every field here feeds a specific number elsewhere in the app — see each field's note. None are required; a
      metric that needs a missing field simply doesn't render.
    </p>
    <div class="form">
      <Field
        label="Birth date"
        type="date"
        value={profile.birthDate ? formatLocalDate(profile.birthDate) : ""}
        oninput={(v) => (profile.birthDate = v ? parseLocalDate(v) : null)}
        hint="Affects: remaining-life projection on Today."
      />
      <Field
        label="Sex"
        type="select"
        value={profile.sex ?? ""}
        options={[
          { value: "", label: "Not set" },
          { value: "male", label: "Male" },
          { value: "female", label: "Female" },
        ]}
        oninput={(v) => (profile.sex = v === "" ? null : (v as "male" | "female"))}
        hint="Affects: Widmark BAC estimate for alcohol entries."
      />
      <Field
        label="Life expectancy (years)"
        type="number"
        value={profile.lifeExpectancyYears ?? ""}
        oninput={(v) => (profile.lifeExpectancyYears = v === "" ? null : Number(v))}
        hint="Affects: remaining-life projection on Today."
      />
      <Field
        label="Typical sleep (hours/day)"
        type="number"
        value={profile.typicalSleepHours ?? ""}
        oninput={(v) => (profile.typicalSleepHours = v === "" ? null : Number(v))}
        hint="Affects: waking-life share — the denominator for cost-of-time metrics."
      />
      <Field
        label="Net hourly income"
        type="number"
        value={profile.netHourlyIncome ?? ""}
        oninput={(v) => (profile.netHourlyIncome = v === "" ? null : Number(v))}
        hint="Affects: opportunity cost shown alongside direct spend."
      />
      <Field
        label="Weight ({weightUnit})"
        type="number"
        value={weightDisplay}
        oninput={setWeight}
        hint="Affects: Widmark BAC estimate for alcohol entries. Change the unit in Settings."
      />
    </div>
  {/if}
</div>

<style>
  .page {
    padding: 1.5rem;
    display: flex;
    flex-direction: column;
    gap: 1.25rem;
    max-width: 32rem;
  }

  .intro {
    color: var(--text-tertiary);
    font-size: 0.8125rem;
  }

  .form {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .muted {
    padding: 2rem;
    text-align: center;
    color: var(--text-tertiary);
  }

  .banner {
    margin: 0 1.5rem;
    padding: 0.5rem 0.875rem;
    border-radius: 0.5rem;
    font-size: 0.8125rem;
  }
  .banner.status {
    background: var(--accent-translucent);
    color: var(--text-primary);
  }
  .banner.error {
    background: rgba(248, 113, 113, 0.15);
    color: #f87171;
  }
</style>
