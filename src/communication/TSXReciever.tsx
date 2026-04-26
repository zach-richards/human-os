// TXSReciever.tsx

// A reciever module for data poins of the focus over time sent from the Rust backend.

import { listen } from "@tauri-apps/api/event";

export let scores: number[] = [];

listen("focus-update", (event) => {
  console.log("Focus score:", event.payload);

  if (scores.length >= 60) {
    scores.pop();
  }

  scores.push(event.payload as number);
});