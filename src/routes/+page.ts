import { redirect } from "@sveltejs/kit";

// The front page is a `startupRoute` setting (Phase 7); Today is the
// default until that setting exists.
export function load() {
  redirect(307, "/today");
}
