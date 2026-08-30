/**
 * Explicit local-time parse/format, matching the Rust side's codec exactly
 * (`YYYY-MM-DD[THH:MM:SS]`, no offset). Deliberately not `new Date(iso)` /
 * `date.toISOString()`: a date-only ISO string parses as UTC per spec, and
 * `toISOString()` always emits UTC — both would silently shift the wall-clock
 * time this app stores and displays as local.
 */

function pad(n: number): string {
  return String(n).padStart(2, "0");
}

export function formatLocalDate(date: Date): string {
  return `${date.getFullYear()}-${pad(date.getMonth() + 1)}-${pad(date.getDate())}`;
}

export function parseLocalDate(s: string): Date {
  const [y, m, d] = s.split("-").map(Number);
  return new Date(y, m - 1, d);
}

export function formatLocalDateTime(date: Date): string {
  return `${formatLocalDate(date)}T${pad(date.getHours())}:${pad(date.getMinutes())}:${pad(date.getSeconds())}`;
}

export function parseLocalDateTime(s: string): Date {
  const [datePart, timePart] = s.split("T");
  const [y, m, d] = datePart.split("-").map(Number);
  const [hh, mm, ss] = timePart.split(":").map(Number);
  return new Date(y, m - 1, d, hh, mm, ss);
}
