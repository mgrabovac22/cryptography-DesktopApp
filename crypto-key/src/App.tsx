import React, { useEffect, useState } from "react";
import { listen } from "@tauri-apps/api/event";
import { Snowfall } from "./components/shared/Snowfall";
import { LoggerView } from "./components/views/react/LoggerView";
import { AppView } from "./components/views/react/AppView";

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
    <>
      <Snowfall />
      {activeView === "app" ? <div className="app-container"><AppView /></div> : <div className="logger-container"><LoggerView /></div>}
    </>
  );
};

export default App;
