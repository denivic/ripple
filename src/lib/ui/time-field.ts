/** Pure helpers behind `TimeField`. Its value is always a 24-hour "HH:MM"
 * string, whatever clock style the selects show. */

export type Period = "AM" | "PM";

export interface ClockParts {
  hour: number;
  minute: number;
}

/** "HH:MM" (or "H:MM") into parts, or null when it isn't a valid time. */
export function parseClockValue(value: string): ClockParts | null {
  const match = value.match(/^(\d{1,2}):(\d{2})$/);
  if (!match) return null;
  const hour = Number(match[1]);
  const minute = Number(match[2]);
  if (hour > 23 || minute > 59) return null;
  return { hour, minute };
}

export function formatClockValue(hour: number, minute: number): string {
  return `${String(hour).padStart(2, "0")}:${String(minute).padStart(2, "0")}`;
}

/** 0–23 into 12-hour form: 0 → 12 AM, 12 → 12 PM, 13 → 1 PM. */
export function toHour12(hour: number): { hour12: number; period: Period } {
  return { hour12: hour % 12 || 12, period: hour < 12 ? "AM" : "PM" };
}

/** The inverse of `toHour12`. */
export function fromHour12(hour12: number, period: Period): number {
  return (hour12 % 12) + (period === "PM" ? 12 : 0);
}

/** 0, 5, … 55, plus `current` in order when it isn't a multiple of 5, so a
 * stored "08:07" still shows as itself instead of silently changing. */
export function minuteOptions(current: number): number[] {
  const steps = Array.from({ length: 12 }, (_, i) => i * 5);
  if (current % 5 === 0 || current < 0 || current > 59) return steps;
  return [...steps, current].sort((a, b) => a - b);
}
