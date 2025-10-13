import React, { useState } from "react";
import "./App.css";
import { Snowfall } from "./components/shared/Snowfall";
import { TabButton } from "./components/shared/TabButton";
import { SymmetricTab } from "./components/tabs/SymmetricTab";
import { KeysTab } from "./components/tabs/KeysTab";
import { AsymmetricTab } from "./components/tabs/AsymmetricTab";
import { SignatureTab } from "./components/tabs/SignatureTab";

const App: React.FC = () => {
  const [activeTab, setActiveTab] = useState<'keys' | 'symmetric' | 'asymmetric' | 'signature'>('keys');
  const [resultMessage, setResultMessage] = useState("");
  const [inputPath, setInputPath] = useState("");
  const [outputPath, setOutputPath] = useState("");
  const [signaturePath, setSignaturePath] = useState("");

  const getOutputPath = (originalPath: string, suffix: string) => {
    if (!originalPath) return "output.bin";
    const parts = originalPath.split(".");
    const base = parts.slice(0, -1).join(".");
    return `${base}_${suffix}.bin`;
  };

  return (
    <div className="app-container">
      <Snowfall />
      <div className="crypto-card">
        <h1 className="crypto-title">❄ Kriptografski Alat ❄</h1>
        <p className="crypto-subtitle">Sigurnost podataka je u hladnim rukama!</p>

        <div className="tabs-container mb-6 flex w-full">
          <TabButton tabId="keys" activeTab={activeTab} setActiveTab={(id) => { setActiveTab(id); setResultMessage(''); }}>
            Ključevi
          </TabButton>
          <TabButton tabId="symmetric" activeTab={activeTab} setActiveTab={(id) => { setActiveTab(id); setResultMessage(''); }}>
            Simetrično
          </TabButton>
          <TabButton tabId="asymmetric" activeTab={activeTab} setActiveTab={(id) => { setActiveTab(id); setResultMessage(''); }}>
            Asimetrično
          </TabButton>
          <TabButton tabId="signature" activeTab={activeTab} setActiveTab={(id) => { setActiveTab(id); setResultMessage(''); }}>
            Potpis
          </TabButton>
        </div>

        <div className="inner-box">
          {activeTab === "keys" && <KeysTab setResultMessage={setResultMessage} />}
          {activeTab === "symmetric" && (
            <SymmetricTab
              inputPath={inputPath}
              setInputPath={setInputPath}
              outputPath={outputPath}
              setOutputPath={setOutputPath}
              setResultMessage={setResultMessage}
              getOutputPath={getOutputPath}
            />
          )}
          {activeTab === "asymmetric" && (
            <AsymmetricTab
              inputPath={inputPath}
              setInputPath={setInputPath}
              outputPath={outputPath}
              setOutputPath={setOutputPath}
              setResultMessage={setResultMessage}
              getOutputPath={getOutputPath}
            />
          )}
          {activeTab === "signature" && (
            <SignatureTab
              inputPath={inputPath}
              setInputPath={setInputPath}
              signaturePath={signaturePath}
              setSignaturePath={setSignaturePath}
              setResultMessage={setResultMessage}
            />
          )}
        </div>

        <div className="status-box">
          <h3 className="font-bold mb-1">Status/Rezultat:</h3>
          <p>{resultMessage || "Čekam komandu..."}</p>
        </div>
      </div>
    </div>
  );
};

export default App;
