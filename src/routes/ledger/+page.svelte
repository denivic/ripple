<script lang="ts">
  import { onMount } from "svelte";
  import { open, save } from "@tauri-apps/plugin-dialog";
  import Toolbar from "$lib/ui/Toolbar.svelte";
  import Button from "$lib/ui/Button.svelte";
  import Toggle from "$lib/ui/Toggle.svelte";
  import { DataGrid, ImportWizard } from "$lib/grid";
  import { commands } from "$lib/ipc";
  import { habitsStore, ledgerStore } from "$lib/stores";

  onMount(() => {
    void habitsStore.mount();
    void ledgerStore.mount();
    return () => {
      habitsStore.unmount();
      ledgerStore.unmount();
    };
  });

  let showDuration = $state(true);
  let showNote = $state(true);
  let importPath = $state<string | null>(null);
  let exporting = $state(false);

  const habitNames = $derived(habitsStore.items.map((h) => h.name));

  async function pickImportFile(): Promise<void> {
    const selection = await open({
      multiple: false,
      filters: [{ name: "Spreadsheets", extensions: ["xlsx", "csv", "numbers"] }],
    });
    if (typeof selection === "string") importPath = selection;
  }

  async function exportEntries(format: "xlsx" | "csv"): Promise<void> {
    const target = await save({
      filters: [{ name: format === "xlsx" ? "Excel Workbook" : "CSV", extensions: [format] }],
      defaultPath: `ripple-export.${format}`,
    });
    if (!target) return;
    exporting = true;
    try {
      await ledgerStore.flushNow();
      const count = await commands.exportEntries(target, format);
      ledgerStore.announce(`${count} ${count === 1 ? "entry" : "entries"} exported.`);
    } catch (e) {
      ledgerStore.announce(`Export failed: ${String(e)}`);
    } finally {
      exporting = false;
    }
  }

  function handleKeydown(e: KeyboardEvent): void {
    const meta = e.metaKey || e.ctrlKey;
    if (!meta) return;
    const target = e.target as HTMLElement;
    if (target.tagName === "INPUT" || target.tagName === "SELECT" || target.tagName === "TEXTAREA") {
      // Undo/redo inside a cell being edited is the browser's native text-field
      // undo, not the grid's row-level undo — only intercept outside cells.
      return;
    }
    if (e.key === "z" && !e.shiftKey) {
      e.preventDefault();
      ledgerStore.undo();
    } else if (e.key === "z" && e.shiftKey) {
      e.preventDefault();
      ledgerStore.redo();
    }
  }
</script>

<svelte:window onkeydown={handleKeydown} />

<Toolbar title="Ledger">
  <Toggle bind:checked={showDuration} label="Duration" />
  <Toggle bind:checked={showNote} label="Note" />
  <Button variant="secondary" disabled={!ledgerStore.canUndo} onclick={() => ledgerStore.undo()}>Undo</Button>
  <Button variant="secondary" disabled={!ledgerStore.canRedo} onclick={() => ledgerStore.redo()}>Redo</Button>
  <Button variant="secondary" onclick={pickImportFile}>Import…</Button>
  <Button variant="secondary" disabled={exporting} onclick={() => exportEntries("csv")}>Export CSV</Button>
  <Button variant="secondary" disabled={exporting} onclick={() => exportEntries("xlsx")}>Export Excel</Button>
</Toolbar>

{#if ledgerStore.status}
  <div class="banner status">{ledgerStore.status}</div>
{/if}
{#if ledgerStore.error}
  <div class="banner error">{ledgerStore.error}</div>
{/if}

<div class="page">
  {#if ledgerStore.loading}
    <p class="muted">Loading entries…</p>
  {:else}
    <DataGrid
      rows={ledgerStore.rows}
      habits={ledgerStore.habitLookup}
      {habitNames}
      habitNameById={(id) => ledgerStore.habitName(id)}
      {showDuration}
      {showNote}
      onPatch={(patches, label) => ledgerStore.patch(patches, label)}
      onDeleteRows={(rowIds) => ledgerStore.deleteRows(rowIds)}
      onInsertRow={() => ledgerStore.insertRow()}
    />
  {/if}
</div>

<ImportWizard
  path={importPath}
  onclose={() => (importPath = null)}
  onImported={() => ledgerStore.announce("Import complete.")}
/>

<style>
  .page {
    display: flex;
    flex-direction: column;
    flex: 1;
    min-height: 0;
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
