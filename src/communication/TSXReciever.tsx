import { listen } from "@tauri-apps/api/event";

export let scores: number[] = [];

listen("focus-update", (event) => {
  console.log("Focus score:", event.payload);

  if (scores.length >= 60) {
    scores.pop();
  }

  scores.push(event.payload as number);
});