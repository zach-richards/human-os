import { GlobalStyles } from "@mui/material";
import { useEffect, useState } from "react";
import { listen } from "@tauri-apps/api/event";

import Dashboard from "./Dashboard";
import { RGB } from "./color";

function App() {
  const [graphColor, setGraphColor] = useState<RGB | null>(null);

  useEffect(() => {
    let unlistenFn: (() => void) | null = null;
    type RGB = { r: number; g: number; b: number };
    listen<RGB>("graph-color", (event) => {
      setGraphColor(event.payload);
    });
  }, []);

  return (
    <>
      <GlobalStyles
        styles={{
          html: { margin: 0, padding: 0, height: "100%" },
          body: {
            margin: 0,
            padding: 0,
            height: "100%",
            backgroundColor: "#18191A",
            overflow: "hidden",
          },
          "#root": {
            height: "100vh",
            width: "100vw",
            backgroundColor: "#18191A",
            color: "white",
          },
        }}
      />

      <main style={{ height: "100%" }}>
        <Dashboard
          color={graphColor ?? { r: 255, g: 255, b: 255 }}
        />
      </main>
    </>
  );
}

export default App;