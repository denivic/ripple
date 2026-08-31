# Ripple

The cumulative cost of a habit, made immediate and legible.

## Contents

- [Why this exists](#why-this-exists)
- [The name](#the-name)
- [Features](#features)
- [Install](#install)
- [Usage](#usage)
- [Stack](#stack)

## Why this exists

Bad habits are hard to quit because their cost is deferred and invisible. A single twenty-minute scroll session, one cigarette, one drink: each is individually trivial, which is exactly why the aggregate never becomes real. The feedback loop that would prompt a change is separated from the behavior by years.

Ripple closes that loop. It turns a habit's cumulative cost into something immediate: not "4 cigarettes today" but "61 hours and 9 days of life expectancy this year, 2.1 months of remaining waking life at this rate." The cost is estimated from a small set of editable assumptions (time per occurrence, life-minutes shortened per occurrence, cost per occurrence), shown alongside the raw counts rather than in place of them.

## The name

A small, repeated action propagates outward into something far larger than its origin. That's the thesis the app makes visible.

## Features

- **Today**: one-tap logging against user-defined habits, with a running toll of the day's time, money, and estimated life cost.
- **Timeline**: cumulative cost over time with a forward projection, daily and weekly totals with a moving average, a calendar heatmap, an hour-of-day by day-of-week matrix, a per-habit breakdown (time, money, or life-minutes), streak tracking, and period-over-period comparison.
- **Ledger**: a virtualized spreadsheet-style editor for every logged entry, with typed cells, undo/redo, TSV copy-paste, fill-down, sort and filter, and inline validation. Imports `.xlsx`, `.csv`, and `.numbers` through a column-mapping wizard that remembers the mapping per source file; exports to `.xlsx` or `.csv`.
- **Theming**: two built-in themes sharing one WCAG-checked, colorblind-safe data palette. Every chart, the grid, and the chrome read from the same token set, so a theme change repaints all of them together. Respects `prefers-reduced-motion`, `prefers-reduced-transparency`, and `prefers-contrast`.

## Install

Prerequisites: [Node.js](https://nodejs.org) and [Rust](https://rustup.rs) via `rustup`.

**Windows** (10/11; WebView2 ships with the OS):

```
npm install
npm run tauri build
```

The installer lands in `src-tauri/target/release/bundle/`.

**macOS**: install the Xcode Command Line Tools (`xcode-select --install`), then run the same two commands. WKWebView is Apple's platform webview and ships with the OS.

`npm run tauri dev` runs the app with hot reload during development instead of producing an installer.

## Usage

- **Today**: log occurrences; the running toll updates immediately.
- **Timeline**: reads the same data for its charts, animating to new values as entries change with no reload needed.
- **Ledger**: presents the same data as a spreadsheet. Edit any field directly, or import an existing tracking spreadsheet through the mapping wizard.

## Stack

Tauri 2 (Rust core), Svelte 5, TypeScript, and SQLite.

Tauri drives the platform's own webview (WebView2 on Windows, WKWebView on macOS) instead of bundling a Chromium runtime, producing a native Rust binary with a small installer footprint.

Rust handles the heavy work: spreadsheet parsing and SQL aggregation over years of entries. CSS and the web platform handle the interface work: gradients, translucent materials, spring-driven motion, and live-updating charts, which they do better than a native toolkit would.

Svelte 5's runes compile reactivity away rather than diffing a virtual DOM, so a chart update costs a scale recomputation, not a re-render.
