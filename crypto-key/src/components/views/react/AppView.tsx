import React, { useState } from "react";
import "../css/AppView.css";
import { TabButton } from "../../shared/TabButton";
import { SymmetricTab } from "../../tabs/SymmetricTab";
import { KeysTab } from "../../tabs/KeysTab";
import { AsymmetricTab } from "../../tabs/AsymmetricTab";
import { SignatureTab } from "../../tabs/SignatureTab";
import { StatusModal } from "../../shared/StatusModal";

export const AppView: React.FC = () => {
  const [activeTab, setActiveTab] = useState<
    "keys" | "symmetric" | "asymmetric" | "signature"
  >("keys");
  const [, setResultMessage] = useState("");
  const [inputPath, setInputPath] = useState("");
  const [outputPath, setOutputPath] = useState("");
  const [signaturePath, setSignaturePath] = useState("");
  const [isModalOpen, setIsModalOpen] = useState(false);
  const [modalMessage, setModalMessage] = useState("");

  const showModalMessage = (msg: string) => {
    setModalMessage(msg);
    setIsModalOpen(true);
  };

  const getOutputPath = (originalPath: string, suffix: string) => {
    if (!originalPath) return "output.bin";
    const parts = originalPath.split(".");
    const base = parts.slice(0, -1).join(".");
    return `${base}_${suffix}.bin`;
  };

  return (
    <>
      <div className="crypto-card">
        <h1 className="crypto-title">❄ CRYPTO KEY ❄</h1>
        <p className="crypto-subtitle">Data security is cool!</p>

        <div className="tabs-container mb-6 flex w-full">
          <TabButton
            tabId="keys"
            activeTab={activeTab}
            setActiveTab={(id) => {
              setActiveTab(id);
              setResultMessage("");
            }}
          >
            Keys
          </TabButton>
          <TabButton
            tabId="symmetric"
            activeTab={activeTab}
            setActiveTab={(id) => {
              setActiveTab(id);
              setResultMessage("");
            }}
          >
            Symmetric
          </TabButton>
          <TabButton
            tabId="asymmetric"
            activeTab={activeTab}
            setActiveTab={(id) => {
              setActiveTab(id);
              setResultMessage("");
            }}
          >
            Asymmetric
          </TabButton>
          <TabButton
            tabId="signature"
            activeTab={activeTab}
            setActiveTab={(id) => {
              setActiveTab(id);
              setResultMessage("");
            }}
          >
            Signature
          </TabButton>
        </div>

        <div className="inner-box">
          {activeTab === "keys" && <KeysTab setResultMessage={showModalMessage} />}

          {activeTab === "symmetric" && (
            <SymmetricTab
              inputPath={inputPath}
              setInputPath={setInputPath}
              outputPath={outputPath}
              setOutputPath={setOutputPath}
              setResultMessage={showModalMessage}
              getOutputPath={getOutputPath}
            />
          )}

          {activeTab === "asymmetric" && (
            <AsymmetricTab
              inputPath={inputPath}
              setInputPath={setInputPath}
              outputPath={outputPath}
              setOutputPath={setOutputPath}
              setResultMessage={showModalMessage}
              getOutputPath={getOutputPath}
            />
          )}

          {activeTab === "signature" && (
            <SignatureTab
              inputPath={inputPath}
              setInputPath={setInputPath}
              signaturePath={signaturePath}
              setSignaturePath={setSignaturePath}
              setResultMessage={showModalMessage}
            />
          )}
        </div>
      </div>

      <StatusModal
        isOpen={isModalOpen}
        message={modalMessage}
        onClose={() => setIsModalOpen(false)}
      />
    </>
  );
};
