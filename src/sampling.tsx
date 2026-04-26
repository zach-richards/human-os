import { scores } from "./communication/TSXReciever"

export function getSampled(scores: number[], n: number): number[] | null {
  if (scores.length < n) return null;

  const step = Math.floor(scores.length / n);

  return Array.from({ length: n }, (_, i) => {
    return scores[i * step];
  });
}