import React from 'react';
import { invoke } from '@tauri-apps/api/core';

interface KeysTabProps {
  setResultMessage: (msg: string) => void;
}

export const KeysTab: React.FC<KeysTabProps> = ({ setResultMessage }) => {
  const generateKeys = async () => {
    setResultMessage('Generating keys...');
    try {
      const res = await invoke<string>('generate_keys');
      setResultMessage(res);
    } catch (err: any) {
      setResultMessage(`ERROR: ${err}`);
    }
  };

  return (
    <div className="space-y-6">
      <h2 className="text-xl font-semibold text-gray-800">Generating Keys</h2>
      <p className="text-gray-600">
        Generate a new set of symmetric (AES) and asymmetric (RSA) keys and save them in the application directory.
      </p>
      <button
        onClick={generateKeys}
        className="w-full py-3 text-lg font-bold text-white bg-green-500 rounded-xl shadow-lg hover:bg-green-600 transition duration-150"
      >
        Generate and Save All Keys
      </button>
    </div>
  );
};
