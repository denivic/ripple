<script lang="ts">
  import { onMount } from "svelte";
  import { confirm, open, save } from "@tauri-apps/plugin-dialog";
  import Toolbar from "$lib/ui/Toolbar.svelte";
  import Button from "$lib/ui/Button.svelte";
  import Field from "$lib/ui/Field.svelte";
  import Sheet from "$lib/ui/Sheet.svelte";
  import Toggle from "$lib/ui/Toggle.svelte";
  import TimeField from "$lib/ui/TimeField.svelte";
  import EmailSettings from "$lib/ui/EmailSettings.svelte";
  import {
    commands,
    formatLocalDate,
    type NotificationCategoryDto,
    type NotificationCategoryId,
    type NotificationSettingsDto,
    type SettingsDto,
  } from "$lib/ipc";
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
        notifications = await commands.getNotificationSettings();
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

  // --- Currency change dialog ----------------------------------------------
  // Picking a currency doesn't save on its own: amounts are stored as plain
  // numbers, so the user chooses between converting them at today's rate or
  // just relabelling them.

  let pendingCurrency = $state<string | null>(null);
  let rate = $state<number | null>(null);
  let rateError = $state<string | null>(null);
  let converting = $state(false);
  // Bumped on Cancel so the <select> re-renders back to the saved value —
  // the DOM already shows the abandoned pick, and the bound prop hasn't changed.
  let currencySelectKey = $state(0);
  let rateRequest = 0;

  async function pickCurrency(code: string): Promise<void> {
    if (code === settingsStore.data.currency) return;
    pendingCurrency = code;
    rate = null;
    rateError = null;
    const request = ++rateRequest;
    try {
      const fetched = await commands.fetchExchangeRate(settingsStore.data.currency, code);
      if (request === rateRequest) rate = fetched;
    } catch (e) {
      if (request === rateRequest) rateError = String(e);
    }
  }

  function closeCurrencyDialog(): void {
    if (converting) return;
    pendingCurrency = null;
    rateRequest++;
    currencySelectKey++;
  }

  async function convertAmounts(): Promise<void> {
    if (!pendingCurrency) return;
    const from = settingsStore.data.currency;
    const to = pendingCurrency;
    converting = true;
    try {
      const result = await commands.convertCurrency(from, to);
      await settingsStore.load();
      announce(
        `Converted ${result.convertedHabits} habit${result.convertedHabits === 1 ? "" : "s"} and your hourly income at 1 ${from} = ${formatRate(result.rate)} ${to}.`,
      );
      converting = false;
      closeCurrencyDialog();
    } catch (e) {
      rateError = String(e);
      converting = false;
    }
  }

  async function relabelOnly(): Promise<void> {
    if (!pendingCurrency) return;
    const to = pendingCurrency;
    closeCurrencyDialog();
    await saveGeneral({ currency: to });
  }

  function formatRate(value: number): string {
    return value.toLocaleString(undefined, { maximumFractionDigits: 4 });
  }

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

  // --- Data: reset ------------------------------------------------------------

  let resetOpen = $state(false);
  let resetting = $state(false);
  let resetError = $state<string | null>(null);

  function closeResetDialog(): void {
    if (resetting) return;
    resetOpen = false;
    resetError = null;
  }

  async function resetDatabase(): Promise<void> {
    resetting = true;
    try {
      await commands.resetDatabase();
      await settingsStore.load();
      resetting = false;
      closeResetDialog();
      announce("Database reset. All data has been erased.");
    } catch (e) {
      resetError = `Reset failed: ${String(e)}`;
      resetting = false;
    }
  }
  // --- Notifications ---------------------------------------------------------
  // Every control saves straight away; the backend returns the saved state
  // (with fresh permission / email status), which replaces the local copy.

  let notifications = $state<NotificationSettingsDto | null>(null);
  let notificationError = $state<string | null>(null);
  let testingNotification = $state(false);

  const categoryInfo: Record<NotificationCategoryId, { title: string; hint: string }> = {
    reminders: { title: "Daily reminder", hint: "A nudge to log, skipped once anything is logged that day." },
    progress: { title: "Progress", hint: "Level-ups, milestones and points earned." },
    weekly_summary: { title: "Weekly summary", hint: "This week against last, points, streak and most improved habit." },
    streak_pace: { title: "Streak and pace alerts", hint: "Protect a clean streak; warn when a habit runs above its usual pace." },
  };

  const weekdayOptions = ["monday", "tuesday", "wednesday", "thursday", "friday", "saturday", "sunday"].map((d) => ({
    value: d,
    label: d[0].toUpperCase() + d.slice(1),
  }));

  async function saveNotifications(patch: Partial<NotificationSettingsDto>): Promise<void> {
    if (!notifications) return;
    const next = { ...$state.snapshot(notifications), ...patch };
    notifications = next;
    try {
      notifications = await commands.saveNotificationSettings(next);
      notificationError = null;
    } catch (e) {
      notificationError = `Couldn't save notification settings: ${String(e)}`;
      // Reload so the controls show what is actually stored.
      notifications = await commands.getNotificationSettings().catch(() => notifications);
    }
  }

  function saveCategory(id: NotificationCategoryId, patch: Partial<NotificationCategoryDto>): void {
    if (!notifications) return;
    const categories = notifications.categories.map((c) => (c.category === id ? { ...c, ...patch } : { ...c }));
    void saveNotifications({ categories });
  }

  function setQuietHours(on: boolean): void {
    void saveNotifications(on ? { quietStart: "22:00", quietEnd: "07:00" } : { quietStart: null, quietEnd: null });
  }

  function saveTime(field: "reminderTime" | "weeklyTime" | "streakAlertTime" | "quietStart" | "quietEnd", v: string): void {
    if (/^\d{2}:\d{2}$/.test(v)) void saveNotifications({ [field]: v });
  }

  async function refreshEmailStatus(): Promise<void> {
    try {
      const fresh = await commands.getNotificationSettings();
      if (notifications) notifications.emailConfigured = fresh.emailConfigured;
    } catch {
      // Status refresh only; the next save reports real errors.
    }
  }

  async function sendTestNotification(): Promise<void> {
    testingNotification = true;
    try {
      await commands.sendTestNotification();
      announce("Test notification sent.");
    } catch (e) {
      notificationError = `Test notification failed: ${String(e)}`;
    } finally {
      testingNotification = false;
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
          label="Time format"
          value={settingsStore.data.timeFormat}
          options={[
            { value: "24h", label: "24-hour (13:00)" },
            { value: "12h", label: "12-hour (1:00 PM)" },
          ]}
          oninput={(v) => saveGeneral({ timeFormat: v as SettingsDto["timeFormat"] })}
        />
        <!-- Stored as the weight unit (kg/lb), which predates the wider setting. -->
        <Field
          type="select"
          label="Units"
          value={settingsStore.data.weightUnit}
          options={[
            { value: "kg", label: "Metric (kg, g, ml)" },
            { value: "lb", label: "Imperial (lb, oz, fl oz)" },
          ]}
          oninput={(v) => saveGeneral({ weightUnit: v as SettingsDto["weightUnit"] })}
          hint="Used for the Profile page's weight field and preset serving sizes."
        />
        {#key currencySelectKey}
          <Field
            type="select"
            label="Currency"
            value={settingsStore.data.currency}
            options={currencyOptions}
            oninput={(v) => void pickCurrency(v)}
            hint="Changing it offers to convert habit costs and your hourly income at today's exchange rate."
          />
        {/key}
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

    {#if notifications}
      <section class="card">
        <h2>Notifications</h2>
        {#if notificationError}
          <p class="warning">{notificationError}</p>
        {/if}
        {#if notifications.permission === "denied"}
          <p class="warning">
            Notifications are blocked for Ripple. Allow them in your system settings
            (macOS: System Settings → Notifications → Ripple; Windows: Settings → System → Notifications).
          </p>
        {/if}

        <div class="notify-categories">
          {#each notifications.categories as cat (cat.category)}
            <div class="notify-row">
              <div class="notify-title">
                <Toggle
                  checked={cat.enabled}
                  label={categoryInfo[cat.category].title}
                  onchange={(on) => saveCategory(cat.category, { enabled: on })}
                />
                <span class="notify-hint">{categoryInfo[cat.category].hint}</span>
              </div>
              <div class="notify-channels">
                <label class="check">
                  <input
                    type="checkbox"
                    checked={cat.local}
                    disabled={!cat.enabled}
                    onchange={(e) => saveCategory(cat.category, { local: e.currentTarget.checked })}
                  />
                  Local
                </label>
                <label class="check" title={notifications.emailConfigured ? undefined : "Set up email below first"}>
                  <input
                    type="checkbox"
                    checked={cat.email && notifications.emailConfigured}
                    disabled={!cat.enabled || !notifications.emailConfigured}
                    onchange={(e) => saveCategory(cat.category, { email: e.currentTarget.checked })}
                  />
                  Email
                </label>
              </div>
            </div>
          {/each}
          {#if !notifications.emailConfigured}
            <p class="notify-hint">Email is off until an email provider is set up below.</p>
          {/if}
        </div>

        <h3>Schedule</h3>
        <div class="form">
          <TimeField
            label="Daily reminder"
            value={notifications.reminderTime}
            oninput={(v) => saveTime("reminderTime", v)}
          />
          <Field
            type="select"
            label="Weekly summary day"
            value={notifications.weeklyDay}
            options={weekdayOptions}
            oninput={(v) => void saveNotifications({ weeklyDay: v as NotificationSettingsDto["weeklyDay"] })}
          />
          <TimeField
            label="Weekly summary time"
            value={notifications.weeklyTime}
            oninput={(v) => saveTime("weeklyTime", v)}
          />
          <TimeField
            label="Streak alert"
            value={notifications.streakAlertTime}
            oninput={(v) => saveTime("streakAlertTime", v)}
            hint="Sent when you're on a clean streak of 2+ days and today is still clean."
          />
          <Field
            type="number"
            label="Pace alert threshold"
            value={notifications.paceThreshold}
            oninput={(v) => {
              const n = Number(v);
              if (Number.isFinite(n) && n > 0) void saveNotifications({ paceThreshold: n });
            }}
            hint="Alert when a harmful habit today exceeds this × its 28-day daily average."
          />
        </div>

        <h3>Quiet hours</h3>
        <div class="form">
          <Toggle
            checked={notifications.quietStart !== null && notifications.quietEnd !== null}
            label="Hold notifications during quiet hours"
            onchange={setQuietHours}
          />
          {#if notifications.quietStart !== null && notifications.quietEnd !== null}
            <TimeField label="From" value={notifications.quietStart} oninput={(v) => saveTime("quietStart", v)} />
            <TimeField
              label="Until"
              value={notifications.quietEnd}
              oninput={(v) => saveTime("quietEnd", v)}
              hint="Anything due in this window is delivered when it ends, not dropped."
            />
          {/if}
        </div>

        <h3>Behaviour</h3>
        <div class="form">
          <Toggle
            checked={notifications.bundleProgress}
            label="Bundle progress into one notification"
            onchange={(on) => void saveNotifications({ bundleProgress: on })}
          />
          <Toggle
            checked={notifications.launchAtLogin}
            label="Launch Ripple at login"
            onchange={(on) => void saveNotifications({ launchAtLogin: on })}
          />
          <Toggle
            checked={notifications.closeToTray}
            label="Keep running in the tray when the window is closed"
            onchange={(on) => void saveNotifications({ closeToTray: on })}
          />
        </div>

        <div class="actions">
          <Button variant="secondary" disabled={testingNotification} onclick={sendTestNotification}>
            {testingNotification ? "Sending…" : "Send test notification"}
          </Button>
        </div>

        <div class="email-settings" onfocusout={refreshEmailStatus}>
          <EmailSettings />
        </div>
      </section>
    {/if}

    <section class="card danger-zone">
      <h2>Danger zone</h2>
      <p class="danger-copy">
        Erase every habit, entry, profile field and setting and start from an empty database.
      </p>
      <div class="actions">
        <Button variant="danger" onclick={() => (resetOpen = true)}>Reset database…</Button>
      </div>
    </section>
  {/if}
</div>

<Sheet open={pendingCurrency !== null} title="Change currency" onclose={closeCurrencyDialog}>
  <div class="currency-dialog">
    <p>
      Switch from {settingsStore.data.currency} to {pendingCurrency}. Converting multiplies every habit's cost per
      unit and your net hourly income by today's rate; changing the symbol only keeps the numbers as they are.
    </p>
    {#if rateError}
      <p class="warning">{rateError}</p>
    {:else if rate === null}
      <p class="rate muted-inline">Fetching today's rate…</p>
    {:else}
      <p class="rate">1 {settingsStore.data.currency} = {formatRate(rate)} {pendingCurrency}</p>
    {/if}
    <div class="dialog-actions">
      <Button variant="primary" disabled={rate === null || rateError !== null || converting} onclick={convertAmounts}>
        {converting ? "Converting…" : "Convert at today's rate"}
      </Button>
      <Button variant="secondary" disabled={converting} onclick={relabelOnly}>Change symbol only</Button>
      <Button variant="ghost" disabled={converting} onclick={closeCurrencyDialog}>Cancel</Button>
    </div>
  </div>
</Sheet>

<Sheet open={resetOpen} title="Reset database?" onclose={closeResetDialog}>
  <div class="reset-dialog">
    <p class="warning">
      This permanently deletes all your habits, logged entries, profile and settings. It cannot be undone.
    </p>
    <p>If you might want this data later, cancel and use “Back up now…” first.</p>
    {#if resetError}
      <p class="warning">{resetError}</p>
    {/if}
    <div class="dialog-actions">
      <Button variant="ghost" disabled={resetting} onclick={closeResetDialog}>Cancel</Button>
      <Button variant="danger" disabled={resetting} onclick={resetDatabase}>
        {resetting ? "Erasing…" : "Erase everything"}
      </Button>
    </div>
  </div>
</Sheet>

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

  .currency-dialog p {
    margin: 0 0 0.75rem;
    font-size: 0.8125rem;
    color: var(--text-secondary);
  }
  .currency-dialog .rate {
    font-size: 0.9375rem;
    font-weight: 600;
    color: var(--text-primary);
    font-variant-numeric: tabular-nums;
  }
  .currency-dialog .warning {
    color: #f87171;
  }
  .currency-dialog .muted-inline {
    font-weight: 400;
    color: var(--text-tertiary);
  }
  .danger-zone {
    border-color: color-mix(in oklch, #d03b3b 45%, var(--line-hairline));
  }
  .danger-zone h2 {
    color: #f87171;
  }
  .danger-copy {
    margin: 0;
    font-size: 0.8125rem;
    color: var(--text-secondary);
  }
  .danger-zone .actions {
    margin-top: 1rem;
  }

  .reset-dialog p {
    margin: 0 0 0.75rem;
    font-size: 0.8125rem;
    color: var(--text-secondary);
  }
  .reset-dialog .warning {
    font-size: 0.875rem;
    font-weight: 500;
    color: #f87171;
  }

  .dialog-actions {
    display: flex;
    flex-wrap: wrap;
    gap: 0.625rem;
    margin-top: 1rem;
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
  .notify-categories {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
    margin-bottom: 1rem;
  }
  .notify-row {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    gap: 1rem;
    flex-wrap: wrap;
  }
  .notify-title {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }
  .notify-hint {
    font-size: 0.75rem;
    color: var(--text-tertiary);
  }
  .notify-channels {
    display: flex;
    gap: 1rem;
  }
  .check {
    display: inline-flex;
    align-items: center;
    gap: 0.375rem;
    font-size: 0.875rem;
    color: var(--text-secondary);
  }
  .email-settings {
    margin-top: 0.5rem;
  }
</style>
