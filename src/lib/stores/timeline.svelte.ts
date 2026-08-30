import { commands, formatLocalDate, onDataChanged, timelineFromDto, type Timeline } from "$lib/ipc";
import type { UnlistenFn } from "@tauri-apps/api/event";

const DEFAULT_WINDOW_DAYS = 365;

function daysAgo(n: number): Date {
  const d = new Date();
  d.setDate(d.getDate() - n);
  return d;
}

class TimelineStore {
  data = $state<Timeline | null>(null);
  loading = $state(true);
  error = $state<string | null>(null);
  start = $state<Date>(daysAgo(DEFAULT_WINDOW_DAYS - 1));
  end = $state<Date>(new Date());

  #unlisten: UnlistenFn | null = null;

  async load(): Promise<void> {
    this.loading = true;
    this.error = null;
    try {
      this.data = timelineFromDto(await commands.computeTimeline(formatLocalDate(this.start), formatLocalDate(this.end)));
    } catch (e) {
      this.error = String(e);
    } finally {
      this.loading = false;
    }
  }

  async mount(): Promise<void> {
    await this.load();
    this.#unlisten ??= await onDataChanged((event) => {
      if (event.scope === "entries" || event.scope === "habits") void this.load();
    });
  }

  unmount(): void {
    this.#unlisten?.();
    this.#unlisten = null;
  }

  async setRange(start: Date, end: Date): Promise<void> {
    this.start = start;
    this.end = end;
    await this.load();
  }
}

export const timelineStore = new TimelineStore();
