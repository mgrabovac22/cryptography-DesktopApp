import React, { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "../css/LoggerView.css";

export const LoggerView: React.FC = () => {
  const [logs, setLogs] = useState<string>("Učitavam logove...");

  useEffect(() => {
    const fetchLogs = async () => {
      try {
        const result = await invoke<string>("read_logs_command");
        setLogs(result || "📭 Nema logova za prikaz.");
      } catch (error) {
        setLogs(`❌ Greška pri učitavanju logova: ${error}`);
      }
    };
    fetchLogs();
  }, []);

  return (
    <div className="logger-view">
      <h2 className="text-xl font-bold mb-4">Log data</h2>
      <pre className="bg-gray-900 text-green-400 p-4 rounded-xl overflow-auto h-[60vh] text-xl">
        {logs}
      </pre>
    </div>
  );
};
