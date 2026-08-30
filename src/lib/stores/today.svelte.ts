import {
  commands,
  entryFromDto,
  entryToDto,
  formatLocalDate,
  onDataChanged,
  type Entry,
  type HabitPreset,
  type TodaySummary,
} from "$lib/ipc";
import type { UnlistenFn } from "@tauri-apps/api/event";

class TodayStore {
  entries = $state<Entry[]>([]);
  summary = $state<TodaySummary | null>(null);
  presets = $state<HabitPreset[]>([]);
  loading = $state(true);
  error = $state<string | null>(null);

  #unlisten: UnlistenFn | null = null;

  async load(): Promise<void> {
    this.loading = true;
    this.error = null;
    try {
      const todayStr = formatLocalDate(new Date());
      const [entryDtos, summary, presets] = await Promise.all([
        commands.listEntriesBetween(`${todayStr}T00:00:00`, `${todayStr}T23:59:59`),
        commands.computeTodaySummary(todayStr),
        this.presets.length ? Promise.resolve(this.presets) : commands.getHabitPresets(),
      ]);
      this.entries = entryDtos.map(entryFromDto).sort((a, b) => b.occurredAt.getTime() - a.occurredAt.getTime());
      this.summary = summary;
      this.presets = presets;
    } catch (e) {
      this.error = String(e);
    } finally {
      this.loading = false;
    }
  }

  async mount(): Promise<void> {
    await this.load();
    this.#unlisten ??= await onDataChanged((event) => {
      if (event.scope === "entries" || event.scope === "profile") void this.load();
    });
  }

  unmount(): void {
    this.#unlisten?.();
    this.#unlisten = null;
  }

  async log(habitId: number, quantity: number, durationMinutes: number | null = null): Promise<void> {
    await commands.logEntry(
      entryToDto({ habitId, occurredAt: new Date(), quantity, durationMinutes, note: null }),
    );
  }

  async remove(entryId: number): Promise<void> {
    await commands.deleteEntry(entryId);
  }
}

export const todayStore = new TodayStore();
