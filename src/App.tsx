import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { GlobalStyles } from "@mui/material";

import Dashboard from "./Dashboard";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");

  async function greet() {
    setGreetMsg(await invoke("greet", { name }));
  }

  return (
    <>
      {/* 🎯 Global OS-style root styling */}
      <GlobalStyles
        styles={{
          html: {
            margin: 0,
            padding: 0,
            height: "100%",
          },

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
        <Dashboard />
      </main>
    </>
  );
}

export default App;