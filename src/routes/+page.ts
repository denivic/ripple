import { redirect } from "@sveltejs/kit";
import { commands, type StartupRouteDto } from "$lib/ipc";

const ROUTES: Record<StartupRouteDto, string> = {
  today: "/today",
  timeline: "/timeline",
  ledger: "/ledger",
  profile: "/profile",
  settings: "/settings",
};

// The front page is the `startupRoute` setting; Today is the fallback if
// settings can't be read (e.g. the very first launch, before Phase 3's
// migrations have ever run — shouldn't happen in practice, but redirecting
// nowhere would strand the user on a blank page).
export async function load() {
  let startupRoute: StartupRouteDto = "today";
  try {
    startupRoute = (await commands.getSettings()).startupRoute;
  } catch {
    // Fall back to Today below.
  }
  redirect(307, ROUTES[startupRoute]);
}
