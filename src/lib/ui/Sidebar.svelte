<script lang="ts">
  import { page } from "$app/state";
  import { progressStore } from "$lib/stores";

  const NAV_ITEMS = [
    { href: "/today", label: "Today", icon: "today" },
    { href: "/timeline", label: "Timeline", icon: "timeline" },
    { href: "/ledger", label: "Ledger", icon: "ledger" },
    { href: "/progress", label: "Progress", icon: "progress" },
    { href: "/profile", label: "Profile", icon: "profile" },
    { href: "/settings", label: "Settings", icon: "settings" },
  ] as const;

  function isActive(href: string): boolean {
    return page.url.pathname === href || page.url.pathname.startsWith(href + "/");
  }
</script>

{#snippet icon(name: string)}
  <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round">
    {#if name === "today"}
      <circle cx="12" cy="12" r="4" />
      <line x1="12" y1="2" x2="12" y2="4" />
      <line x1="12" y1="20" x2="12" y2="22" />
      <line x1="2" y1="12" x2="4" y2="12" />
      <line x1="20" y1="12" x2="22" y2="12" />
    {:else if name === "timeline"}
      <line x1="3" y1="21" x2="21" y2="21" />
      <polyline points="3,17 9,11 13,15 21,5" />
    {:else if name === "ledger"}
      <rect x="3" y="4" width="18" height="16" rx="2" />
      <line x1="3" y1="10" x2="21" y2="10" />
      <line x1="9" y1="4" x2="9" y2="20" />
    {:else if name === "profile"}
      <circle cx="12" cy="8" r="4" />
      <path d="M4 20c0-4 4-6 8-6s8 2 8 6" />
    {:else if name === "progress"}
      <path d="M12 3c3 4 6 7.5 6 11a6 6 0 0 1-12 0c0-3.5 3-7 6-11z" />
    {:else if name === "settings"}
      <line x1="4" y1="6" x2="20" y2="6" />
      <circle cx="9" cy="6" r="2" />
      <line x1="4" y1="12" x2="20" y2="12" />
      <circle cx="15" cy="12" r="2" />
      <line x1="4" y1="18" x2="20" y2="18" />
      <circle cx="9" cy="18" r="2" />
    {/if}
  </svg>
{/snippet}

<nav class="sidebar" aria-label="Main">
  <div class="brand">Ripple</div>
  <ul class="nav-list">
    {#each NAV_ITEMS as item (item.href)}
      <li>
        <a
          class="nav-item"
          class:active={isActive(item.href)}
          href={item.href}
          aria-current={isActive(item.href) ? "page" : undefined}
        >
          <span class="nav-icon" aria-hidden="true">{@render icon(item.icon)}</span>
          <span>{item.label}</span>
        </a>
      </li>
    {/each}
  </ul>

  {#if progressStore.data}
    {@const p = progressStore.data}
    <a class="level-badge" href="/progress" aria-label="{p.level}, {p.totalPoints} points. View progress">
      <span class="level-row">
        <span class="level-name">{p.level}</span>
        <span class="level-points">{p.totalPoints.toLocaleString()} pts</span>
      </span>
      <span class="level-track" aria-hidden="true">
        <span class="level-fill" style:width="{Math.round(p.levelProgressFraction * 100)}%"></span>
      </span>
      <!-- Non-blocking, self-clearing notice for newly earned points. -->
      <span class="level-notice" aria-live="polite">{progressStore.notice ?? ""}</span>
    </a>
  {/if}
</nav>

<style>
  .sidebar {
    display: flex;
    flex-direction: column;
    height: 100%;
    padding: 1.25rem 0.75rem;
    /* Heavier material than the content pane — separates structural chrome
       from content, per apple-design's material-weight hierarchy. */
    background: linear-gradient(180deg, var(--surface-sidebar-from), var(--surface-sidebar-to));
    border-right: 1px solid var(--line-hairline);
  }

  .brand {
    font-size: 1.0625rem;
    font-weight: 600;
    letter-spacing: -0.02em;
    color: var(--text-primary);
    padding: 0.5rem 0.625rem 1.25rem;
  }

  .nav-list {
    list-style: none;
    margin: 0;
    padding: 0;
    display: flex;
    flex-direction: column;
    gap: 0.125rem;
  }

  .nav-item {
    display: flex;
    align-items: center;
    gap: 0.625rem;
    padding: 0.5rem 0.625rem;
    border-radius: 0.5rem;
    color: var(--text-secondary);
    text-decoration: none;
    font-size: 0.9375rem;
    transition:
      background-color 120ms ease-out,
      color 120ms ease-out;
  }

  .nav-item:hover {
    background: color-mix(in oklch, white 6%, transparent);
    color: var(--text-primary);
  }

  .nav-item.active {
    background: var(--accent-translucent);
    color: var(--text-primary);
  }

  .nav-item.active .nav-icon {
    color: var(--accent-base);
  }

  .nav-icon {
    display: inline-flex;
    color: currentColor;
  }

  .level-badge {
    display: flex;
    flex-direction: column;
    gap: 0.375rem;
    margin-top: 1rem;
    padding: 0.625rem;
    border-radius: 0.5rem;
    border: 1px solid var(--line-hairline);
    text-decoration: none;
    color: var(--text-secondary);
    transition: background-color 120ms ease-out;
  }

  .level-badge:hover {
    background: color-mix(in oklch, white 6%, transparent);
  }

  .level-badge:focus-visible {
    outline: 2px solid var(--focus-ring);
    outline-offset: 2px;
  }

  .level-row {
    display: flex;
    justify-content: space-between;
    align-items: baseline;
    font-size: 0.8125rem;
  }

  .level-name {
    color: var(--text-primary);
    font-weight: 600;
  }

  .level-points {
    font-variant-numeric: tabular-nums;
  }

  .level-track {
    height: 3px;
    border-radius: 999px;
    background: var(--chart-empty);
    overflow: hidden;
  }

  .level-fill {
    display: block;
    height: 100%;
    background: var(--accent-base);
    transition: width 300ms ease-out;
  }

  .level-notice {
    min-height: 1em;
    font-size: 0.75rem;
    color: var(--accent-base);
    font-variant-numeric: tabular-nums;
  }

  .level-notice:empty {
    display: none;
  }

  .nav-item:focus-visible {
    outline: 2px solid var(--focus-ring);
    outline-offset: 2px;
  }
</style>
