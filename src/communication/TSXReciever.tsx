// TSXReciever.tsx

// Receives focus score data points from the Rust backend and stores the last 60.
// Exposes a subscribe function so React components can re-render on new data.

import { listen } from "@tauri-apps/api/event";

const MAX_SCORES = 60;

export let scores: number[] = [];

type Listener = () => void;
const listeners = new Set<Listener>();

export function subscribeToScores(fn: Listener): () => void {
  listeners.add(fn);
  return () => listeners.delete(fn);
}

listen("focus-update", (event) => {
  const score = event.payload as number;

  if (scores.length >= MAX_SCORES) {
    scores.shift(); // remove oldest, not newest
  }

  scores.push(score);

  // notify all subscribers so components re-render immediately
  listeners.forEach((fn) => fn());
});