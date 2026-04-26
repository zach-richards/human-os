// App.tsx

// The instance of the app itself. Contains dashboard.

import { GlobalStyles } from "@mui/material";

import Dashboard from "./Dashboard";

function App() {
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
        <Dashboard/>
      </main>
    </>
  );
}

export default App;