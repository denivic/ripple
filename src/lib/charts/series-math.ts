/**
 * Metric-agnostic array reductions over the daily series the backend sends
 * (see `application::compute_series::compute_timeline`). These stay
 * client-side deliberately: they're generic math over whichever metric
 * (time/life-minutes/money) the user is currently viewing, and computing
 * them here means toggling the metric never needs a round trip.
 */

export function cumulativeSum(values: number[]): number[] {
  let running = 0;
  return values.map((v) => (running += v));
}

/** Trailing-window average over a *continuous* (zero-filled) series. */
export function movingAverage(values: number[], windowDays: number): number[] {
  if (windowDays <= 0) return values.map(() => 0);
  return values.map((_, i) => {
    const start = Math.max(0, i - windowDays + 1);
    const window = values.slice(start, i + 1);
    return window.reduce((a, b) => a + b, 0) / window.length;
  });
}

export function recentDailyRate(values: number[], trailingDays: number): number {
  if (values.length === 0 || trailingDays <= 0) return 0;
  const window = values.slice(Math.max(0, values.length - trailingDays));
  return window.reduce((a, b) => a + b, 0) / window.length;
}

export interface ProjectedPoint {
  date: Date;
  value: number;
}

/** Linear extrapolation from `fromValue` at `dailyRate`/day — the dashed
 * forward cone on the Cumulative Ripple chart. */
export function forwardProjection(
  fromDate: Date,
  fromValue: number,
  dailyRate: number,
  daysForward: number,
): ProjectedPoint[] {
  return Array.from({ length: daysForward + 1 }, (_, i) => {
    const date = new Date(fromDate);
    date.setDate(date.getDate() + i);
    return { date, value: fromValue + dailyRate * i };
  });
}
