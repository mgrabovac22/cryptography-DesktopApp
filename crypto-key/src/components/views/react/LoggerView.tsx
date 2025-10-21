import React, { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

interface LogEntry { timestamp: string; level: string; event: string; details: { [key: string]: string }; }

const formatDetails = (d: { [k: string]: string }): string => 
    Object.keys(d).length ? Object.entries(d).map(([k, v]) => `${k}:${v}`).join(' | ') : 'N/A';
const getColor = (level: string) => 
    level.includes('ERROR') ? '#F87171' : level.includes('WARN') ? '#FBBF24' : '#4ADE80';

export const LoggerView: React.FC = () => {
    const [logs, setLogs] = useState<LogEntry[] | string>('loading');
    const [rawLogs, setRawLogs] = useState<string>('');
    const [viewRaw, setViewRaw] = useState(false);

    useEffect(() => {
        const fetchLogs = async () => {
            try {
                const [structured, raw] = await Promise.all([
                    invoke<LogEntry[]>("get_formatted_logs_command"),
                    invoke<string>("read_logs_command")
                ]);
                const validLogs = structured.filter(log => log.timestamp !== 'N/A');
                setLogs(validLogs.length > 0 ? validLogs : 'empty');
                setRawLogs(raw || 'No raw logs.');
            } catch (e) {
                setLogs('error');
                setRawLogs(`❌ Error: ${e}`);
            }
        };
        fetchLogs();
    }, []);

    if (logs === 'loading') return <div className="text-center py-8 font-mono border border-gray-700 m-4">... Loading ...</div>;
    if (logs === 'error') return <div className="text-red-500 p-4 font-mono border border-red-500 m-4">{rawLogs}</div>;
    if (logs === 'empty' && !viewRaw) return <div className="text-gray-500 text-center py-8 font-mono border border-gray-700 m-4">📭 No relevant logs.</div>;

    return (
        <div className="p-4 border border-gray-700 m-4 rounded-lg">
            <h2 className="text-xl font-bold mb-4 text-white">Log Data</h2>

            <div className="flex justify-end mb-4 text-white">
                <label className="flex items-center space-x-2 cursor-pointer">
                    <input type="checkbox" checked={viewRaw} onChange={(e) => setViewRaw(e.target.checked)} className="h-5 w-5 rounded" />
                    <span>See raw data</span>
                </label>
            </div>

            {viewRaw ? (
                <pre className="bg-gray-900 text-green-400 p-4 rounded-lg overflow-auto h-[60vh] text-sm whitespace-pre-wrap border border-gray-600">
                    {rawLogs}
                </pre>
            ) : (
                <div className="bg-gray-900 rounded-lg overflow-x-auto h-[60vh] border border-gray-600">
                    <table border={1} className="min-w-full font-mono text-sm text-white divide-y divide-gray-700">
                        <thead className="sticky top-0 bg-gray-800 text-gray-400">
                            <tr>
                                <th className="px-4 py-3 text-left">Time</th>
                                <th className="px-4 py-3 text-left">Level</th>
                                <th className="px-4 py-3 text-left">Event</th>
                                <th className="px-4 py-3 text-left">Details</th>
                            </tr>
                        </thead>
                        <tbody>
                            {(logs as LogEntry[]).map((log, i) => (
                                <tr key={i} className={log.level.includes('ERROR') ? 'bg-red-900/40' : 'hover:bg-gray-700'}>
                                    <td className="px-4 py-2 text-gray-400">{log.timestamp}</td>
                                    <td className="px-4 py-2 font-bold" style={{ color: getColor(log.level) }}>{log.level}</td>
                                    <td className="px-4 py-2">{log.event}</td>
                                    <td className="px-4 py-2 text-gray-500 italic">{formatDetails(log.details)}</td>
                                </tr>
                            ))}
                        </tbody>
                    </table>
                </div>
            )}
        </div>
    );
};