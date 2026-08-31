/** Mirrors src-tauri/src/interface/dto.rs exactly (camelCase serde output). */

export interface HabitDto {
  id: number | null;
  name: string;
  unitLabel: string;
  lifeMinutesPerUnit: number;
  costPerUnit: number;
  color: string | null;
  archived: boolean;
}

export interface EntryDto {
  id: number | null;
  habitId: number;
  /** Local wall-clock time, `YYYY-MM-DDTHH:MM:SS`. */
  occurredAt: string;
  quantity: number;
  durationMinutes: number | null;
  note: string | null;
}

export interface ProfileDto {
  birthDate: string | null;
  sex: "male" | "female" | null;
  lifeExpectancyYears: number | null;
  typicalSleepHours: number | null;
  netHourlyIncome: number | null;
  weightKg: number | null;
}

export interface DailyPointDto {
  date: string;
  timeSpentMinutes: number;
  lifeShortenedMinutes: number;
  money: number;
}

export interface HabitBreakdownItemDto {
  habitId: number;
  timeSpentMinutes: number;
  lifeShortenedMinutes: number;
  money: number;
}

export interface PeriodCompareDto {
  currentTotalMinutes: number;
  previousTotalMinutes: number;
  deltaMinutes: number;
  percentChange: number | null;
}

export interface TimelineDto {
  daily: DailyPointDto[];
  habitBreakdown: HabitBreakdownItemDto[];
  /** `[weekday 0=Mon..6=Sun][hour 0..23]` */
  hourWeekdayMatrix: number[][];
  currentStreakDays: number;
  longestStreakDays: number;
  periodCompare: PeriodCompareDto;
}

export interface TodaySummaryDto {
  timeSpentMinutes: number;
  lifeShortenedMinutes: number;
  moneySpent: number;
  opportunityCost: number | null;
  wakingLifeShareToday: number;
  remainingWakingLifeMonthsAtTodaysRate: number | null;
}

export interface HabitPresetDto {
  name: string;
  unitLabel: string;
  lifeMinutesPerUnit: number;
}

/** Mirrors `HabitMapping` (src-tauri/src/infrastructure/import/mapping.rs) —
 * a serde-external-tagged enum, so exactly one key is present. */
export type HabitMappingDto = { column: number } | { fixed: number };

export interface ColumnMappingDto {
  habit: HabitMappingDto;
  occurredAtColumn: number;
  quantityColumn: number | null;
  durationColumn: number | null;
  noteColumn: number | null;
  hasHeaderRow: boolean;
}

export interface RowErrorDto {
  rowIndex: number;
  message: string;
}

export interface SheetPreviewDto {
  name: string;
  rows: string[][];
}

export interface ImportPreviewDto {
  sheets: SheetPreviewDto[];
  sourceSignature: string;
  rememberedMapping: ColumnMappingDto | null;
}

export interface ImportSummaryDto {
  entriesCreated: number;
  rowErrors: RowErrorDto[];
}
