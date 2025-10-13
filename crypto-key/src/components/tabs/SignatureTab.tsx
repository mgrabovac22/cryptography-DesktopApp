import React from 'react';
import { invoke } from '@tauri-apps/api/core';
import { FileInput } from '../shared/FileInput';

interface SignatureTabProps {
  inputPath: string;
  setInputPath: React.Dispatch<React.SetStateAction<string>>;
  signaturePath: string;
  setSignaturePath: React.Dispatch<React.SetStateAction<string>>;
  setResultMessage: (msg: string) => void;
}

export const SignatureTab: React.FC<SignatureTabProps> = ({
  inputPath,
  setInputPath,
  signaturePath,
  setSignaturePath,
  setResultMessage,
}) => {
  const handleDigest = async () => {
    if (!inputPath) return setResultMessage('Odaberite datoteku za digest.');
    setResultMessage('Izračunavanje digest-a...');
    try {
      const res = await invoke<string>('calculate_digest_and_save', { inputPath });
      setResultMessage(res);
    } catch (err: any) {
      setResultMessage(`GREŠKA: ${err}`);
    }
  };

  const handleSign = async () => {
    if (!inputPath) return setResultMessage('Odaberite datoteku za potpisivanje.');
    setResultMessage('Digitalno potpisivanje...');
    try {
      const res = await invoke<string>('digitally_sign', { filePath: inputPath });
      setResultMessage(res);
    } catch (err: any) {
      setResultMessage(`GREŠKA: ${err}`);
    }
  };

  const handleVerify = async () => {
    if (!inputPath || !signaturePath) return setResultMessage('Odaberite datoteku i datoteku potpisa.');
    setResultMessage('Verifikacija u tijeku...');
    try {
      const isValid = await invoke<boolean>('verify_signature', { filePath: inputPath, signaturePath });
      setResultMessage(isValid ? 'VERIFIKACIJA USPIJELA: Potpis je validan!' : 'VERIFIKACIJA PALA: Potpis je nevažeći.');
    } catch (err: any) {
      setResultMessage(`GREŠKA: ${err}`);
    }
  };

  return (
    <div className="space-y-6">
      <h2 className="text-xl font-semibold text-gray-800">Digitalni Potpis (RSA PKCS#1 v1.5)</h2>
      <FileInput label="Datoteka za Potpis/Verifikaciju" path={inputPath} setter={setInputPath} setResultMessage={setResultMessage} />
      <FileInput label="Datoteka Digitalnog Potpisa (.txt)" path={signaturePath} setter={setSignaturePath} setResultMessage={setResultMessage} />

      <div className="grid grid-cols-3 gap-4">
        <button onClick={handleDigest} className="py-3 text-md font-bold text-white bg-gray-500 rounded-xl shadow-lg hover:bg-gray-600">
          Izračunaj Digest (SHA256)
        </button>
        <button onClick={handleSign} className="py-3 text-md font-bold text-white bg-red-500 rounded-xl shadow-lg hover:bg-red-600">
          POTPIŠI (.txt)
        </button>
        <button onClick={handleVerify} className="py-3 text-md font-bold text-red-800 bg-red-200 rounded-xl shadow-lg hover:bg-red-300">
          VERIFICIRAJ
        </button>
      </div>
    </div>
  );
};
