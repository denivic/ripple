import { formatLocalDate, formatLocalDateTime, parseLocalDate, parseLocalDateTime } from "./datetime";
import type {
  DailyPointDto,
  EntryDto,
  HabitBreakdownItemDto,
  HabitDto,
  HabitPresetDto,
  PeriodCompareDto,
  ProfileDto,
  TimelineDto,
  TodaySummaryDto,
} from "./dto";

/** Frontend-shaped models: DTOs with wire-format date strings converted to
 * real `Date` objects, since every consumer (charts, forms) wants Dates. */

export interface Habit {
  id: number;
  name: string;
  unitLabel: string;
  lifeMinutesPerUnit: number;
  costPerUnit: number;
  color: string | null;
  archived: boolean;
}

export interface Entry {
  id: number;
  habitId: number;
  occurredAt: Date;
  quantity: number;
  durationMinutes: number | null;
  note: string | null;
}

export interface Profile {
  birthDate: Date | null;
  sex: "male" | "female" | null;
  lifeExpectancyYears: number | null;
  typicalSleepHours: number | null;
  netHourlyIncome: number | null;
  weightKg: number | null;
}

export function habitFromDto(dto: HabitDto): Habit {
  if (dto.id === null) throw new Error("habitFromDto: DTO has no id — expected a persisted habit");
  return { ...dto, id: dto.id };
}

export function habitToDto(habit: Omit<Habit, "id"> & { id?: number | null }): HabitDto {
  return { ...habit, id: habit.id ?? null };
}

export function entryFromDto(dto: EntryDto): Entry {
  if (dto.id === null) throw new Error("entryFromDto: DTO has no id — expected a persisted entry");
  return {
    id: dto.id,
    habitId: dto.habitId,
    occurredAt: parseLocalDateTime(dto.occurredAt),
    quantity: dto.quantity,
    durationMinutes: dto.durationMinutes,
    note: dto.note,
  };
}

export function entryToDto(entry: Omit<Entry, "id"> & { id?: number | null }): EntryDto {
  return {
    id: entry.id ?? null,
    habitId: entry.habitId,
    occurredAt: formatLocalDateTime(entry.occurredAt),
    quantity: entry.quantity,
    durationMinutes: entry.durationMinutes,
    note: entry.note,
  };
}

export function profileFromDto(dto: ProfileDto): Profile {
  return { ...dto, birthDate: dto.birthDate ? parseLocalDate(dto.birthDate) : null };
}

export function profileToDto(profile: Profile): ProfileDto {
  return { ...profile, birthDate: profile.birthDate ? formatLocalDate(profile.birthDate) : null };
}

export interface DailyPoint {
  date: Date;
  timeSpentMinutes: number;
  lifeShortenedMinutes: number;
  money: number;
}

export type HabitBreakdownItem = HabitBreakdownItemDto;
export type PeriodCompare = PeriodCompareDto;
export type TodaySummary = TodaySummaryDto;
export type HabitPreset = HabitPresetDto;

export interface Timeline {
  daily: DailyPoint[];
  habitBreakdown: HabitBreakdownItem[];
  hourWeekdayMatrix: number[][];
  currentStreakDays: number;
  longestStreakDays: number;
  periodCompare: PeriodCompare;
}

function dailyPointFromDto(dto: DailyPointDto): DailyPoint {
  return { ...dto, date: parseLocalDate(dto.date) };
}

export function timelineFromDto(dto: TimelineDto): Timeline {
  return {
    daily: dto.daily.map(dailyPointFromDto),
    habitBreakdown: dto.habitBreakdown,
    hourWeekdayMatrix: dto.hourWeekdayMatrix,
    currentStreakDays: dto.currentStreakDays,
    longestStreakDays: dto.longestStreakDays,
    periodCompare: dto.periodCompare,
  };
}
