import React, { useState } from 'react';
import './App.css';
import { TabButton } from './components/shared/TabButton';
import { SymmetricTab } from './components/tabs/SymmetricTab';
import { KeysTab } from './components/tabs/KeysTab';
import { AsymmetricTab } from './components/tabs/AsymmetricTab';
import { SignatureTab } from './components/tabs/SignatureTab';

const App: React.FC = () => {
  const [activeTab, setActiveTab] = useState<'keys' | 'symmetric' | 'asymmetric' | 'signature'>('keys');
  const [resultMessage, setResultMessage] = useState('');
  const [inputPath, setInputPath] = useState('');
  const [outputPath, setOutputPath] = useState('');
  const [signaturePath, setSignaturePath] = useState('');

  const getOutputPath = (originalPath: string, suffix: string) => {
    if (!originalPath) return 'output.bin';
    const dotIndex = originalPath.lastIndexOf('.');
    if (dotIndex === -1) return `${originalPath}_${suffix}.bin`;
    const base = originalPath.substring(0, dotIndex);
    return `${base}_${suffix}.bin`;
  };

  return (
    <div className="min-h-screen bg-gray-100 flex items-center justify-center p-6">
      <div className="w-full max-w-3xl bg-white p-8 rounded-2xl shadow-2xl border border-gray-200">
        <h1 className="text-4xl font-extrabold text-center text-indigo-700 mb-8">
          🔐 Kriptografski Alat
        </h1>

        <div className="flex justify-center mb-6 border-b border-indigo-200">
          <TabButton tabId="keys" activeTab={activeTab} setActiveTab={setActiveTab} setResultMessage={setResultMessage}>
            Ključevi
          </TabButton>
          <TabButton tabId="symmetric" activeTab={activeTab} setActiveTab={setActiveTab} setResultMessage={setResultMessage}>
            Simetrično
          </TabButton>
          <TabButton tabId="asymmetric" activeTab={activeTab} setActiveTab={setActiveTab} setResultMessage={setResultMessage}>
            Asimetrično
          </TabButton>
          <TabButton tabId="signature" activeTab={activeTab} setActiveTab={setActiveTab} setResultMessage={setResultMessage}>
            Potpis
          </TabButton>
        </div>

        <div className="bg-gray-50 p-6 rounded-xl border border-gray-100 shadow-inner">
          {activeTab === 'keys' && <KeysTab setResultMessage={setResultMessage} />}

          {activeTab === 'symmetric' && (
            <SymmetricTab
              inputPath={inputPath}
              setInputPath={setInputPath}
              outputPath={outputPath}
              setOutputPath={setOutputPath}
              setResultMessage={setResultMessage}
              getOutputPath={getOutputPath}
            />
          )}

          {activeTab === 'asymmetric' && (
            <AsymmetricTab
              inputPath={inputPath}
              setInputPath={setInputPath}
              outputPath={outputPath}
              setOutputPath={setOutputPath}
              setResultMessage={setResultMessage}
              getOutputPath={getOutputPath}
            />
          )}

          {activeTab === 'signature' && (
            <SignatureTab
              inputPath={inputPath}
              setInputPath={setInputPath}
              signaturePath={signaturePath}
              setSignaturePath={setSignaturePath}
              setResultMessage={setResultMessage}
            />
          )}
        </div>

        <div className="mt-8 p-5 rounded-xl font-mono text-sm shadow-inner bg-indigo-50 text-indigo-800 border border-indigo-300">
          <h3 className="font-bold mb-2">📜 Status / Rezultat:</h3>
          <p className="whitespace-pre-wrap break-all">
            {resultMessage || 'Čekam komandu...'}
          </p>
        </div>
      </div>
    </div>
  );
};

export default App;
