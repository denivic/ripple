import type { Theme } from "../schema";

export const deepCurrent: Theme = {
  schemaVersion: 1,
  name: "Deep Current",
  surface: { base: "#0B0D0C", sidebarFrom: "#0A0C0B", sidebarTo: "#16241F", raised: "#121514" },
  text: { primary: "#FFFFFF", secondary: "#9BA3A0", tertiary: "#5E6664" },
  accent: { base: "#2DD4BF", muted: "#134E4A" },
  line: { hairline: "#1C201F" },
  data: { series: ["#2DD4BF", "#5EEAD4", "#0EA5A5", "#7DD3FC", "#A78BFA"] },
  material: { blur: "20px", saturate: "180%", alpha: 0.6 },
  motion: { damping: 1.0, response: 0.35 },
};
