/**
 * Rounded-rect bar paths: 4px radius at the data end, square at the
 * baseline (mark spec). Both assume non-negative values — every quantity
 * this app charts (time, money, occurrences) is >= 0, so there's no
 * negative-bar case to handle.
 */

export function verticalBarPath(x: number, top: number, width: number, height: number, radius = 4): string {
  if (height <= 0 || width <= 0) return "";
  const r = Math.min(radius, width / 2, height);
  const bottom = top + height;
  return `M${x},${bottom} V${top + r} A${r},${r} 0 0 1 ${x + r},${top} H${x + width - r} A${r},${r} 0 0 1 ${x + width},${top + r} V${bottom} Z`;
}

export function horizontalBarPath(x: number, y: number, width: number, height: number, radius = 4): string {
  if (height <= 0 || width <= 0) return "";
  const r = Math.min(radius, height / 2, width);
  return `M${x},${y} H${x + width - r} A${r},${r} 0 0 1 ${x + width},${y + r} V${y + height - r} A${r},${r} 0 0 1 ${x + width - r},${y + height} H${x} Z`;
}
