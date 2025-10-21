import React from 'react';
import { open } from '@tauri-apps/plugin-dialog';

interface FileInputProps {
  label: string;
  path: string;
  setter: React.Dispatch<React.SetStateAction<string>>;
  setResultMessage: (msg: string) => void;
}

export const FileInput: React.FC<FileInputProps> = ({ label, path, setter, setResultMessage }) => {
  const selectFile = async () => {
    try {
      const selected = await open({ multiple: false });
      if (selected) {
        setter(Array.isArray(selected) ? selected[0] : selected);
      }
    } catch (err) {
      setResultMessage(`ERROR: ${err}`);
    }
  };

  return (
    <div className="flex flex-col space-y-2">
      <label className="text-sm font-medium text-gray-700">{label}</label>
      <div className="flex space-x-2">
        <input
          type="text"
          readOnly
          value={path || 'Not selected...'}
          className="p-2 border border-gray-300 bg-gray-50 rounded-lg focus:ring-2 focus:ring-indigo-500" />
        <button
          onClick={selectFile}
          className="px-4 py-2 text-sm font-semibold text-white bg-indigo-500 rounded-lg shadow-md hover:bg-indigo-600 transition duration-150"
        >
          Browse
        </button>
      </div>
    </div>
  );
};
