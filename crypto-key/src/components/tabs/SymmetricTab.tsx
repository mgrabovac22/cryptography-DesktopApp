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
    if (!inputPath) return setResultMessage('Odaberite ulaznu datoteku.');
    const outPath = getOutputPath(inputPath, 'sym_enc');
    setResultMessage('Simetrična enkripcija u tijeku...');
    try {
      const res = await invoke<string>('symmetric_encrypt', { inputPath, outputPath: outPath });
      setOutputPath(outPath);
      setResultMessage(res);
    } catch (err: any) {
      setResultMessage(`GREŠKA: ${err}`);
    }
  };

  const handleSymmetricDecrypt = async () => {
    if (!inputPath) return setResultMessage('Odaberite ulaznu datoteku.');
    const outPath = getOutputPath(inputPath, 'sym_dec_out');
    setResultMessage('Simetrična dekripcija u tijeku...');
    try {
      const res = await invoke<string>('symmetric_decrypt', { inputPath, outputPath: outPath });
      setOutputPath(outPath);
      setResultMessage(res);
    } catch (err: any) {
      setResultMessage(`GREŠKA: ${err}`);
    }
  };

  return (
    <div className="space-y-6">
      <h2 className="text-xl font-semibold text-gray-800">Simetrična Enkripcija (AES-256 GCM)</h2>
      <FileInput label="Ulazna Datoteka za Enkripciju/Dekripciju" path={inputPath} setter={setInputPath} setResultMessage={setResultMessage} />
      <div className="flex space-x-4">
        <button
          onClick={handleSymmetricEncrypt}
          className="flex-1 py-3 text-lg font-bold text-white bg-blue-500 rounded-xl shadow-lg hover:bg-blue-600"
        >
          ENKRIPTIRAJ (AES)
        </button>
        <button
          onClick={handleSymmetricDecrypt}
          className="flex-1 py-3 text-lg font-bold text-blue-800 bg-blue-200 rounded-xl shadow-lg hover:bg-blue-300"
        >
          DEKRIPTIRAJ (AES)
        </button>
      </div>
      {outputPath && (
        <p className="text-sm text-gray-500 mt-2">
          Izlazna datoteka: <span className="font-mono text-gray-700">{outputPath}</span>
        </p>
      )}
    </div>
  );
};
