import React from 'react';
import { invoke } from '@tauri-apps/api/core';
import { FileInput } from '../shared/FileInput';

interface SymmetricTabProps {
  inputPath: string;
  setInputPath: React.Dispatch<React.SetStateAction<string>>;
  outputPath: string;
  setOutputPath: React.Dispatch<React.SetStateAction<string>>;
  setResultMessage: (msg: string) => void;
  getOutputPath: (path: string, suffix: string) => string;
}

export const SymmetricTab: React.FC<SymmetricTabProps> = ({
  inputPath,
  setInputPath,
  outputPath,
  setOutputPath,
  setResultMessage,
  getOutputPath,
}) => {
  const handleSymmetricEncrypt = async () => {
    if (!inputPath) return setResultMessage('Choose input file.');
    const outPath = getOutputPath(inputPath, 'sym_enc');
    setResultMessage('Symmetric encryption in progress...');
    try {
      const res = await invoke<string>('symmetric_encrypt', { inputPath, outputPath: outPath });
      setOutputPath(outPath);
      setResultMessage(res);
    } catch (err: any) {
      setResultMessage(`ERROR: ${err}`);
    }
  };

  const handleSymmetricDecrypt = async () => {
    if (!inputPath) return setResultMessage('Choose input file.');
    const outPath = getOutputPath(inputPath, 'sym_dec_out');
    setResultMessage('Symmetric decryption in progress...');
    try {
      const res = await invoke<string>('symmetric_decrypt', { inputPath, outputPath: outPath });
      setOutputPath(outPath);
      setResultMessage(res);
    } catch (err: any) {
      setResultMessage(`ERROR: ${err}`);
    }
  };

  return (
    <div className="space-y-6">
      <h2 className="text-xl font-semibold text-gray-800">Symmetric Encryption (AES-256 GCM)</h2>
      <FileInput label="Input File for Encryption/Decryption" path={inputPath} setter={setInputPath} setResultMessage={setResultMessage} />
      <div className="flex space-x-4">
        <button
          onClick={handleSymmetricEncrypt}
          className="flex-1 py-3 text-lg font-bold text-white bg-blue-500 rounded-xl shadow-lg hover:bg-blue-600"
        >
          ENCRYPT (AES)
        </button>
        <button
          onClick={handleSymmetricDecrypt}
          className="flex-1 py-3 text-lg font-bold text-blue-800 bg-blue-200 rounded-xl shadow-lg hover:bg-blue-300"
        >
          DECRYPT (AES)
        </button>
      </div>
      {outputPath && (
        <p className="text-sm text-gray-500 mt-2">
          Output file: <span className="font-mono text-gray-700">{outputPath}</span>
        </p>
      )}
    </div>
  );
};
