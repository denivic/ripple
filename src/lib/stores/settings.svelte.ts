import { commands, onDataChanged, type SettingsDto } from "$lib/ipc";
import { builtInThemes, themeStore, validateTheme, type Theme } from "$lib/theme";
import type { UnlistenFn } from "@tauri-apps/api/event";

const DEFAULT_SETTINGS: SettingsDto = {
  startupRoute: "today",
  weekStart: "monday",
  weightUnit: "kg",
  themeName: null,
  customThemeJson: null,
};

class SettingsStore {
  data = $state<SettingsDto>(DEFAULT_SETTINGS);
  loading = $state(true);
  error = $state<string | null>(null);

  #unlisten: UnlistenFn | null = null;

  async load(): Promise<void> {
    this.loading = true;
    this.error = null;
    try {
      this.data = await commands.getSettings();
      this.#applyTheme();
    } catch (e) {
      this.error = String(e);
    } finally {
      this.loading = false;
    }
  }

  async mount(): Promise<void> {
    await this.load();
    this.#unlisten ??= await onDataChanged((event) => {
      if (event.scope === "settings") void this.load();
    });
  }

  unmount(): void {
    this.#unlisten?.();
    this.#unlisten = null;
  }

  /** Resolves the persisted theme choice to an actual `Theme` and applies it
   * — a custom edited theme wins over a built-in name when both somehow
   * disagree, since `customThemeJson` is only ever set by an active edit. */
  #applyTheme(): void {
    if (this.data.customThemeJson) {
      try {
        const result = validateTheme(JSON.parse(this.data.customThemeJson));
        if (result.valid) {
          themeStore.set(result.theme);
          return;
        }
      } catch {
        // Malformed JSON in storage — fall through to the built-in lookup.
      }
    }
    const preset = builtInThemes.find((t) => t.name === this.data.themeName) ?? builtInThemes[0];
    themeStore.set(preset);
  }

  async save(patch: Partial<SettingsDto>): Promise<void> {
    this.data = { ...this.data, ...patch };
    await commands.saveSettings(this.data);
  }

  /** Picking a built-in theme: applies it and remembers it by name, so a
   * later app update to that preset's colors is picked up automatically. */
  async applyBuiltIn(theme: Theme): Promise<void> {
    themeStore.set(theme);
    await this.save({ themeName: theme.name, customThemeJson: null });
  }

  /** Editing or importing a theme: applies it and remembers the exact JSON,
   * since it no longer matches any built-in preset. */
  async applyCustom(theme: Theme): Promise<void> {
    themeStore.set(theme);
    await this.save({ themeName: null, customThemeJson: JSON.stringify(theme) });
  }
}

export const settingsStore = new SettingsStore();
