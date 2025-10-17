import React, { useEffect, useState } from "react";
import { listen } from "@tauri-apps/api/event";
import { LoggerView } from "./components/views/react/LoggerView";
import { AppView } from "./components/views/react/AppView";
import { AboutApp } from "./components/views/react/AboutAppView";
import { DeveloperInfo } from "./components/views/react/DeveloperInfoView";
import Snowfall from "react-snowfall";
import { SnowfallCustom } from "./components/shared/Snowfall";

const App: React.FC = () => {
  const [activeView, setActiveView] = useState<"app" | "logger">("app");

  const [snowEnabled, setSnowEnabled] = useState(true);
  const [snowIntensity, setSnowIntensity] = useState(300);
  const [showAbout, setShowAbout] = useState(false);
  const [showDeveloper, setShowDeveloper] = useState(false);

  useEffect(() => {
    const unlistenApp = listen("menu-open-app", () => setActiveView("app"));
    const unlistenLogger = listen("menu-open-logger", () => setActiveView("logger"));
    const unlistenAbout = listen("menu-open-about", () => setShowAbout(true));
    const unlistenDev = listen("menu-open-developer", () => setShowDeveloper(true));

    const unlistenIncrease = listen("snow-increase", () => {
      setSnowIntensity((prev) => Math.min(prev + 100, 10000));
    });

    const unlistenDecrease = listen("snow-decrease", () => {
      setSnowIntensity((prev) => Math.max(prev - 100, 0));
    });

    const unlistenToggle = listen("snow-toggle", () => {
      setSnowEnabled((prev) => !prev);
    });

    return () => {
      unlistenApp.then((f) => f());
      unlistenLogger.then((f) => f());
      unlistenIncrease.then((f) => f());
      unlistenDecrease.then((f) => f());
      unlistenToggle.then((f) => f());
      unlistenAbout.then(f => f());
      unlistenDev.then(f => f());
    };
  }, []);

  return (
    <div style={{ position: "relative", width: "100%" }}>
      {snowEnabled && (
        <Snowfall
          snowflakeCount={snowIntensity}
          color="#61a0ec"
          style={{
            position: "fixed",
            top: 0,
            left: 0,
            width: "100vw",
            height: "100vh",
            pointerEvents: "none",
            zIndex: 1000000,
          }}
        />
      )}

      <SnowfallCustom />

      {activeView === "app" ? (
        <div className="app-container" style={{ position: "relative", zIndex: 10 }}>
          <AppView />
        </div>
      ) : (
        <div className="logger-container" style={{ position: "relative", zIndex: 10 }}>
          <LoggerView />
        </div>
      )}

      {showAbout && <AboutApp onClose={() => setShowAbout(false)} />}
      {showDeveloper && <DeveloperInfo onClose={() => setShowDeveloper(false)} />}
    </div>
  );
};

export default App;
