import { invoke } from "@tauri-apps/api/core";
import type { EntryDto, HabitDto, HabitPresetDto, ProfileDto, TimelineDto, TodaySummaryDto } from "./dto";

export const commands = {
  listHabits: (includeArchived: boolean) => invoke<HabitDto[]>("list_habits", { includeArchived }),
  getHabit: (habitId: number) => invoke<HabitDto | null>("get_habit", { habitId }),
  createHabit: (habit: HabitDto) => invoke<HabitDto>("create_habit", { habit }),
  updateHabit: (habit: HabitDto) => invoke<void>("update_habit", { habit }),
  archiveHabit: (habitId: number) => invoke<void>("archive_habit", { habitId }),

  logEntry: (entry: EntryDto) => invoke<EntryDto>("log_entry", { entry }),
  getEntry: (entryId: number) => invoke<EntryDto | null>("get_entry", { entryId }),
  updateEntry: (entry: EntryDto) => invoke<void>("update_entry", { entry }),
  deleteEntry: (entryId: number) => invoke<void>("delete_entry", { entryId }),
  listEntries: () => invoke<EntryDto[]>("list_entries"),
  listEntriesBetween: (start: string, end: string) => invoke<EntryDto[]>("list_entries_between", { start, end }),

  getProfile: () => invoke<ProfileDto>("get_profile"),
  saveProfile: (profile: ProfileDto) => invoke<void>("save_profile", { profile }),

  computeTimeline: (start: string, end: string) => invoke<TimelineDto>("compute_timeline", { start, end }),
  computeTodaySummary: (today: string) => invoke<TodaySummaryDto>("compute_today_summary", { today }),
  getHabitPresets: () => invoke<HabitPresetDto[]>("get_habit_presets"),
};
