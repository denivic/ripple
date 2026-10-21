<script lang="ts">
  import { onMount } from "svelte";
  import { confirm, open, save } from "@tauri-apps/plugin-dialog";
  import Toolbar from "$lib/ui/Toolbar.svelte";
  import Button from "$lib/ui/Button.svelte";
  import Field from "$lib/ui/Field.svelte";
  import { commands, formatLocalDate, type SettingsDto } from "$lib/ipc";
  import { settingsStore } from "$lib/stores";
  import { builtInThemes, contrastRatio, meetsAA, themeStore, validateTheme, type Theme } from "$lib/theme";

  let ready = $state(false);
  let draft = $state<Theme>(themeStore.current);
  let dataDir = $state("");
  let status = $state<string | null>(null);
  let loadError = $state<string | null>(null);
  let saveTimer: ReturnType<typeof setTimeout> | undefined;

  onMount(() => {
    (async () => {
      try {
        await settingsStore.mount();
        // `themeStore.current` is a $state proxy, which structuredClone
        // rejects (DataCloneError) — snapshot it instead.
        draft = $state.snapshot(themeStore.current);
        dataDir = await commands.getDataDir();
      } catch (e) {
        loadError = `Couldn't load settings: ${String(e)}`;
      } finally {
        // Render whatever did load rather than sitting on "Loading…".
        ready = true;
      }
    })();
    return () => {
      settingsStore.unmount();
      clearTimeout(saveTimer);
    };
  });

  // Every ISO 4217 code the runtime can format, labelled "EUR — Euro".
  const currencyNames = new Intl.DisplayNames(undefined, { type: "currency" });
  const currencyOptions = Intl.supportedValuesOf("currency").map((code) => ({
    value: code,
    label: `${code} — ${currencyNames.of(code) ?? code}`,
  }));

  function announce(message: string): void {
    status = message;
  }

  // --- General ------------------------------------------------------------

  async function saveGeneral(patch: Partial<SettingsDto>): Promise<void> {
    try {
      await settingsStore.save(patch);
    } catch (e) {
      announce(`Couldn't save: ${String(e)}`);
    }
  }

  // --- Appearance / theme editor -------------------------------------------

  function applyDraftLive(): void {
    themeStore.set(draft);
    // A color picker fires `input` continuously while dragging; only the
    // settled value needs to reach SQLite.
    clearTimeout(saveTimer);
    saveTimer = setTimeout(() => void settingsStore.applyCustom(draft), 300);
  }

  function setSurface(field: keyof Theme["surface"], value: string): void {
    draft = { ...draft, surface: { ...draft.surface, [field]: value } };
    applyDraftLive();
  }
  function setText(field: keyof Theme["text"], value: string): void {
    draft = { ...draft, text: { ...draft.text, [field]: value } };
    applyDraftLive();
  }
  function setAccent(field: keyof Theme["accent"], value: string): void {
    draft = { ...draft, accent: { ...draft.accent, [field]: value } };
    applyDraftLive();
  }
  function setHairline(value: string): void {
    draft = { ...draft, line: { hairline: value } };
    applyDraftLive();
  }
  function setSeries(i: number, value: string): void {
    const series = draft.data.series.slice();
    series[i] = value;
    draft = { ...draft, data: { series } };
    applyDraftLive();
  }

  async function selectBuiltIn(theme: Theme): Promise<void> {
    clearTimeout(saveTimer);
    draft = $state.snapshot(theme);
    await settingsStore.applyBuiltIn(theme);
  }

  const contrastChecks = $derived(
    [
      { label: "Primary text on base surface", a: draft.text.primary, b: draft.surface.base },
      { label: "Secondary text on base surface", a: draft.text.secondary, b: draft.surface.base },
      { label: "Primary text on raised surface", a: draft.text.primary, b: draft.surface.raised },
      { label: "Accent on base surface", a: draft.accent.base, b: draft.surface.base },
    ].map((c) => ({ ...c, ratio: contrastRatio(c.a, c.b), ok: meetsAA(c.a, c.b) })),
  );

  async function exportTheme(): Promise<void> {
    const target = await save({
      filters: [{ name: "Theme", extensions: ["json"] }],
      defaultPath: `${draft.name.toLowerCase().replace(/\s+/g, "-")}.json`,
    });
    if (!target) return;
    try {
      await commands.exportTheme(target, JSON.stringify(draft, null, 2));
      announce("Theme exported.");
    } catch (e) {
      announce(`Export failed: ${String(e)}`);
    }
  }

  async function importTheme(): Promise<void> {
    const selection = await open({ multiple: false, filters: [{ name: "Theme", extensions: ["json"] }] });
    if (typeof selection !== "string") return;
    try {
      const text = await commands.importTheme(selection);
      const result = validateTheme(JSON.parse(text));
      if (!result.valid) {
        announce(`Invalid theme file: ${result.errors[0]?.message ?? "unknown error"}`);
        return;
      }
      draft = result.theme;
      await settingsStore.applyCustom(draft);
      announce("Theme imported.");
    } catch (e) {
      announce(`Import failed: ${String(e)}`);
    }
  }

  // --- Data: backups --------------------------------------------------------

  async function backupNow(): Promise<void> {
    const target = await save({
      filters: [{ name: "Ripple Backup", extensions: ["sqlite"] }],
      defaultPath: `ripple-backup-${formatLocalDate(new Date())}.sqlite`,
    });
    if (!target) return;
    try {
      await commands.backupDatabase(target);
      announce("Backup saved.");
    } catch (e) {
      announce(`Backup failed: ${String(e)}`);
    }
  }

  async function restoreBackup(): Promise<void> {
    const selection = await open({ multiple: false, filters: [{ name: "Ripple Backup", extensions: ["sqlite"] }] });
    if (typeof selection !== "string") return;
    const ok = await confirm(
      "Restoring replaces every habit, entry, profile field and setting with the backup's contents. This cannot be undone.",
      { title: "Restore backup", kind: "warning" },
    );
    if (!ok) return;
    try {
      await commands.restoreDatabase(selection);
      announce("Backup restored.");
    } catch (e) {
      announce(`Restore failed: ${String(e)}`);
    }
  }
</script>

<Toolbar title="Settings" />

{#if status}
  <div class="banner status">{status}</div>
{/if}
{#if loadError}
  <div class="banner error">{loadError}</div>
{/if}
{#if settingsStore.error}
  <div class="banner error">{settingsStore.error}</div>
{/if}

<div class="page">
  {#if !ready}
    <p class="muted">Loading…</p>
  {:else}
    <section class="card">
      <h2>General</h2>
      <div class="form">
        <Field
          type="select"
          label="Front page"
          value={settingsStore.data.startupRoute}
          options={[
            { value: "today", label: "Today" },
            { value: "timeline", label: "Timeline" },
            { value: "ledger", label: "Ledger" },
            { value: "profile", label: "Profile" },
            { value: "settings", label: "Settings" },
          ]}
          oninput={(v) => saveGeneral({ startupRoute: v as SettingsDto["startupRoute"] })}
          hint="Which section opens when Ripple starts."
        />
        <Field
          type="select"
          label="Week starts on"
          value={settingsStore.data.weekStart}
          options={[
            { value: "monday", label: "Monday" },
            { value: "sunday", label: "Sunday" },
          ]}
          oninput={(v) => saveGeneral({ weekStart: v as SettingsDto["weekStart"] })}
          hint="Affects the Timeline calendar heatmap's layout."
        />
        <Field
          type="select"
          label="Weight unit"
          value={settingsStore.data.weightUnit}
          options={[
            { value: "kg", label: "Kilograms" },
            { value: "lb", label: "Pounds" },
          ]}
          oninput={(v) => saveGeneral({ weightUnit: v as SettingsDto["weightUnit"] })}
          hint="Used for the Profile page's weight field."
        />
        <Field
          type="select"
          label="Currency"
          value={settingsStore.data.currency}
          options={currencyOptions}
          oninput={(v) => saveGeneral({ currency: v })}
          hint="How money amounts are displayed. Existing amounts are not converted."
        />
      </div>
    </section>

    <section class="card">
      <h2>Appearance</h2>

      <div class="theme-picks">
        {#each builtInThemes as theme (theme.name)}
          <button
            type="button"
            class="theme-swatch"
            class:active={draft.name === theme.name}
            style:background={theme.surface.base}
            onclick={() => selectBuiltIn(theme)}
          >
            <span class="dot" style:background={theme.accent.base}></span>
            {theme.name}
          </button>
        {/each}
      </div>

      <div class="wells">
        <label class="well"><span>Sidebar top</span><input type="color" value={draft.surface.sidebarFrom} oninput={(e) => setSurface("sidebarFrom", e.currentTarget.value)} /></label>
        <label class="well"><span>Sidebar bottom</span><input type="color" value={draft.surface.sidebarTo} oninput={(e) => setSurface("sidebarTo", e.currentTarget.value)} /></label>
        <label class="well"><span>Base surface</span><input type="color" value={draft.surface.base} oninput={(e) => setSurface("base", e.currentTarget.value)} /></label>
        <label class="well"><span>Raised surface</span><input type="color" value={draft.surface.raised} oninput={(e) => setSurface("raised", e.currentTarget.value)} /></label>
        <label class="well"><span>Primary text</span><input type="color" value={draft.text.primary} oninput={(e) => setText("primary", e.currentTarget.value)} /></label>
        <label class="well"><span>Secondary text</span><input type="color" value={draft.text.secondary} oninput={(e) => setText("secondary", e.currentTarget.value)} /></label>
        <label class="well"><span>Tertiary text</span><input type="color" value={draft.text.tertiary} oninput={(e) => setText("tertiary", e.currentTarget.value)} /></label>
        <label class="well"><span>Accent</span><input type="color" value={draft.accent.base} oninput={(e) => setAccent("base", e.currentTarget.value)} /></label>
        <label class="well"><span>Accent (muted)</span><input type="color" value={draft.accent.muted} oninput={(e) => setAccent("muted", e.currentTarget.value)} /></label>
        <label class="well"><span>Hairline</span><input type="color" value={draft.line.hairline} oninput={(e) => setHairline(e.currentTarget.value)} /></label>
      </div>

      <h3>Data series</h3>
      <div class="wells">
        {#each draft.data.series as color, i (i)}
          <label class="well"><span>Series {i + 1}</span><input type="color" value={color} oninput={(e) => setSeries(i, e.currentTarget.value)} /></label>
        {/each}
      </div>

      {#if contrastChecks.some((c) => !c.ok)}
        <div class="contrast-warnings">
          {#each contrastChecks.filter((c) => !c.ok) as c (c.label)}
            <p class="warning">{c.label}: contrast {c.ratio.toFixed(2)}:1 — WCAG AA needs at least 4.5:1.</p>
          {/each}
        </div>
      {/if}

      <div class="actions">
        <Button variant="secondary" onclick={importTheme}>Import theme…</Button>
        <Button variant="secondary" onclick={exportTheme}>Export theme…</Button>
      </div>
    </section>

    <section class="card">
      <h2>Data</h2>
      <p class="data-location">
        Database location: <span class="path">{dataDir}</span>
      </p>
      <div class="actions">
        <Button variant="secondary" onclick={backupNow}>Back up now…</Button>
        <Button variant="secondary" onclick={restoreBackup}>Restore from backup…</Button>
      </div>
    </section>
  {/if}
</div>

<style>
  .page {
    padding: 1.5rem;
    display: flex;
    flex-direction: column;
    gap: 1.25rem;
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

  .card {
    background: var(--surface-raised);
    border: 1px solid var(--line-hairline);
    border-radius: 1rem;
    padding: 1.25rem;
    max-width: 40rem;
  }

  .card h2 {
    font-size: 0.9375rem;
    font-weight: 600;
    color: var(--text-secondary);
    margin: 0 0 1rem;
  }
  .card h3 {
    font-size: 0.8125rem;
    font-weight: 600;
    color: var(--text-tertiary);
    margin: 1rem 0 0.625rem;
  }

  .form {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    max-width: 22rem;
  }

  .theme-picks {
    display: flex;
    gap: 0.625rem;
    margin-bottom: 1.25rem;
  }
  .theme-swatch {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem 0.875rem;
    border-radius: 0.625rem;
    border: 1px solid var(--line-hairline);
    color: var(--text-primary);
    font-size: 0.8125rem;
    cursor: pointer;
  }
  .theme-swatch.active {
    border-color: var(--accent-base);
  }
  .dot {
    width: 0.625rem;
    height: 0.625rem;
    border-radius: 50%;
  }

  .wells {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(9rem, 1fr));
    gap: 0.75rem 1rem;
  }
  .well {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 0.5rem;
    font-size: 0.8125rem;
    color: var(--text-secondary);
  }
  .well input[type="color"] {
    width: 2rem;
    height: 1.5rem;
    padding: 0;
    border: 1px solid var(--line-hairline);
    border-radius: 0.375rem;
    background: none;
    cursor: pointer;
  }

  .contrast-warnings {
    margin-top: 1rem;
    padding: 0.75rem 0.875rem;
    border-radius: 0.625rem;
    background: rgba(248, 113, 113, 0.1);
  }
  .warning {
    margin: 0;
    font-size: 0.75rem;
    color: #f87171;
  }
  .warning + .warning {
    margin-top: 0.25rem;
  }

  .actions {
    display: flex;
    gap: 0.625rem;
    margin-top: 1.25rem;
  }

  .data-location {
    font-size: 0.8125rem;
    color: var(--text-secondary);
    margin: 0 0 0.25rem;
  }
  .path {
    color: var(--text-primary);
    font-family: ui-monospace, monospace;
    user-select: all;
  }
</style>
