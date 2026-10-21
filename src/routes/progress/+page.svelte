<script lang="ts">
  import { onMount } from "svelte";
  import Toolbar from "$lib/ui/Toolbar.svelte";
  import { formatCalendarDate } from "$lib/charts";
  import { parseLocalDate } from "$lib/ipc";
  import { progressStore } from "$lib/stores";

  onMount(() => {
    void progressStore.mount();
    return () => progressStore.unmount();
  });

  // Mirrors the rules in src-tauri/src/application/award_points.rs.
  const RULES = [
    { points: "+5", text: "for every day you log at least one entry. Today counts as soon as you log." },
    { points: "+10", text: "for every completed clean day, with no harmful entries." },
    {
      points: "+25",
      text: "for every habit whose completed week beat the week before: less of a harmful habit, more of a good one.",
    },
    { points: "+50", text: "for every 7th clean day in a row (7, 14, 21…)." },
  ];

  const data = $derived(progressStore.data);

  function formatDay(s: string): string {
    return formatCalendarDate(parseLocalDate(s));
  }
</script>

<Toolbar title="Progress" />

{#if progressStore.error}
  <div class="banner error">{progressStore.error}</div>
{/if}

<div class="page">
  {#if !data}
    <p class="muted">Loading…</p>
  {:else}
    <section class="card hero">
      <div class="hero-row">
        <div>
          <p class="eyebrow">Level {data.levelIndex + 1}</p>
          <h2 class="level">{data.level}</h2>
        </div>
        <p class="total"><span>{data.totalPoints.toLocaleString()}</span> points</p>
      </div>
      <div
        class="track"
        role="progressbar"
        aria-label="Progress to next level"
        aria-valuemin={0}
        aria-valuemax={100}
        aria-valuenow={Math.round(data.levelProgressFraction * 100)}
      >
        <span class="fill" style:width="{Math.round(data.levelProgressFraction * 100)}%"></span>
      </div>
      <p class="muted">
        {#if data.nextLevel && data.pointsToNext !== null}
          {data.pointsToNext.toLocaleString()} points to {data.nextLevel}
        {:else}
          The top level. Points keep counting.
        {/if}
      </p>
    </section>

    <section class="card">
      <h2>How you earn points</h2>
      <ul class="rules">
        {#each RULES as rule (rule.text)}
          <li><span class="pts">{rule.points}</span> {rule.text}</li>
        {/each}
      </ul>
      <p class="muted">Milestones add one-off bonuses. Points never go down, even if you edit your history.</p>
    </section>

    <section class="card">
      <h2>Milestones</h2>
      <ul class="milestones">
        {#each data.milestones as m (m.key)}
          <li class="milestone" class:achieved={m.achievedOn !== null}>
            <div class="milestone-head">
              <span class="milestone-title">{m.title}</span>
              {#if m.points > 0}<span class="pts">+{m.points}</span>{/if}
            </div>
            <p class="milestone-desc">{m.description}</p>
            <p class="milestone-date">
              {m.achievedOn ? formatDay(m.achievedOn) : "Locked"}
            </p>
          </li>
        {/each}
      </ul>
    </section>

    <section class="card">
      <h2>Recent awards</h2>
      {#if data.recentAwards.length === 0}
        <p class="muted">Log an entry to earn your first points.</p>
      {:else}
        <ul class="awards">
          {#each data.recentAwards as award (award.id)}
            <li>
              <span class="pts">+{award.points}</span>
              <span class="award-label">{award.label}</span>
              <span class="award-date">{formatDay(award.awardedFor)}</span>
            </li>
          {/each}
        </ul>
      {/if}
    </section>
  {/if}
</div>

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
    gap: 1.5rem;
  }

  .card {
    background: var(--surface-raised);
    border: 1px solid var(--line-hairline);
    border-radius: 1rem;
    padding: 1.25rem;
  }

  .card h2 {
    font-size: 0.9375rem;
    font-weight: 600;
    color: var(--text-secondary);
    margin: 0 0 0.75rem;
  }

  .muted {
    margin: 0;
    font-size: 0.8125rem;
    color: var(--text-tertiary);
  }

  .pts {
    color: var(--accent-base);
    font-weight: 600;
    font-variant-numeric: tabular-nums;
  }

  /* Header */
  .hero {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }
  .hero-row {
    display: flex;
    justify-content: space-between;
    align-items: flex-end;
    gap: 1rem;
  }
  .eyebrow {
    margin: 0;
    font-size: 0.75rem;
    color: var(--text-tertiary);
    text-transform: uppercase;
    letter-spacing: 0.06em;
  }
  .card h2.level {
    margin: 0.125rem 0 0;
    font-size: 1.75rem;
    letter-spacing: -0.02em;
    color: var(--text-primary);
  }
  .total {
    margin: 0;
    color: var(--text-secondary);
    font-size: 0.875rem;
  }
  .total span {
    font-size: 1.25rem;
    font-weight: 600;
    color: var(--text-primary);
    font-variant-numeric: tabular-nums;
  }
  .track {
    height: 6px;
    border-radius: 999px;
    background: var(--chart-empty);
    overflow: hidden;
  }
  .fill {
    display: block;
    height: 100%;
    background: var(--accent-base);
    transition: width 300ms ease-out;
  }

  /* Rules */
  .rules {
    list-style: none;
    margin: 0 0 0.75rem;
    padding: 0;
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    font-size: 0.875rem;
    color: var(--text-secondary);
  }

  /* Milestones */
  .milestones {
    list-style: none;
    margin: 0;
    padding: 0;
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(12rem, 1fr));
    gap: 0.75rem;
  }
  .milestone {
    padding: 0.875rem;
    border-radius: 0.75rem;
    border: 1px solid var(--line-hairline);
    opacity: 0.45;
  }
  .milestone.achieved {
    opacity: 1;
    background: var(--accent-translucent);
    border-color: var(--accent-base);
  }
  .milestone-head {
    display: flex;
    justify-content: space-between;
    gap: 0.5rem;
    font-size: 0.875rem;
  }
  .milestone-title {
    color: var(--text-primary);
    font-weight: 600;
  }
  .milestone-desc {
    margin: 0.25rem 0 0.5rem;
    font-size: 0.8125rem;
    color: var(--text-secondary);
  }
  .milestone-date {
    margin: 0;
    font-size: 0.75rem;
    color: var(--text-tertiary);
  }

  /* Recent awards */
  .awards {
    list-style: none;
    margin: 0;
    padding: 0;
  }
  .awards li {
    display: grid;
    grid-template-columns: 3rem 1fr auto;
    gap: 0.75rem;
    padding: 0.5rem 0;
    font-size: 0.875rem;
    border-top: 1px solid var(--line-hairline);
  }
  .awards li:first-child {
    border-top: none;
  }
  .award-label {
    color: var(--text-primary);
  }
  .award-date {
    color: var(--text-tertiary);
    font-variant-numeric: tabular-nums;
  }
</style>
