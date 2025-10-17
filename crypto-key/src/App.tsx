import React, { useEffect, useState } from "react";
import { listen } from "@tauri-apps/api/event";
import { LoggerView } from "./components/views/react/LoggerView";
import { AppView } from "./components/views/react/AppView";
import Snowfall from 'react-snowfall';
import { SnowfallCustom } from "./components/shared/Snowfall";

const App: React.FC = () => {
  const [activeView, setActiveView] = useState<"app" | "logger">("app");

  useEffect(() => {
    const unlistenApp = listen("menu-open-app", () => setActiveView("app"));
    const unlistenLogger = listen("menu-open-logger", () => setActiveView("logger"));

    return () => {
      unlistenApp.then(f => f());
      unlistenLogger.then(f => f());
    };
  }, []);

  return (
    <div style={{ position: 'relative', width: '100%' }}>
      <Snowfall
        snowflakeCount={300}
        color="#61a0ec"
        style={{ position: 'fixed', top: 0, left: 0, width: '100vw', height: '100vh', pointerEvents: 'none', zIndex: 1000000 }}
      />
      <SnowfallCustom />

      {activeView === "app" ? (
        <div className="app-container" style={{ position: 'relative', zIndex: 10 }}>
          <AppView />
        </div>
      ) : (
        <div className="logger-container" style={{ position: 'relative', zIndex: 10 }}>
          <LoggerView />
        </div>
      )}
    </div>
  );
};

export default App;
