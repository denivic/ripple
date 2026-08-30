import type { Theme } from "../schema";

export const ember: Theme = {
  schemaVersion: 1,
  name: "Ember",
  surface: { base: "#0D0B09", sidebarFrom: "#0C0A08", sidebarTo: "#241A0F", raised: "#151210" },
  text: { primary: "#FFFFFF", secondary: "#A39B92", tertiary: "#66605A" },
  accent: { base: "#F59E42", muted: "#5A3512" },
  line: { hairline: "#201C18" },
  // Same validated categorical order as Deep Current — re-validated against
  // this theme's own surface color rather than assumed to still pass.
  data: {
    series: ["#3987E5", "#D95926", "#199E70", "#C98500", "#D55181", "#008300", "#9085E9", "#E66767"],
  },
  material: { blur: "20px", saturate: "180%", alpha: 0.6 },
  motion: { damping: 1.0, response: 0.35 },
};
