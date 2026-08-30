import { listen, type UnlistenFn } from "@tauri-apps/api/event";

export type DataChangedScope = "habits" | "entries" | "profile";

export interface DataChangedEvent {
  scope: DataChangedScope;
}

const DATA_CHANGED_EVENT = "ripple://data-changed";

/** The Observer half of the plan's real-time loop: Rust emits after every
 * write, this subscribes once per store, and stores patch their own state —
 * see src-tauri/src/interface/commands.rs's notify_changed. */
export function onDataChanged(handler: (event: DataChangedEvent) => void): Promise<UnlistenFn> {
  return listen<DataChangedEvent>(DATA_CHANGED_EVENT, (e) => handler(e.payload));
}
