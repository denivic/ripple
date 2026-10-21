<p align="center">
  <img src="github-banner.png" alt="Ripple — Small actions. Lasting consequences." width="100%">
</p>

# Ripple

The cumulative cost of a habit, made immediate and legible. And the progress you make when you change it.

## Contents

- [Why this exists](#why-this-exists)
- [The name](#the-name)
- [Features](#features)
- [How the numbers are estimated](#how-the-numbers-are-estimated)
- [Install](#install)
- [Usage](#usage)
- [Stack](#stack)

## Why this exists

Bad habits are hard to quit because their cost is deferred and invisible. A single twenty-minute scroll session, one cigarette, one drink: each is individually trivial, which is exactly why the aggregate never becomes real. The feedback loop that would prompt a change is separated from the behavior by years.

Ripple closes that loop. It turns a habit's cumulative cost into something immediate: time spent, money spent, and an estimate of life shortened, shown alongside the raw counts rather than in place of them. It works the other way too: when your habits improve, Ripple shows the time you're winning back, keeps your streaks, and rewards the change with points and levels.

## The name

A small, repeated action propagates outward into something far larger than its origin. That's the thesis the app makes visible.

## Features

- **Today**
  - One-click logging against your habits, with a running toll of the day: time spent, life shortened or gained, money spent, share of today's waking hours, and opportunity cost.
  - **At your 30-day pace**: projects your average daily intake forward to your life-expectancy age, as months of waking life lost (or gained). It averages at least 7 days, so a single entry can't swing it.
  - **This week vs last**: the change in net life lost, and how many months of waking life that pace wins back.
  - A **clean-streak** tile, and a week-over-week trend badge on every habit card.
- **Cited habit presets**
  - 24 presets, each with its source, year, confidence level and a short note: tobacco, alcohol, foods and drinks, sedentary time, and habits that add life (exercise, fruit, nuts, coffee).
  - Habits with no credible per-unit estimate (cannabis, cigars, pipes, vaping, energy drinks) ship at 0 with an explanation instead of an invented number.
  - Every preset is a starting point: name, unit, life effect, minutes per unit and cost stay editable.
- **Timeline**
  - Cumulative cost over time with a forward projection, plus daily and weekly totals with a moving average.
  - A calendar heatmap and an hour-of-day × day-of-week matrix.
  - A per-habit breakdown by life, time or money.
  - Streaks and period-over-period comparison.
- **Progress**
  - Points for logging, clean days, habits that improve week over week, and streaks.
  - Water-themed levels from *Drop* to *Still Water*, and milestones.
  - Points are stored in a ledger and never go down, so logging a bad day honestly is never punished.
- **Notifications**
  - Four categories: daily reminders, progress & points, a weekly summary, and streak & pace alerts.
  - Each category has its own switch and delivery: local, email or both.
  - Configurable times and quiet hours. Anything due during quiet hours waits until they end.
  - Ripple can keep running in the menu bar / system tray and launch at login, so reminders fire on time.
  - Email goes through your own SMTP account (TLS only), Resend, or Postmark. The password or API key is stored in the macOS Keychain or Windows Credential Manager, never in the database.
- **Ledger**
  - A virtualized spreadsheet-style editor for every logged entry, with typed cells, undo/redo, TSV copy-paste, fill-down, sort and filter, and inline validation.
  - Imports `.xlsx`, `.csv` and `.numbers` through a column-mapping wizard that remembers the mapping per source file. Exports to `.xlsx` or `.csv`.
- **Profile**: the inputs that feed the cost model (birth date, sex, life expectancy, typical sleep, net hourly income, weight). Each field is labeled with the number it affects.
- **Settings**
  - Start page and week start.
  - **Time format**: 24-hour by default, with 12-hour as an option.
  - **Units**: metric by default, with imperial as an option. Units cover weight and preset serving sizes.
  - **Currency**: defaults to your system region's currency. When you switch, you can convert stored amounts at today's European Central Bank rate (via Frankfurter) or change only the symbol.
  - Theme editor: built-in themes or hand-picked colors, live preview, WCAG-AA contrast warnings, JSON import/export.
  - Notification and email setup.
  - Backup and restore.
  - A guarded **Reset database** option that erases everything after confirmation.
- **Theming and accessibility**
  - Every chart, the grid and the window chrome read from one token set, so a theme change repaints them all together.
  - The data palette is colorblind-safe and WCAG-checked.
  - Respects `prefers-reduced-motion`, `prefers-reduced-transparency` and `prefers-contrast`.

## How the numbers are estimated

Each habit has four editable assumptions:

- **life effect per unit**: minutes of life lost per unit, or gained if negative;
- **minutes per unit**: time spent;
- **cost per unit**;
- **unit**.

An entry's time is its own duration if you entered one, otherwise quantity × minutes per unit.

The life figures are population-level estimates from published research. Examples:

- the cigarette figure comes from Jackson et al. (2025);
- the alcohol, TV and exercise figures come from Spiegelhalter's microlives;
- the food figures come from the Health Nutritional Index (Stylianou et al., 2021).

None of them is a personal prognosis. Projections assume you keep your recent average pace until the life expectancy set in your Profile, counted in waking hours. They estimate what a pace adds up to, not how long you have left.

## Install

Prerequisites: [Node.js](https://nodejs.org) and [Rust](https://rustup.rs) via `rustup`.

**Windows** (10/11; WebView2 ships with the OS):

```
npm install
npm run tauri build
```

The installer lands in `src-tauri/target/release/bundle/`.

**macOS**: install the Xcode Command Line Tools (`xcode-select --install`), then run the same two commands. WKWebView is Apple's platform webview and ships with the OS.

`npm run tauri dev` runs the app with hot reload during development instead of producing an installer. In development builds, macOS shows notifications as coming from Terminal and Windows toasts may look generic; installed builds show them as Ripple.

Tests: `npm test` (frontend), `cd src-tauri && cargo test` (Rust), and `npm run check` (types).

## Usage

- **Add a habit**: pick a preset or define your own. The preset fills in its values and shows its source.
- **Today**: log occurrences. The tiles and trend badges update immediately.
- **Timeline**: shows the same data as charts, animating to new values as entries change.
- **Ledger**: shows the same data as a spreadsheet. Edit any field directly, or import an existing tracking spreadsheet through the mapping wizard. Typed times are accepted as either 13:05 or 1:05 PM.
- **Profile**: fill in what applies. Every field is optional, and a metric that needs a missing field simply doesn't render.
- **Progress**: see your level, milestones, recent awards and how points are earned.
- **Settings**: set your clock, units and currency, configure notifications and email, edit a theme, and back up, restore or reset the database.

## Stack

Tauri 2 (Rust core), Svelte 5, TypeScript and SQLite.

Tauri drives the platform's own webview (WebView2 on Windows, WKWebView on macOS) instead of bundling a Chromium runtime. The result is a native Rust binary with a small installer footprint.

Rust handles the heavy work: spreadsheet parsing, SQL aggregation over years of entries, the points ledger, and the notification scheduler. Email is sent with `lettre` over rustls, secrets live in the OS keychain via `keyring`, and the tray and launch-at-login come from Tauri plugins. CSS and the web platform handle the interface: gradients, translucent materials, spring-driven motion and live-updating charts, which they do better than a native toolkit would.

Svelte 5's runes compile reactivity away rather than diffing a virtual DOM, so a chart update costs a scale recomputation, not a re-render.
