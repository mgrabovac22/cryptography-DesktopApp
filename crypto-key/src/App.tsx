import React, { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';
import './App.css';

const App = () => {
  const [resultMessage, setResultMessage] = useState('');
  const [inputPath, setInputPath] = useState('');
  const [outputPath, setOutputPath] = useState('');
  const [signaturePath, setSignaturePath] = useState('');
  const [activeTab, setActiveTab] = useState('keys');

  const handleResult = (res: string | boolean) => {
    if (typeof res === 'boolean') {
      setResultMessage(res ? 'VERIFIKACIJA USPIJELA: Potpis je validan!' : 'VERIFIKACIJA PALA: Potpis je nevažeći.');
    } else {
      setResultMessage(res);
    }
  };

  const handleError = (err: any) => {
    setResultMessage(`GREŠKA: ${err.toString()}`);
  };

  const selectFile = async (setter: React.Dispatch<React.SetStateAction<string>>) => {
    try {
      const selected = await open({ multiple: false });
      if (selected) {
        setter(Array.isArray(selected) ? selected[0] : selected);
      }
    } catch (err) {
      setResultMessage(`GREŠKA: ${err}`);
    }
  };

  const getOutputPath = (originalPath: string, suffix: string) => {
    if (!originalPath) return 'output.bin';
    const parts = originalPath.split('.');
    const base = parts.slice(0, -1).join('.'); 
    return `${base}_${suffix}.bin`;
  };

  const generateKeys = async () => {
    setResultMessage('Generiranje ključeva...');
    try {
      const res = await invoke<string>('generate_keys');
      handleResult(res);
    } catch (err) {
      handleError(err);
    }
  };

  const handleSymmetricEncrypt = async () => {
    if (!inputPath) return handleError('Odaberite ulaznu datoteku.');
    const outPath = getOutputPath(inputPath, 'sym_enc'); 
    setResultMessage('Simetrična enkripcija u tijeku...');
    try {
      console.log('Sending invoke:', { inputPath, outputPath: outPath });
      const res = await invoke<string>('symmetric_encrypt', {
        inputPath: inputPath,
        outputPath: outPath
      });
      setOutputPath(outPath);
      handleResult(res);
    } catch (err) {
      handleError(err);
    }
  };

  const handleSymmetricDecrypt = async () => {
    if (!inputPath) return handleError('Odaberite ulaznu datoteku.');
    const outPath = getOutputPath(inputPath, 'sym_dec_out'); 
    setResultMessage('Simetrična dekripcija u tijeku...');
    try {
      const res = await invoke<string>('symmetric_decrypt', { inputPath, outputPath: outPath });
      setOutputPath(outPath);
      handleResult(res);
    } catch (err) {
      handleError(err);
    }
  };

  const handleAsymmetricEncrypt = async () => {
    if (!inputPath) return handleError('Odaberite ulaznu datoteku.');
    const outPath = getOutputPath(inputPath, 'asym_enc');
    setResultMessage('Asimetrična enkripcija u tijeku...');
    try {
      const res = await invoke<string>('asymmetric_encrypt', { inputPath, outputPath: outPath });
      setOutputPath(outPath);
      handleResult(res);
    } catch (err) {
      handleError(err);
    }
  };

  const handleAsymmetricDecrypt = async () => {
    if (!inputPath) return handleError('Odaberite ulaznu datoteku.');
    const outPath = getOutputPath(inputPath, 'asym_dec_out');
    setResultMessage('Asimetrična dekripcija u tijeku...');
    try {
      const res = await invoke<string>('asymmetric_decrypt', { inputPath, outputPath: outPath });
      setOutputPath(outPath);
      handleResult(res);
    } catch (err) {
      handleError(err);
    }
  };

  const handleSign = async () => {
    if (!inputPath) return handleError('Odaberite ulaznu datoteku za potpisivanje.');
    setResultMessage('Digitalno potpisivanje...');
    try {
      const res = await invoke<string>('digitally_sign', { filePath: inputPath });
      handleResult(res);
    } catch (err) {
      handleError(err);
    }
  };

  const handleVerify = async () => {
    if (!inputPath || !signaturePath) return handleError('Odaberite datoteku i datoteku potpisa.');
    setResultMessage('Verifikacija u tijeku...');
    try {
      const isValid = await invoke<boolean>('verify_signature', { filePath: inputPath, signaturePath });
      handleResult(isValid);
    } catch (err) {
      handleError(err);
    }
  };

  const handleDigest = async () => {
    if (!inputPath) return handleError('Odaberite datoteku za digest.');
    setResultMessage('Izračunavanje digest-a...');
    try {
      const res = await invoke<string>('calculate_digest_and_save', { inputPath });
      handleResult(res);
    } catch (err) {
      handleError(err);
    }
  };

  const TabButton = ({ tabId, children }: { tabId: string; children: React.ReactNode }) => (
    <button
      className={`px-4 py-2 text-sm font-medium transition duration-150 ease-in-out rounded-t-lg focus:outline-none ${
        activeTab === tabId
          ? 'bg-indigo-600 text-white shadow-lg'
          : 'bg-indigo-100 text-indigo-700 hover:bg-indigo-200'
      }`}
      onClick={() => {
        setActiveTab(tabId);
        setResultMessage('');
      }}
    >
      {children}
    </button>
  );

  const FileInput = ({ label, path, setter }: { label: string; path: string; setter: React.Dispatch<React.SetStateAction<string>> }) => (
    <div className="flex flex-col space-y-2">
      <label className="text-sm font-medium text-gray-700">{label}</label>
      <div className="flex space-x-2">
        <input
          type="text"
          readOnly
          value={path || 'Nije odabrano...'}
          className="flex-grow p-2 border border-gray-300 rounded-lg bg-gray-50 focus:outline-none focus:ring-2 focus:ring-indigo-500 transition duration-150"
          placeholder="Putanja datoteke..."
        />
        <button
          onClick={() => selectFile(setter)}
          className="px-4 py-2 text-sm font-semibold text-white bg-indigo-500 rounded-lg shadow-md hover:bg-indigo-600 transition duration-150 ease-in-out transform hover:scale-[1.01] active:scale-[0.99]"
        >
          Odaberi
        </button>
      </div>
    </div>
  );

  const renderContent = () => {
    switch (activeTab) {
      case 'keys':
        return (
          <div className="space-y-6">
            <h2 className="text-xl font-semibold text-gray-800">Generiranje Ključeva</h2>
            <p className="text-gray-600">Generiraj novi set simetričnih (AES) i asimetričnih (RSA) ključeva i spremi ih u `./keys/` mapu.</p>
            <button
              onClick={generateKeys}
              className="w-full py-3 text-lg font-bold text-white bg-green-500 rounded-xl shadow-lg hover:bg-green-600 transition duration-150 ease-in-out transform hover:scale-[1.01] active:scale-[0.99] focus:outline-none focus:ring-4 focus:ring-green-300"
            >
              Generiraj i Spremi Sve Ključeve
            </button>
          </div>
        );

      case 'symmetric':
        return (
          <div className="space-y-6">
            <h2 className="text-xl font-semibold text-gray-800">Simetrična Enkripcija (AES-256 GCM)</h2>
            <FileInput label="Ulazna Datoteka za Enkripciju/Dekripciju" path={inputPath} setter={setInputPath} />
            <div className="flex space-x-4">
              <button
                onClick={handleSymmetricEncrypt}
                className="flex-1 py-3 text-lg font-bold text-white bg-blue-500 rounded-xl shadow-lg hover:bg-blue-600 transition duration-150 transform hover:scale-[1.01] active:scale-[0.99] focus:outline-none focus:ring-4 focus:ring-blue-300"
              >
                ENKRIPTIRAJ (AES)
              </button>
              <button
                onClick={handleSymmetricDecrypt}
                className="flex-1 py-3 text-lg font-bold text-blue-800 bg-blue-200 rounded-xl shadow-lg hover:bg-blue-300 transition duration-150 transform hover:scale-[1.01] active:scale-[0.99] focus:outline-none focus:ring-4 focus:ring-blue-100"
              >
                DEKRIPTIRAJ (AES)
              </button>
            </div>
            {outputPath && (
              <p className="text-sm text-gray-500 mt-2">Izlazna datoteka spremljena kao: <span className="font-mono text-gray-700">{outputPath}</span></p>
            )}
          </div>
        );

      case 'asymmetric':
        return (
          <div className="space-y-6">
            <h2 className="text-xl font-semibold text-gray-800">Asimetrična Enkripcija (RSA OAEP)</h2>
            <p className="text-sm text-yellow-700 bg-yellow-100 p-3 rounded-lg border border-yellow-300">
                <span className="font-bold">Upozorenje:</span> RSA je primarno za male datoteke/ključne razmjene.
            </p>
            <FileInput label="Ulazna Datoteka za Enkripciju/Dekripciju" path={inputPath} setter={setInputPath} />
            <div className="flex space-x-4">
              <button
                onClick={handleAsymmetricEncrypt}
                className="flex-1 py-3 text-lg font-bold text-white bg-purple-500 rounded-xl shadow-lg hover:bg-purple-600 transition duration-150 transform hover:scale-[1.01] active:scale-[0.99] focus:outline-none focus:ring-4 focus:ring-purple-300"
              >
                ENKRIPTIRAJ (RSA)
              </button>
              <button
                onClick={handleAsymmetricDecrypt}
                className="flex-1 py-3 text-lg font-bold text-purple-800 bg-purple-200 rounded-xl shadow-lg hover:bg-purple-300 transition duration-150 transform hover:scale-[1.01] active:scale-[0.99] focus:outline-none focus:ring-4 focus:ring-purple-100"
              >
                DEKRIPTIRAJ (RSA)
              </button>
            </div>
             {outputPath && (
              <p className="text-sm text-gray-500 mt-2">Izlazna datoteka spremljena kao: <span className="font-mono text-gray-700">{outputPath}</span></p>
            )}
          </div>
        );

      case 'signature':
        return (
          <div className="space-y-6">
            <h2 className="text-xl font-semibold text-gray-800">Digitalni Potpis (RSA PKCS#1 v1.5)</h2>
            <FileInput label="Datoteka za Potpis/Verifikaciju" path={inputPath} setter={setInputPath} />
            <FileInput label="Datoteka Digitalnog Potpisa (.txt)" path={signaturePath} setter={setSignaturePath} />

            <div className="grid grid-cols-3 gap-4">
              <button
                onClick={handleDigest}
                className="py-3 text-md font-bold text-white bg-gray-500 rounded-xl shadow-lg hover:bg-gray-600 transition duration-150 transform hover:scale-[1.01] active:scale-[0.99] focus:outline-none focus:ring-4 focus:ring-gray-300"
              >
                Izračunaj Digest (SHA256)
              </button>
              <button
                onClick={handleSign}
                className="py-3 text-md font-bold text-white bg-red-500 rounded-xl shadow-lg hover:bg-red-600 transition duration-150 transform hover:scale-[1.01] active:scale-[0.99] focus:outline-none focus:ring-4 focus:ring-red-300"
              >
                POTPIŠI (.txt)
              </button>
              <button
                onClick={handleVerify}
                className="py-3 text-md font-bold text-red-800 bg-red-200 rounded-xl shadow-lg hover:bg-red-300 transition duration-150 transform hover:scale-[1.01] active:scale-[0.99] focus:outline-none focus:ring-4 focus:ring-red-100"
              >
                VERIFICIRAJ
              </button>
            </div>
          </div>
        );
      default:
        return null;
    }
  };

  return (
    <div className="min-h-screen bg-gray-50 flex items-center justify-center p-4 font-sans">
      <div className="w-full max-w-xl bg-white p-8 rounded-2xl shadow-2xl border border-gray-200">
        <h1 className="text-3xl font-extrabold text-center text-indigo-700 mb-6">Kriptografski Alat</h1>
        <p className="text-center text-gray-500 mb-8">Tauri + Rust Backend</p>

        <div className="flex justify-center mb-6 border-b border-indigo-200">
          <TabButton tabId="keys">Ključevi</TabButton>
          <TabButton tabId="symmetric">Simetrično</TabButton>
          <TabButton tabId="asymmetric">Asimetrično</TabButton>
          <TabButton tabId="signature">Potpis</TabButton>
        </div>

        <div className="bg-gray-50 p-6 rounded-xl border border-gray-100 shadow-inner">
            {renderContent()}
        </div>

        <div className={`mt-8 p-4 rounded-xl font-mono text-sm shadow-inner ${
            resultMessage.startsWith('GREŠKA') ? 'bg-red-100 text-red-700 border border-red-300' : 
            resultMessage.includes('USPIJELA') || resultMessage.includes('kompleted') ? 'bg-green-100 text-green-700 border border-green-300' : 
            'bg-indigo-100 text-indigo-700 border border-indigo-300'
        }`}>
          <h3 className="font-bold mb-1">Status/Rezultat:</h3>
          <p>{resultMessage || 'Čekam komandu...'}</p>
        </div>
      </div>
    </div>
  );
};

export default App;