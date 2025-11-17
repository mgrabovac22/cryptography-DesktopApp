import React, { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';

interface KeysTabProps {
  setResultMessage: (msg: string) => void;
}

const FolderOpenIcon = () => (
    <svg 
        xmlns="http://www.w3.org/2000/svg" 
        width="24" 
        height="24" 
        viewBox="0 0 24 24" 
        fill="none" 
        stroke="currentColor" 
        strokeWidth="2" 
        strokeLinecap="round" 
        strokeLinejoin="round" 
        className="w-6 h-6 mr-2"
    >
        <path d="M4 20h16a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.93a2 2 0 0 1-1.66-.9l-.82-1.2A2 2 0 0 0 11.93 2H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2z"></path>
    </svg>
);

export const KeysTab: React.FC<KeysTabProps> = ({ setResultMessage }) => {
  const [isModalOpen, setIsModalOpen] = useState(false);
  const [selectedKey, setSelectedKey] = useState<'private' | 'public' | 'secret'>('private');
  const [keyContent, setKeyContent] = useState<string>('');

  const generateKeys = async () => {
    setResultMessage('Generating keys...');
    try {
      const res = await invoke<string>('generate_keys');
      setResultMessage(res);
    } catch (err: any) {
      setResultMessage(`ERROR: ${err}`);
    }
  };

  const openKeyModal = async () => {
    try {
      let content: string;
      if (selectedKey === 'private') content = await invoke<string>('get_private_key');
      else if (selectedKey === 'public') content = await invoke<string>('get_public_key');
      else content = await invoke<string>('get_secret_key');
      setKeyContent(content);
      setIsModalOpen(true);
    } catch (err: any) {
      setResultMessage(`ERROR: ${err}`);
    }
  };

  const openKeysDirectory = async () => {
    setResultMessage('Opening keys directory...');
    try {
      await invoke('open_keys_directory');
      setResultMessage('Keys directory opened successfully.');
    } catch (err: any) {
      setResultMessage(`ERROR: ${err}`);
    }
  };

  return (
    <div className="space-y-6 relative">
      <h2 className="text-xl font-semibold text-gray-800">Keys Management</h2>
      <p className="text-gray-600">
        Generate a new set of symmetric (AES) and asymmetric (RSA) keys and save them in the application directory.
      </p>

      <button
        onClick={generateKeys}
        className="w-full py-3 text-lg font-bold text-white bg-green-500 rounded-xl shadow-lg hover:bg-green-600 transition duration-150"
      >
        Generate and Save All Keys
      </button>

      <div className="flex space-x-4">
          <button
            onClick={openKeyModal}
            className="flex-1 py-3 text-lg font-bold text-white bg-blue-500 rounded-xl shadow-lg hover:bg-blue-600 transition duration-150"
          >
            View Keys
          </button>
          
          <button
            onClick={openKeysDirectory}
            title="Open Keys Directory"
            className="flex items-center justify-center w-20 py-3 text-white bg-indigo-500 rounded-xl shadow-lg hover:bg-indigo-600 transition duration-150"
          >
             <FolderOpenIcon />
          </button>
      </div>

      {isModalOpen && (
        <div className="absolute inset-0 z-50 flex justify-center items-center">
          <div
            className="absolute inset-0 bg-black bg-opacity-30"
            onClick={() => setIsModalOpen(false)}
          />
          <div className="relative bg-white rounded-xl p-6 w-full max-w-md shadow-lg z-10 flex flex-col h-[600px]">
            <h3 className="text-lg font-bold mb-4">Select Key to View</h3>

            <select
              value={selectedKey}
              onChange={async (e) => {
                const key = e.target.value as 'private' | 'public' | 'secret';
                setSelectedKey(key);
                try {
                  let content: string;
                  if (key === 'private') content = await invoke<string>('get_private_key');
                  else if (key === 'public') content = await invoke<string>('get_public_key');
                  else content = await invoke<string>('get_secret_key');
                  setKeyContent(content);
                } catch (err: any) {
                  setResultMessage(`ERROR: ${err}`);
                }
              }}
              className="w-full mb-4 p-2 border rounded"
            >
              <option value="private">Private Key</option>
              <option value="public">Public Key</option>
              <option value="secret">Secret Key</option>
            </select>

            <textarea
              className="w-full p-2 border rounded resize-none mb-4 overflow-auto font-mono text-sm"
              style={{ minHeight: '300px' }}
              value={keyContent}
              readOnly
            />

            <div className="flex justify-end mt-auto pt-4">
              <button
                onClick={() => setIsModalOpen(false)}
                className="px-4 py-2 bg-red-500 text-white rounded shadow hover:bg-red-600"
              >
                Close
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
};
