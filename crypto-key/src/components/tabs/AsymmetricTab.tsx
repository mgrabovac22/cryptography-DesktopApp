import React from 'react';
import { invoke } from '@tauri-apps/api/core';
import { FileInput } from '../shared/FileInput';

interface AsymmetricTabProps {
  inputPath: string;
  setInputPath: React.Dispatch<React.SetStateAction<string>>;
  outputPath: string;
  setOutputPath: React.Dispatch<React.SetStateAction<string>>;
  setResultMessage: (msg: string) => void;
  getOutputPath: (path: string, suffix: string) => string;
}

export const AsymmetricTab: React.FC<AsymmetricTabProps> = ({
  inputPath,
  setInputPath,
  outputPath,
  setOutputPath,
  setResultMessage,
  getOutputPath,
}) => {
  const handleAsymmetricEncrypt = async () => {
    if (!inputPath) return setResultMessage('Odaberite ulaznu datoteku.');
    const outPath = getOutputPath(inputPath, 'asym_enc');
    setResultMessage('Asimetrična enkripcija u tijeku...');
    try {
      const res = await invoke<string>('asymmetric_encrypt', { inputPath, outputPath: outPath });
      setOutputPath(outPath);
      setResultMessage(res);
    } catch (err: any) {
      setResultMessage(`GREŠKA: ${err}`);
    }
  };

  const handleAsymmetricDecrypt = async () => {
    if (!inputPath) return setResultMessage('Odaberite ulaznu datoteku.');
    const outPath = getOutputPath(inputPath, 'asym_dec_out');
    setResultMessage('Asimetrična dekripcija u tijeku...');
    try {
      const res = await invoke<string>('asymmetric_decrypt', { inputPath, outputPath: outPath });
      setOutputPath(outPath);
      setResultMessage(res);
    } catch (err: any) {
      setResultMessage(`GREŠKA: ${err}`);
    }
  };

  return (
    <div className="space-y-6">
      <h2 className="text-xl font-semibold text-gray-800">Asimetrična Enkripcija (RSA OAEP)</h2>
      <p className="text-sm text-yellow-700 bg-yellow-100 p-3 rounded-lg border border-yellow-300">
        <span className="font-bold">Upozorenje:</span> RSA je primarno za male datoteke i razmjenu ključeva.
      </p>
      <FileInput label="Ulazna Datoteka" path={inputPath} setter={setInputPath} setResultMessage={setResultMessage} />
      <div className="flex space-x-4">
        <button
          onClick={handleAsymmetricEncrypt}
          className="flex-1 py-3 text-lg font-bold text-white bg-purple-500 rounded-xl shadow-lg hover:bg-purple-600"
        >
          ENKRIPTIRAJ (RSA)
        </button>
        <button
          onClick={handleAsymmetricDecrypt}
          className="flex-1 py-3 text-lg font-bold text-purple-800 bg-purple-200 rounded-xl shadow-lg hover:bg-purple-300"
        >
          DEKRIPTIRAJ (RSA)
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
