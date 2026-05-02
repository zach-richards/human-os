// sampling.tsx

// Returns the last n scores from the buffer, evenly sampled if there are more than n.

export function getSampled(scores: number[], n: number): number[] | null {
  if (scores.length === 0) return null;

  // Not enough data yet — return what we have, left-padded with nulls for spacing
  if (scores.length < n) return null;

  // Take the most recent n points evenly spaced from the tail
  const step = scores.length / n;

  return Array.from({ length: n }, (_, i) => {
    const idx = Math.min(Math.round(i * step), scores.length - 1);
    return scores[idx];
  });
}