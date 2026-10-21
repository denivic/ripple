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

export type StartupRouteDto = "today" | "timeline" | "ledger" | "profile" | "settings";

export interface SettingsDto {
  startupRoute: StartupRouteDto;
  weekStart: "monday" | "sunday";
  weightUnit: "kg" | "lb";
  themeName: string | null;
  customThemeJson: string | null;
  /** ISO 4217 code, e.g. "USD". */
  currency: string;
}

export interface CurrencyConversionDto {
  /** How many of the new currency one unit of the old was worth. */
  rate: number;
  convertedHabits: number;
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
  /** Months of waking life lost by `lifeExpectancyAge` at the window's
   *  average daily pace; negative = months gained. Null without a complete profile. */
  projectedWakingMonthsLost: number | null;
  /** Days the daily average is taken over: 30, or fewer (min 7) for a short history. */
  projectionWindowDays: number;
  /** Net life-minutes per day over the window; negative = net gain. */
  averageDailyLifeMinutes: number;
  lifeExpectancyAge: number | null;
}

export type PresetCategory = "tobacco" | "alcohol" | "cannabis" | "food" | "drink" | "sedentary" | "gain";
export type PresetConfidence = "high" | "medium" | "low" | "insufficient";

export interface HabitPresetDto {
  id: string;
  name: string;
  unitLabel: string;
  /** Positive = life lost per unit, negative = life gained. */
  lifeMinutesPerUnit: number;
  category: PresetCategory;
  /** Null only for "insufficient" presets with no source at all. */
  sourceTitle: string | null;
  sourceUrl: string | null;
  year: number | null;
  confidence: PresetConfidence;
  derived: boolean;
  derivation: string | null;
  note: string;
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
