<script lang="ts">
  import Button from "$lib/ui/Button.svelte";
  import Sheet from "$lib/ui/Sheet.svelte";
  import Toggle from "$lib/ui/Toggle.svelte";
  import { commands, type ColumnMappingDto, type ImportPreviewDto, type ImportSummaryDto } from "$lib/ipc";
  import { habitsStore } from "$lib/stores";

  interface Props {
    /** A chosen file path opens the wizard; `null` keeps it closed. Owning
     * the open/closed state via the path (rather than a separate boolean)
     * means there's no state to reset by hand between imports. */
    path: string | null;
    onclose: () => void;
    onImported: () => void;
  }

  let { path, onclose, onImported }: Props = $props();

  let preview = $state<ImportPreviewDto | null>(null);
  let loading = $state(false);
  let loadError = $state<string | null>(null);
  let applying = $state(false);
  let summary = $state<ImportSummaryDto | null>(null);

  let sheetIndex = $state(0);
  let hasHeaderRow = $state(true);
  let habitMode = $state<"fixed" | "column">("fixed");
  let fixedHabitId = $state<number | null>(null);
  let habitColumn = $state<number | null>(null);
  let occurredAtColumn = $state<number | null>(null);
  let quantityColumn = $state<number | null>(null);
  let durationColumn = $state<number | null>(null);
  let noteColumn = $state<number | null>(null);

  function applyRemembered(mapping: ColumnMappingDto | null): void {
    if (!mapping) {
      hasHeaderRow = true;
      fixedHabitId = habitsStore.items[0]?.id ?? null;
      habitMode = fixedHabitId !== null ? "fixed" : "column";
      habitColumn = null;
      occurredAtColumn = 0;
      quantityColumn = null;
      durationColumn = null;
      noteColumn = null;
      return;
    }
    hasHeaderRow = mapping.hasHeaderRow;
    occurredAtColumn = mapping.occurredAtColumn;
    quantityColumn = mapping.quantityColumn;
    durationColumn = mapping.durationColumn;
    noteColumn = mapping.noteColumn;
    if ("fixed" in mapping.habit) {
      habitMode = "fixed";
      fixedHabitId = mapping.habit.fixed;
      habitColumn = null;
    } else {
      habitMode = "column";
      habitColumn = mapping.habit.column;
      fixedHabitId = null;
    }
  }

  $effect(() => {
    if (!path) {
      preview = null;
      summary = null;
      return;
    }
    loading = true;
    loadError = null;
    summary = null;
    commands
      .previewImport(path)
      .then((p) => {
        preview = p;
        sheetIndex = 0;
        applyRemembered(p.rememberedMapping);
      })
      .catch((e) => (loadError = String(e)))
      .finally(() => (loading = false));
  });

  const currentSheet = $derived(preview?.sheets[sheetIndex] ?? null);
  const previewRows = $derived(currentSheet?.rows.slice(0, 8) ?? []);
  const columnCount = $derived(currentSheet?.rows[0]?.length ?? 0);
  const columnIndices = $derived(Array.from({ length: columnCount }, (_, i) => i));

  function columnLabel(i: number): string {
    if (hasHeaderRow && currentSheet?.rows[0]?.[i]) return currentSheet.rows[0][i];
    return `Column ${i + 1}`;
  }

  const mapping = $derived.by((): ColumnMappingDto | null => {
    if (occurredAtColumn === null) return null;
    if (habitMode === "fixed" && fixedHabitId === null) return null;
    if (habitMode === "column" && habitColumn === null) return null;
    return {
      habit: habitMode === "fixed" ? { fixed: fixedHabitId as number } : { column: habitColumn as number },
      occurredAtColumn,
      quantityColumn,
      durationColumn,
      noteColumn,
      hasHeaderRow,
    };
  });

  async function apply(): Promise<void> {
    if (!path || !mapping) return;
    applying = true;
    loadError = null;
    try {
      summary = await commands.applyImport(path, sheetIndex, mapping);
      onImported();
    } catch (e) {
      loadError = String(e);
    } finally {
      applying = false;
    }
  }

  function close(): void {
    summary = null;
    onclose();
  }
</script>

<Sheet open={path !== null} title="Import" onclose={close}>
  {#if loading}
    <p class="muted">Reading file…</p>
  {:else if loadError}
    <p class="error">{loadError}</p>
  {:else if summary}
    <div class="summary">
      <p>{summary.entriesCreated} {summary.entriesCreated === 1 ? "entry" : "entries"} imported.</p>
      {#if summary.rowErrors.length > 0}
        <p class="error">{summary.rowErrors.length} row{summary.rowErrors.length === 1 ? "" : "s"} skipped:</p>
        <ul class="row-errors">
          {#each summary.rowErrors.slice(0, 20) as rowError (rowError.rowIndex)}
            <li>Row {rowError.rowIndex + 1}: {rowError.message}</li>
          {/each}
        </ul>
      {/if}
      <Button variant="primary" onclick={close}>Done</Button>
    </div>
  {:else if preview}
    <div class="wizard">
      {#if preview.sheets.length > 1}
        <div class="sheet-tabs">
          {#each preview.sheets as sheet, i (sheet.name)}
            <button type="button" class="tab" class:active={i === sheetIndex} onclick={() => (sheetIndex = i)}>
              {sheet.name}
            </button>
          {/each}
        </div>
      {/if}

      <Toggle bind:checked={hasHeaderRow} label="First row is a header" />

      <div class="habit-mode">
        <Button variant={habitMode === "fixed" ? "primary" : "secondary"} onclick={() => (habitMode = "fixed")}>
          One habit for the whole file
        </Button>
        <Button variant={habitMode === "column" ? "primary" : "secondary"} onclick={() => (habitMode = "column")}>
          Each row names its own habit
        </Button>
      </div>

      {#if habitMode === "fixed"}
        {#if habitsStore.items.length === 0}
          <p class="muted">No habits exist yet — add one first, or switch to "each row names its own habit".</p>
        {:else}
          <label class="select-field">
            <span>Habit</span>
            <select bind:value={fixedHabitId}>
              {#each habitsStore.items as habit (habit.id)}
                <option value={habit.id}>{habit.name}</option>
              {/each}
            </select>
          </label>
        {/if}
      {:else}
        <label class="select-field">
          <span>Habit name column</span>
          <select bind:value={habitColumn}>
            <option value={null}>—</option>
            {#each columnIndices as i (i)}
              <option value={i}>{columnLabel(i)}</option>
            {/each}
          </select>
        </label>
      {/if}

      <label class="select-field">
        <span>Date / time column</span>
        <select bind:value={occurredAtColumn}>
          <option value={null}>—</option>
          {#each columnIndices as i (i)}
            <option value={i}>{columnLabel(i)}</option>
          {/each}
        </select>
      </label>

      <label class="select-field">
        <span>Quantity column (optional, defaults to 1)</span>
        <select bind:value={quantityColumn}>
          <option value={null}>—</option>
          {#each columnIndices as i (i)}
            <option value={i}>{columnLabel(i)}</option>
          {/each}
        </select>
      </label>

      <label class="select-field">
        <span>Duration column (optional)</span>
        <select bind:value={durationColumn}>
          <option value={null}>—</option>
          {#each columnIndices as i (i)}
            <option value={i}>{columnLabel(i)}</option>
          {/each}
        </select>
      </label>

      <label class="select-field">
        <span>Note column (optional)</span>
        <select bind:value={noteColumn}>
          <option value={null}>—</option>
          {#each columnIndices as i (i)}
            <option value={i}>{columnLabel(i)}</option>
          {/each}
        </select>
      </label>

      {#if previewRows.length > 0}
        <div class="preview-table-wrap">
          <table class="preview-table">
            <tbody>
              {#each previewRows as row, i (i)}
                <tr class:header-row={hasHeaderRow && i === 0}>
                  {#each row as cell, c (c)}
                    <td>{cell}</td>
                  {/each}
                </tr>
              {/each}
            </tbody>
          </table>
        </div>
      {/if}

      {#if loadError}<p class="error">{loadError}</p>{/if}

      <Button variant="primary" disabled={!mapping || applying} onclick={apply}>
        {applying ? "Importing…" : "Import"}
      </Button>
    </div>
  {/if}
</Sheet>

<style>
  .muted {
    color: var(--text-tertiary);
    font-size: 0.875rem;
  }
  .error {
    color: #f87171;
    font-size: 0.875rem;
  }

  .wizard {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    max-width: 32rem;
  }

  .sheet-tabs {
    display: flex;
    gap: 0.375rem;
    flex-wrap: wrap;
  }
  .tab {
    background: var(--surface-raised);
    border: 1px solid var(--line-hairline);
    border-radius: 0.5rem;
    padding: 0.375rem 0.75rem;
    color: var(--text-secondary);
    font-size: 0.8125rem;
    cursor: pointer;
  }
  .tab.active {
    background: var(--accent-translucent);
    border-color: var(--accent-base);
    color: var(--text-primary);
  }

  .habit-mode {
    display: flex;
    gap: 0.5rem;
    flex-wrap: wrap;
  }

  .select-field {
    display: flex;
    flex-direction: column;
    gap: 0.375rem;
    font-size: 0.8125rem;
    color: var(--text-secondary);
  }
  .select-field select {
    background: var(--surface-raised);
    border: 1px solid var(--line-hairline);
    border-radius: 0.5rem;
    padding: 0.5rem 0.625rem;
    color: var(--text-primary);
    font-size: 0.9375rem;
  }

  .preview-table-wrap {
    max-height: 12rem;
    overflow: auto;
    border: 1px solid var(--line-hairline);
    border-radius: 0.5rem;
  }
  .preview-table {
    border-collapse: collapse;
    width: 100%;
    font-size: 0.75rem;
  }
  .preview-table td {
    padding: 0.3rem 0.5rem;
    border-bottom: 1px solid var(--line-hairline);
    white-space: nowrap;
    color: var(--text-secondary);
  }
  .preview-table tr.header-row td {
    color: var(--text-primary);
    font-weight: 600;
  }

  .summary {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }
  .row-errors {
    max-height: 10rem;
    overflow: auto;
    font-size: 0.8125rem;
    color: var(--text-secondary);
    margin: 0;
    padding-left: 1.25rem;
  }
</style>
