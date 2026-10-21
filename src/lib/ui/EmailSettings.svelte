<script lang="ts">
  // Self-contained email subsection for Settings → Notifications. Loads and
  // saves through its own commands; the secret is write-only from here — the
  // backend only ever tells us whether one is in the keychain.
  import { onMount } from "svelte";

  import { commands, type EmailProviderDto, type SmtpSecurityDto } from "$lib/ipc";
  import {
    dtoFromForm,
    formFromDto,
    PROVIDER_OPTIONS,
    providerHint,
    SECURITY_OPTIONS,
    secretLabel,
    type EmailForm,
  } from "$lib/email-settings";

  import Button from "./Button.svelte";
  import Field from "./Field.svelte";

  let loaded = $state(false);
  let form = $state<EmailForm>({
    provider: "none",
    fromAddress: "",
    toAddress: "",
    smtpHost: "",
    smtpPort: "",
    smtpUsername: "",
    smtpSecurity: "starttls",
  });
  /** The form as last saved, to tell whether the test would use stale settings. */
  let saved = $state<string>("");
  let savedProvider = $state<EmailProviderDto>("none");
  let hasSecret = $state(false);
  let replacing = $state(false);
  let newSecret = $state("");

  let busy = $state(false);
  let error = $state<string | null>(null);
  let status = $state<string | null>(null);
  let testResult = $state<{ ok: boolean; message: string } | null>(null);

  // hasSecret describes the saved provider's secret; after switching provider
  // the field must be filled in for the new one.
  const showSavedSecret = $derived(hasSecret && form.provider === savedProvider && !replacing);
  const dirty = $derived(JSON.stringify(form) !== saved || newSecret.trim() !== "");
  const hint = $derived(providerHint(form));

  async function load() {
    const dto = await commands.getEmailSettings();
    form = formFromDto(dto);
    saved = JSON.stringify(form);
    savedProvider = dto.provider;
    hasSecret = dto.hasSecret;
    replacing = false;
    newSecret = "";
    loaded = true;
  }

  onMount(() => {
    load().catch((e) => (error = String(e)));
  });

  async function run(action: () => Promise<void>, done: string | null) {
    busy = true;
    error = null;
    status = null;
    testResult = null;
    try {
      await action();
      status = done;
    } catch (e) {
      error = String(e);
    } finally {
      busy = false;
    }
  }

  function save() {
    const result = dtoFromForm(form);
    if ("error" in result) {
      error = result.error;
      return;
    }
    const secret = newSecret.trim();
    void run(async () => {
      await commands.saveEmailSettings(result.dto, secret === "" ? undefined : secret);
      await load();
    }, "Email settings saved.");
  }

  function removeSecret() {
    void run(async () => {
      await commands.clearEmailSecret();
      await load();
    }, `${secretLabel(savedProvider)} removed from your keychain.`);
  }

  async function sendTest() {
    busy = true;
    error = null;
    status = null;
    testResult = null;
    try {
      await commands.sendTestEmail();
      testResult = { ok: true, message: `Test email sent to ${form.toAddress.trim()}.` };
    } catch (e) {
      testResult = { ok: false, message: String(e) };
    } finally {
      busy = false;
    }
  }
</script>

<div class="email">
  <h3>Email</h3>
  {#if !loaded && !error}
    <p class="note">Loading…</p>
  {:else}
    <p class="note">
      Emails include your habit data (names, amounts, streaks and points) and pass through the provider you
      choose.
    </p>

    <div class="form">
      <Field
        type="select"
        label="Provider"
        value={form.provider}
        options={PROVIDER_OPTIONS}
        oninput={(v) => (form.provider = v as EmailProviderDto)}
      />

      {#if form.provider !== "none"}
        <Field type="email" label="From address" bind:value={form.fromAddress} placeholder="you@example.com" />
        <Field type="email" label="To address" bind:value={form.toAddress} placeholder="you@example.com" />

        {#if form.provider === "smtp"}
          <Field label="SMTP host" bind:value={form.smtpHost} placeholder="smtp.gmail.com" />
          <Field
            type="select"
            label="Security"
            value={form.smtpSecurity}
            options={SECURITY_OPTIONS}
            oninput={(v) => (form.smtpSecurity = v as SmtpSecurityDto)}
          />
          <Field
            label="Port"
            bind:value={form.smtpPort}
            placeholder={form.smtpSecurity === "tls" ? "465" : "587"}
            hint="Leave empty for the usual port."
          />
          <Field label="Username" bind:value={form.smtpUsername} placeholder="you@example.com" />
        {/if}

        {#if showSavedSecret}
          <div class="secret">
            <span class="secret-label">{secretLabel(form.provider)}</span>
            <div class="secret-row">
              <span class="saved">Saved in your system keychain</span>
              <Button variant="ghost" disabled={busy} onclick={() => (replacing = true)}>Replace</Button>
              <Button variant="ghost" disabled={busy} onclick={removeSecret}>Remove</Button>
            </div>
          </div>
        {:else}
          <Field
            type="password"
            label={secretLabel(form.provider)}
            bind:value={newSecret}
            hint="Stored only in your system keychain, never in Ripple's database."
          />
          {#if replacing}
            <div>
              <Button variant="ghost" onclick={() => ((replacing = false), (newSecret = ""))}>Keep the saved one</Button>
            </div>
          {/if}
        {/if}

        {#if hint}
          <p class="note">{hint}</p>
        {/if}
      {/if}
    </div>

    <div class="actions">
      <Button variant="primary" disabled={busy || !dirty} onclick={save}>Save</Button>
      {#if savedProvider !== "none"}
        <Button variant="secondary" disabled={busy || dirty} onclick={sendTest}>Send test email</Button>
      {/if}
    </div>
    {#if dirty && savedProvider !== "none"}
      <p class="note">Save your changes to send a test email.</p>
    {/if}

    {#if error}
      <p class="message error" role="alert">{error}</p>
    {:else if status}
      <p class="message ok" role="status">{status}</p>
    {/if}
    {#if testResult}
      <p class="message" class:ok={testResult.ok} class:error={!testResult.ok} role="status">
        {testResult.message}
      </p>
    {/if}
  {/if}
</div>

<style>
  .email h3 {
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

  .note {
    margin: 0 0 0.75rem;
    font-size: 0.75rem;
    color: var(--text-tertiary);
    max-width: 30rem;
  }
  .form .note {
    margin: 0;
  }

  .secret {
    display: flex;
    flex-direction: column;
    gap: 0.375rem;
  }
  .secret-label {
    font-size: 0.8125rem;
    color: var(--text-secondary);
  }
  .secret-row {
    display: flex;
    align-items: center;
    gap: 0.25rem;
    flex-wrap: wrap;
  }
  .saved {
    font-size: 0.875rem;
    color: var(--text-primary);
    margin-right: 0.5rem;
  }

  .actions {
    display: flex;
    gap: 0.625rem;
    margin-top: 1.25rem;
  }

  .message {
    margin: 0.75rem 0 0;
    font-size: 0.8125rem;
  }
  .message.ok {
    color: var(--text-secondary);
  }
  .message.error {
    color: #f87171;
  }
</style>
