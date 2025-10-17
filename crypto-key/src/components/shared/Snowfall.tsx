import React, { useEffect, useState } from "react";

export const SnowfallCustom: React.FC = () => {
  const [flakes, setFlakes] = useState<React.ReactElement[]>([]);

  useEffect(() => {
    const flakeCount = 50;
    const newFlakes = Array.from({ length: flakeCount }).map((_, i) => {
      const style = {
        left: `${Math.random() * 100}%`,
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
  }, []);

  return <div className="snowfall">{flakes}</div>;
};
