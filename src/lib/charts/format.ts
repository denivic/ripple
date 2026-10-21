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

/**
 * Where `formatMoney` gets its default currency. The settings store installs
 * a getter that reads its `$state`, so templates calling `formatMoney(x)`
 * re-render when the currency changes — while this module stays free of any
 * store/IPC import (and testable under plain vitest).
 */
let currencySource: () => string = () => "USD";

export function setCurrencySource(source: () => string): void {
  currencySource = source;
}

export function formatMoney(amount: number, currency = currencySource()): string {
  return new Intl.NumberFormat(undefined, { style: "currency", currency, maximumFractionDigits: 2 }).format(amount);
}

/**
 * Money sized for a stat tile: cents only while they still matter, whole
 * units from 1,000, and compact notation ("DKK 153K") from 100,000. Pair it
 * with `formatMoney` for the exact figure in a tooltip.
 */
export function formatMoneyCompact(amount: number, currency = currencySource()): string {
  const abs = Math.abs(amount);
  const options: Intl.NumberFormatOptions =
    abs >= 100_000
      ? { notation: "compact", maximumFractionDigits: 1 }
      : abs >= 1_000
        ? { maximumFractionDigits: 0 }
        : { maximumFractionDigits: 2 };
  return new Intl.NumberFormat(undefined, { style: "currency", currency, ...options }).format(amount);
}

export interface MoneyParts {
  /** The currency symbol or code as the locale renders it ("DKK", "kr.", "$"). */
  unit: string;
  unitPosition: "before" | "after";
  /** Everything else, sign included ("-15,302"). */
  figure: string;
}

/**
 * `formatMoneyCompact`, split so a tile can size the figure and the currency
 * independently and move the currency onto its own line when space is short.
 */
export function formatMoneyCompactParts(amount: number, currency = currencySource()): MoneyParts {
  const formatted = formatMoneyCompact(amount, currency);
  const parts = new Intl.NumberFormat(undefined, {
    style: "currency",
    currency,
    ...(Math.abs(amount) >= 100_000 ? { notation: "compact" as const, maximumFractionDigits: 1 } : {}),
  }).formatToParts(amount);
  const unitIndex = parts.findIndex((p) => p.type === "currency");
  const firstNumberIndex = parts.findIndex((p) => p.type === "integer");
  if (unitIndex === -1) return { unit: "", unitPosition: "before", figure: formatted };
  const unit = parts[unitIndex].value;
  // Strip the unit (and the space Intl puts next to it) from the full string
  // rather than re-joining parts, so fraction digits match formatMoneyCompact.
  const figure = formatted.replace(unit, "").replace(/^[\s  ]+|[\s  ]+$/g, "").replace(/(-|−)[\s  ]+/, "$1");
  return { unit, unitPosition: unitIndex < firstNumberIndex ? "before" : "after", figure };
}

export type TimeFormat = "24h" | "12h";

/** Where the clock helpers get the 24h/12h choice — installed by the settings
 * store, like `setCurrencySource`, so times re-render when it changes. */
let timeFormatSource: () => TimeFormat = () => "24h";

export function setTimeFormatSource(source: () => TimeFormat): void {
  timeFormatSource = source;
}

function pad2(n: number): string {
  return String(n).padStart(2, "0");
}

/** A time of day: "13:05" in 24-hour mode, "1:05 PM" in 12-hour mode. */
export function formatClock(hours: number, minutes: number, format = timeFormatSource()): string {
  if (format === "24h") return `${pad2(hours)}:${pad2(minutes)}`;
  const period = hours < 12 ? "AM" : "PM";
  return `${hours % 12 || 12}:${pad2(minutes)} ${period}`;
}

/** `formatClock` for a Date's local time. */
export function formatClockTime(date: Date, format = timeFormatSource()): string {
  return formatClock(date.getHours(), date.getMinutes(), format);
}

/** A bare hour for compact axis labels: "06" in 24-hour mode, "6 AM" in 12-hour. */
export function formatHourLabel(hour: number, format = timeFormatSource()): string {
  if (format === "24h") return pad2(hour);
  return `${hour % 12 || 12} ${hour < 12 ? "AM" : "PM"}`;
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
        : timeDay(date) < date
          ? // Minute and hour ticks are times of day, so they follow the setting.
            (d: Date) => formatClockTime(d)
          : timeMonth(date) < date
            ? timeWeek(date) < date
              ? formatDayTick
              : formatWeekTick
            : timeYear(date) < date
              ? formatMonthTick
              : formatYearTick;
  return format(date);
}
