import { commands, habitFromDto, habitToDto, onDataChanged, type Habit } from "$lib/ipc";
import type { UnlistenFn } from "@tauri-apps/api/event";

class HabitsStore {
  items = $state<Habit[]>([]);
  loading = $state(true);
  error = $state<string | null>(null);

  #unlisten: UnlistenFn | null = null;

  async load(includeArchived = false): Promise<void> {
    this.loading = true;
    this.error = null;
    try {
      this.items = (await commands.listHabits(includeArchived)).map(habitFromDto);
    } catch (e) {
      this.error = String(e);
    } finally {
      this.loading = false;
    }
  }

  async mount(): Promise<void> {
    await this.load();
    this.#unlisten ??= await onDataChanged((event) => {
      if (event.scope === "habits") void this.load();
    });
  }

  unmount(): void {
    this.#unlisten?.();
    this.#unlisten = null;
  }

  byId(id: number): Habit | undefined {
    return this.items.find((h) => h.id === id);
  }

  async create(habit: Omit<Habit, "id">): Promise<Habit> {
    return habitFromDto(await commands.createHabit(habitToDto(habit)));
  }

  async update(habit: Habit): Promise<void> {
    await commands.updateHabit(habitToDto(habit));
  }

  async archive(id: number): Promise<void> {
    await commands.archiveHabit(id);
  }
}

export const habitsStore = new HabitsStore();
