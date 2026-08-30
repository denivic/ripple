import type { Theme } from "../schema";

export const ember: Theme = {
  schemaVersion: 1,
  name: "Ember",
  surface: { base: "#0D0B09", sidebarFrom: "#0C0A08", sidebarTo: "#241A0F", raised: "#151210" },
  text: { primary: "#FFFFFF", secondary: "#A39B92", tertiary: "#66605A" },
  accent: { base: "#F59E42", muted: "#5A3512" },
  line: { hairline: "#201C18" },
  data: { series: ["#F59E42", "#FBBF77", "#EA8B1F", "#F87171", "#C084FC"] },
  material: { blur: "20px", saturate: "180%", alpha: 0.6 },
  motion: { damping: 1.0, response: 0.35 },
};
