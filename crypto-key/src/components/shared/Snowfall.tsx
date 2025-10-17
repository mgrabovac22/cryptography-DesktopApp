import React, { useEffect, useState, useRef } from "react";
import { listen } from "@tauri-apps/api/event";

export const SnowfallCustom: React.FC = () => {
  const [flakeCount, setFlakeCount] = useState(50);
  const [flakes, setFlakes] = useState<React.ReactElement[]>([]);
  const [enabled, setEnabled] = useState(true);
  const firstRender = useRef(true);

  useEffect(() => {
    const unlistenIncrease = listen("snow-increase", () => {
      setFlakeCount((prev) => Math.min(prev + 10, 100));
    });

    const unlistenDecrease = listen("snow-decrease", () => {
      setFlakeCount((prev) => Math.max(prev - 10, 0));
    });

    const unlistenToggle = listen("snow-toggle", () => {
      setEnabled((prev) => !prev);
    });

    return () => {
      unlistenIncrease.then((f) => f());
      unlistenDecrease.then((f) => f());
      unlistenToggle.then((f) => f());
    };
  }, []);

  useEffect(() => {
    if (!enabled) {
      setFlakes([]);
      return;
    }

    const newFlakes = Array.from({ length: flakeCount }).map((_, i) => {
      const style: React.CSSProperties = {
        left: `${Math.random() * 100}%`,
        top: firstRender.current ? `${Math.random() * 100}%` : `-${Math.random() * 20}%`,
        animationDuration: `${5 + Math.random() * 10}s`,
        animationDelay: `${Math.random() * 5}s`,
        fontSize: `${Math.random() * 1.2 + 0.6}em`,
      };
      return (
        <span key={i} className="snowflake" style={style}>
          ❄
        </span>
      );
    });

    setFlakes(newFlakes);
    firstRender.current = false;
  }, [flakeCount, enabled]);

  if (!enabled) return null;

  return (
    <div
      className="snowfall"
      style={{
        position: "fixed",
        top: 0,
        left: 0,
        width: "100vw",
        height: "100vh",
        pointerEvents: "none",
        zIndex: 9999,
        overflow: "hidden",
      }}
    >
      {flakes}
    </div>
  );
};
