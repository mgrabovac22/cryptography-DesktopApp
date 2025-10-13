import React from 'react';
import { invoke } from '@tauri-apps/api/core';

interface KeysTabProps {
  setResultMessage: (msg: string) => void;
}

export const KeysTab: React.FC<KeysTabProps> = ({ setResultMessage }) => {
  const generateKeys = async () => {
    setResultMessage('Generiranje ključeva...');
    try {
      const res = await invoke<string>('generate_keys');
      setResultMessage(res);
    } catch (err: any) {
      setResultMessage(`GREŠKA: ${err}`);
    }
  };

  return (
    <div className="space-y-6">
      <h2 className="text-xl font-semibold text-gray-800">Generiranje Ključeva</h2>
      <p className="text-gray-600">
        Generiraj novi set simetričnih (AES) i asimetričnih (RSA) ključeva i spremi ih u mapu aplikacije.
      </p>
      <button
        onClick={generateKeys}
        className="w-full py-3 text-lg font-bold text-white bg-green-500 rounded-xl shadow-lg hover:bg-green-600 transition duration-150"
      >
        Generiraj i Spremi Sve Ključeve
      </button>
    </div>
  );
};
