import { commands, formatLocalDate, onDataChanged, type PointAwardDto, type ProgressDto } from "$lib/ipc";
import type { UnlistenFn } from "@tauri-apps/api/event";

/** How long the "+10 · Clean day" notice stays up. */
const NOTICE_MS = 4000;

function noticeText(awards: PointAwardDto[]): string {
  const points = awards.reduce((sum, a) => sum + a.points, 0);
  const what = awards.length === 1 ? awards[0].label : `${awards.length} awards`;
  return `+${points} · ${what}`;
}

class ProgressStore {
  data = $state<ProgressDto | null>(null);
  loading = $state(true);
  error = $state<string | null>(null);
  /** Brief summary of awards earned since the last load; null when idle. */
  notice = $state<string | null>(null);

  // Reference-counted: the layout and the Progress page each mount/unmount
  // the store, and a page leaving must not tear down the layout's listener.
  #refs = 0;
  #listening: Promise<UnlistenFn> | null = null;
  // Highest award id seen so far; null until the first load, so the startup
  // backfill isn't announced as if it had just been earned.
  #lastAwardId: number | null = null;
  #noticeTimer: ReturnType<typeof setTimeout> | null = null;

  async load(): Promise<void> {
    this.loading = true;
    this.error = null;
    try {
      // get_progress evaluates the history before answering, so this is
      // also what awards points on startup and after every change.
      const data = await commands.getProgress(formatLocalDate(new Date()));
      const maxId = data.recentAwards.reduce((max, a) => Math.max(max, a.id), 0);
      if (this.#lastAwardId !== null) {
        const fresh = data.recentAwards.filter((a) => a.id > (this.#lastAwardId ?? 0));
        if (fresh.length) this.#showNotice(noticeText(fresh));
      }
      // A database reset restarts ids, so follow them down as well as up.
      this.#lastAwardId = maxId;
      this.data = data;
    } catch (e) {
      this.error = String(e);
    } finally {
      this.loading = false;
    }
  }

  async mount(): Promise<void> {
    this.#refs += 1;
    // Assigned synchronously so concurrent mounts share one subscription.
    const listening = (this.#listening ??= onDataChanged((event) => {
      if (
        event.scope === "entries" ||
        event.scope === "habits" ||
        event.scope === "settings" ||
        event.scope === "points"
      ) {
        void this.load();
      }
    }).catch((e) => {
      this.#listening = null;
      throw e;
    }));
    await this.load();
    await listening;
  }

  unmount(): void {
    if (this.#refs === 0) return;
    this.#refs -= 1;
    if (this.#refs > 0 || !this.#listening) return;
    const listening = this.#listening;
    this.#listening = null;
    void listening.then((unlisten) => unlisten()).catch(() => {});
  }

  #showNotice(text: string): void {
    this.notice = text;
    if (this.#noticeTimer) clearTimeout(this.#noticeTimer);
    this.#noticeTimer = setTimeout(() => {
      this.notice = null;
      this.#noticeTimer = null;
    }, NOTICE_MS);
  }
}

export const progressStore = new ProgressStore();
