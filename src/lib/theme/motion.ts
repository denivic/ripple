export interface SpringOptions {
  stiffness: number;
  damping: number;
}

/**
 * Converts our theme tokens' Apple-style (damping ratio, response-seconds)
 * pair into svelte/motion's normalized (stiffness, damping ∈ [0,1]) spring
 * model. Not a closed-form physical equivalence — svelte/motion doesn't
 * publish one — but it preserves intent (lower response snaps faster,
 * damping < 1 keeps some bounce) and lands on svelte/motion's own defaults
 * (stiffness 0.15, damping 0.8) for our default theme's damping:1, response:0.35.
 */
export function toSpringOptions(motion: { damping: number; response: number }): SpringOptions {
  const stiffness = Math.min(1, Math.max(0.01, 1 / (motion.response * 20)));
  const damping = Math.min(1, Math.max(0.01, motion.damping * 0.8));
  return { stiffness, damping };
}

/** Momentum-carrying gestures (flicks, throws, drag release) get a touch of
 * bounce; apple-design's own table puts this at damping ≈ 0.8. Everything
 * else should stay critically damped (damping: 1) — see toSpringOptions. */
export const BOUNCE_DAMPING = 0.8;
