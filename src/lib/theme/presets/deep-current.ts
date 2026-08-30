import type { Theme } from "../schema";

export const deepCurrent: Theme = {
  schemaVersion: 1,
  name: "Deep Current",
  surface: { base: "#0B0D0C", sidebarFrom: "#0A0C0B", sidebarTo: "#16241F", raised: "#121514" },
  text: { primary: "#FFFFFF", secondary: "#9BA3A0", tertiary: "#5E6664" },
  accent: { base: "#2DD4BF", muted: "#134E4A" },
  line: { hairline: "#1C201F" },
  // Documented, validated categorical order (dataviz skill's palette.md), not
  // hand-picked — passes the lightness band, chroma floor, CVD separation
  // and contrast checks against this theme's surface color.
  data: {
    series: ["#3987E5", "#D95926", "#199E70", "#C98500", "#D55181", "#008300", "#9085E9", "#E66767"],
  },
  material: { blur: "20px", saturate: "180%", alpha: 0.6 },
  motion: { damping: 1.0, response: 0.35 },
};
