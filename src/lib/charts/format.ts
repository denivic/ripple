import { timeDay, timeHour, timeMinute, timeMonth, timeSecond, timeWeek, timeYear } from "d3-time";
import { timeFormat } from "d3-time-format";

/** Durations are always "4h 20m", never decimal hours. */
export function formatDuration(totalMinutes: number): string {
  const sign = totalMinutes < 0 ? "-" : "";
  const abs = Math.round(Math.abs(totalMinutes));
  const days = Math.floor(abs / (24 * 60));
  const hours = Math.floor((abs % (24 * 60)) / 60);
  const minutes = abs % 60;

  if (days > 0) {
    return `${sign}${days}d ${hours}h`;
  }
  if (hours > 0) {
    return minutes > 0 ? `${sign}${hours}h ${minutes}m` : `${sign}${hours}h`;
  }
  return `${sign}${minutes}m`;
}

export function formatMoney(amount: number, currency = "USD"): string {
  return new Intl.NumberFormat(undefined, { style: "currency", currency, maximumFractionDigits: 2 }).format(amount);
}

export function formatPercent(fraction: number, digits = 0): string {
  // Fixed locale (unlike formatCalendarDate): some locales insert a space
  // before "%", which reads poorly as a compact chart/badge value.
  return new Intl.NumberFormat("en-US", { style: "percent", maximumFractionDigits: digits }).format(fraction);
}

export function formatSignedPercent(fraction: number, digits = 0): string {
  const formatted = formatPercent(Math.abs(fraction), digits);
  if (fraction > 0) return `+${formatted}`;
  if (fraction < 0) return `-${formatted}`;
  return formatted;
}

/** Locale-respecting, for prose and tooltips (e.g. "Mar 14, 2026"). */
export function formatCalendarDate(date: Date): string {
  return new Intl.DateTimeFormat(undefined, { month: "short", day: "numeric", year: "numeric" }).format(date);
}

const formatMillisecond = timeFormat(".%L");
const formatSecond = timeFormat(":%S");
const formatMinuteTick = timeFormat("%H:%M");
const formatHourTick = timeFormat("%H:%M");
const formatDayTick = timeFormat("%b %d");
const formatWeekTick = timeFormat("%b %d");
const formatMonthTick = timeFormat("%b");
const formatYearTick = timeFormat("%Y");

/**
 * The canonical d3 "multi-scale" time format: picks the most specific unit
 * that actually changed at this tick, coarsening automatically as the axis
 * spans more time — so a daily axis shows "Mar 14" while a yearly one shows
 * just the year, with no manual scale-switching logic at call sites.
 */
export function multiScaleTimeFormat(date: Date): string {
  const format =
    timeSecond(date) < date
      ? formatMillisecond
      : timeMinute(date) < date
        ? formatSecond
        : timeHour(date) < date
          ? formatMinuteTick
          : timeDay(date) < date
            ? formatHourTick
            : timeMonth(date) < date
              ? timeWeek(date) < date
                ? formatDayTick
                : formatWeekTick
              : timeYear(date) < date
                ? formatMonthTick
                : formatYearTick;
  return format(date);
}
