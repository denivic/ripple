import { describe, expect, it } from "vitest";
import { toSpringOptions } from "./motion";

describe("toSpringOptions", () => {
  it("lands close to svelte/motion's own defaults for our default theme motion", () => {
    const { stiffness, damping } = toSpringOptions({ damping: 1, response: 0.35 });
    expect(stiffness).toBeCloseTo(0.1429, 3);
    expect(damping).toBeCloseTo(0.8, 5);
  });

  it("a lower response yields higher stiffness (snappier)", () => {
    const slow = toSpringOptions({ damping: 1, response: 0.6 });
    const fast = toSpringOptions({ damping: 1, response: 0.2 });
    expect(fast.stiffness).toBeGreaterThan(slow.stiffness);
  });

  it("clamps stiffness and damping to [0.01, 1]", () => {
    const extreme = toSpringOptions({ damping: 5, response: 0.001 });
    expect(extreme.stiffness).toBeLessThanOrEqual(1);
    expect(extreme.damping).toBeLessThanOrEqual(1);
  });
});
