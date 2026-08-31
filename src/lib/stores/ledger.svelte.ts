import {
  commands,
  entryFromDto,
  entryToDto,
  onDataChanged,
  type Entry,
} from "$lib/ipc";
import {
  CommandStack,
  DeleteRowsCommand,
  InsertRowCommand,
  PatchRowsCommand,
  type GridCommand,
  type GridModel,
  type GridRow,
  type HabitLookup,
  type RowPatch,
} from "$lib/grid";
import type { UnlistenFn } from "@tauri-apps/api/event";
import { habitsStore } from "./habits.svelte";

const FLUSH_DEBOUNCE_MS = 800;
const STATUS_DURATION_MS = 4000;

/** Negative, monotonically decreasing — never reused, so a row created and
 * then undone can't collide with a later insert. Real entry ids from SQLite
 * start at 1, so any negative id unambiguously means "not yet persisted." */
let nextTempId = -1;

type EntryFields = Pick<GridRow, "habitId" | "occurredAt" | "quantity" | "durationMinutes" | "note">;

interface SyncedRow {
  /** The real backend row id. Distinct from `GridRow.id`, which is a stable
   * client-side key assigned once and never renumbered — see the module
   * comment on why the two must not be conflated. */
  serverId: number;
  occurredAtMs: number;
  habitId: number;
  quantity: number;
  durationMinutes: number | null;
  note: string | null;
}

function toComparable(row: EntryFields): Omit<SyncedRow, "serverId"> {
  return {
    occurredAtMs: row.occurredAt.getTime(),
    habitId: row.habitId,
    quantity: row.quantity,
    durationMinutes: row.durationMinutes,
    note: row.note,
  };
}

function fieldsEqual(a: Omit<SyncedRow, "serverId">, b: Omit<SyncedRow, "serverId">): boolean {
  return (
    a.occurredAtMs === b.occurredAtMs &&
    a.habitId === b.habitId &&
    a.quantity === b.quantity &&
    a.durationMinutes === b.durationMinutes &&
    a.note === b.note
  );
}

function toEntry(row: GridRow, serverId: number | null): Omit<Entry, "id"> & { id?: number | null } {
  return {
    id: serverId,
    habitId: row.habitId,
    occurredAt: row.occurredAt,
    quantity: row.quantity,
    durationMinutes: row.durationMinutes,
    note: row.note,
  };
}

/**
 * The Ledger's in-memory grid: `rows` is the single source of truth the
 * command stack edits directly (see plan-v1.md's Command pattern note — no
 * IPC round trip per keystroke). A debounced `#flush` reconciles that state
 * against SQLite by diffing against `#synced`, the last-known-persisted
 * shape of each row, rather than tracking dirty/deleted sets incrementally.
 * That diff-based design is what makes undo/redo "just work" against the
 * backend for free: undoing an edit reverts a field, the next flush's diff
 * notices the mismatch and re-saves it; undoing a delete puts the row back
 * in `rows`, and the flush either finds it still in `#synced` (delete never
 * went out — nothing to do) or not (delete already committed — it's
 * recreated as a new row, which is the only sound thing to do once the old
 * server row is actually gone).
 */
class LedgerStore implements GridModel {
  rows = $state<GridRow[]>([]);
  loading = $state(true);
  error = $state<string | null>(null);
  status = $state<string | null>(null);

  #stack = new CommandStack();
  #synced = new Map<number, SyncedRow>();
  #flushTimer: ReturnType<typeof setTimeout> | null = null;
  #statusTimer: ReturnType<typeof setTimeout> | null = null;
  #unlisten: UnlistenFn | null = null;
  #flushing = false;
  #flushAgainAfter = false;

  get canUndo(): boolean {
    return this.#stack.canUndo;
  }

  get canRedo(): boolean {
    return this.#stack.canRedo;
  }

  get habitLookup(): HabitLookup {
    return {
      idByName: (name) => {
        const target = name.trim().toLowerCase();
        return habitsStore.items.find((h) => h.name.toLowerCase() === target)?.id ?? null;
      },
    };
  }

  habitName(habitId: number): string | null {
    return habitsStore.byId(habitId)?.name ?? null;
  }

  async load(): Promise<void> {
    this.loading = true;
    this.error = null;
    try {
      const entries = (await commands.listEntries()).map(entryFromDto);
      this.rows = entries
        .map((e) => ({
          id: e.id,
          habitId: e.habitId,
          occurredAt: e.occurredAt,
          quantity: e.quantity,
          durationMinutes: e.durationMinutes,
          note: e.note,
        }))
        .sort((a, b) => a.occurredAt.getTime() - b.occurredAt.getTime());
      this.#synced.clear();
      for (const row of this.rows) this.#synced.set(row.id, { serverId: row.id, ...toComparable(row) });
      this.#stack = new CommandStack();
    } catch (e) {
      this.error = String(e);
    } finally {
      this.loading = false;
    }
  }

  async mount(): Promise<void> {
    await this.load();
    this.#unlisten ??= await onDataChanged((event) => {
      // Skip an external refresh while a flush owns `rows` or an edit is
      // about to become one — a reload here would race the in-flight write
      // and could clobber it with pre-edit server state.
      if ((event.scope === "entries" || event.scope === "habits") && !this.#flushing && this.#flushTimer === null) {
        void this.load();
      }
    });
  }

  unmount(): void {
    this.#unlisten?.();
    this.#unlisten = null;
    if (this.#flushTimer) clearTimeout(this.#flushTimer);
    if (this.#statusTimer) clearTimeout(this.#statusTimer);
  }

  #setStatus(message: string): void {
    this.status = message;
    if (this.#statusTimer) clearTimeout(this.#statusTimer);
    this.#statusTimer = setTimeout(() => {
      this.status = null;
    }, STATUS_DURATION_MS);
  }

  #scheduleFlush(): void {
    if (this.#flushTimer) clearTimeout(this.#flushTimer);
    this.#flushTimer = setTimeout(() => void this.#flush(), FLUSH_DEBOUNCE_MS);
  }

  /** Runs the debounced flush immediately — used before export, so the file
   * reflects the edit just made rather than racing the debounce window. */
  async flushNow(): Promise<void> {
    if (this.#flushTimer) {
      clearTimeout(this.#flushTimer);
      this.#flushTimer = null;
      await this.#flush();
      return;
    }
    while (this.#flushing) {
      await new Promise((resolve) => setTimeout(resolve, 25));
    }
  }

  async #flush(): Promise<void> {
    this.#flushTimer = null;
    if (this.#flushing) {
      this.#flushAgainAfter = true;
      return;
    }
    this.#flushing = true;
    try {
      const currentIds = new Set(this.rows.map((r) => r.id));
      const toDelete = [...this.#synced.entries()].filter(([id]) => !currentIds.has(id));

      const results = await Promise.allSettled([
        ...toDelete.map(async ([id, synced]) => {
          await commands.deleteEntry(synced.serverId);
          this.#synced.delete(id);
        }),
        ...this.rows.map(async (row) => {
          const fields = toComparable(row);
          const existing = this.#synced.get(row.id);
          if (existing && fieldsEqual(existing, fields)) return;
          const dto = entryToDto(toEntry(row, existing?.serverId ?? null));
          if (existing) {
            await commands.updateEntry(dto);
            this.#synced.set(row.id, { serverId: existing.serverId, ...fields });
          } else {
            const created = entryFromDto(await commands.logEntry(dto));
            this.#synced.set(row.id, { serverId: created.id, ...fields });
          }
        }),
      ]);

      const failures = results.filter((r) => r.status === "rejected");
      if (failures.length > 0) {
        this.error = `${failures.length} change${failures.length === 1 ? "" : "s"} failed to save`;
      }
    } finally {
      this.#flushing = false;
      if (this.#flushAgainAfter) {
        this.#flushAgainAfter = false;
        void this.#flush();
      }
    }
  }

  #run(command: GridCommand): void {
    this.#stack.do(this, command);
    this.#scheduleFlush();
  }

  patch(patches: RowPatch[], label = "Edit"): void {
    if (patches.length === 0) return;
    this.#run(new PatchRowsCommand(patches, label));
  }

  insertRow(): void {
    const firstHabit = habitsStore.items[0];
    if (!firstHabit) {
      this.#setStatus("Add a habit before inserting a row.");
      return;
    }
    const row: GridRow = {
      id: nextTempId--,
      habitId: firstHabit.id,
      occurredAt: new Date(),
      quantity: 1,
      durationMinutes: null,
      note: null,
    };
    this.#run(new InsertRowCommand(this.rows.length, row));
  }

  deleteRows(rowIds: number[]): void {
    if (rowIds.length === 0) return;
    this.#run(new DeleteRowsCommand(rowIds));
  }

  undo(): void {
    if (!this.#stack.canUndo) return;
    this.#stack.undo(this);
    this.#scheduleFlush();
  }

  redo(): void {
    if (!this.#stack.canRedo) return;
    this.#stack.redo(this);
    this.#scheduleFlush();
  }

  announce(message: string): void {
    this.#setStatus(message);
  }
}

export const ledgerStore = new LedgerStore();
