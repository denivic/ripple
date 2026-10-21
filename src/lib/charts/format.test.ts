import { afterEach, describe, expect, it } from "vitest";
import { formatDuration, formatMoney, formatMoneyCompact, formatMoneyCompactParts, formatSignedPercent, multiScaleTimeFormat, setCurrencySource } from "./format";

describe("formatDuration", () => {
  it("formats zero as 0m", () => {
    expect(formatDuration(0)).toBe("0m");
  });

  it("never renders decimal hours", () => {
    expect(formatDuration(260)).toBe("4h 20m");
  });

  it("drops minutes when they're exactly zero", () => {
    expect(formatDuration(240)).toBe("4h");
  });

  it("renders sub-hour durations as minutes only", () => {
    expect(formatDuration(45)).toBe("45m");
  });

  it("rolls over to days at 24h, dropping minutes at that scale", () => {
    expect(formatDuration(3 * 24 * 60 + 5 * 60 + 30)).toBe("3d 5h");
  });

  it("preserves sign for negative durations (e.g. period deltas)", () => {
    expect(formatDuration(-260)).toBe("-4h 20m");
  });

  it("rounds fractional minutes", () => {
    expect(formatDuration(59.6)).toBe("1h");
  });
});

describe("formatSignedPercent", () => {
  it("prefixes a plus sign for positive values", () => {
    expect(formatSignedPercent(0.24)).toBe("+24%");
  });

  it("prefixes a minus sign for negative values", () => {
    expect(formatSignedPercent(-0.1)).toBe("-10%");
  });

  it("has no sign for exactly zero", () => {
    expect(formatSignedPercent(0)).toBe("0%");
  });
});

describe("multiScaleTimeFormat", () => {
  it("shows just the year at a year boundary", () => {
    expect(multiScaleTimeFormat(new Date(2026, 0, 1))).toBe("2026");
  });

  it("shows month abbreviation at a month boundary (non-January)", () => {
    expect(multiScaleTimeFormat(new Date(2026, 2, 1))).toBe("Mar");
  });

  it("shows month + day for an ordinary day", () => {
    expect(multiScaleTimeFormat(new Date(2026, 2, 14))).toBe("Mar 14");
  });

  it("shows hours:minutes for an intraday tick", () => {
    expect(multiScaleTimeFormat(new Date(2026, 2, 14, 8, 30))).toBe("08:30");
  });
});

describe("formatMoney", () => {
  // Fixed-locale expectations would be brittle (Intl output depends on the
  // runtime locale), so compare against Intl's own rendering instead.
  const expected = (amount: number, currency: string) =>
    new Intl.NumberFormat(undefined, { style: "currency", currency, maximumFractionDigits: 2 }).format(amount);

  afterEach(() => setCurrencySource(() => "USD"));

  it("defaults to USD before any currency source is installed", () => {
    expect(formatMoney(12.5)).toBe(expected(12.5, "USD"));
  });

  it("uses the installed currency source as the default", () => {
    setCurrencySource(() => "EUR");
    expect(formatMoney(12.5)).toBe(expected(12.5, "EUR"));
    expect(formatMoney(12.5)).not.toBe(expected(12.5, "USD"));
  });

  it("re-reads the source on every call", () => {
    let currency = "GBP";
    setCurrencySource(() => currency);
    expect(formatMoney(3)).toBe(expected(3, "GBP"));
    currency = "JPY";
    expect(formatMoney(3)).toBe(expected(3, "JPY"));
  });

  it("an explicit currency argument wins over the source", () => {
    setCurrencySource(() => "EUR");
    expect(formatMoney(7, "CHF")).toBe(expected(7, "CHF"));
  });
});

describe("formatMoneyCompact", () => {
  const intl = (amount: number, options: Intl.NumberFormatOptions) =>
    new Intl.NumberFormat(undefined, { style: "currency", currency: "DKK", ...options }).format(amount);

  it("keeps cents below 1,000", () => {
    expect(formatMoneyCompact(302.46, "DKK")).toBe(intl(302.46, { maximumFractionDigits: 2 }));
  });

  it("drops cents from 1,000", () => {
    expect(formatMoneyCompact(15_302.46, "DKK")).toBe(intl(15_302.46, { maximumFractionDigits: 0 }));
  });

  it("switches to compact notation from 100,000, including negatives", () => {
    const compact = { notation: "compact", maximumFractionDigits: 1 } as const;
    expect(formatMoneyCompact(153_020, "DKK")).toBe(intl(153_020, compact));
    expect(formatMoneyCompact(-2_400_000, "DKK")).toBe(intl(-2_400_000, compact));
  });
});

describe("formatMoneyCompactParts", () => {
  it("splits the currency from the figure without losing anything", () => {
    for (const amount of [0, 302.46, 15_302.46, -15_302.46, 153_020, 1_000_000_000_000]) {
      const { unit, figure } = formatMoneyCompactParts(amount, "DKK");
      const full = formatMoneyCompact(amount, "DKK");
      expect(unit).not.toBe("");
      expect(full).toContain(unit);
      expect(figure).not.toContain(unit);
      // Same digits, separators and sign, just without the unit and its spacing.
      expect(figure.replace(/[\s  ]/g, "")).toBe(full.replace(unit, "").replace(/[\s  ]/g, ""));
    }
  });

  it("keeps a trillion short", () => {
    expect(formatMoneyCompactParts(1_000_000_000_000, "DKK").figure.length).toBeLessThanOrEqual(5);
  });
});
