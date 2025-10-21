import React, { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { FileInput } from "../shared/FileInput";

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
  const [useCustomSignature, setUseCustomSignature] = useState(false);
  const [signatures, setSignatures] = useState<string[]>([]);
  const [selectedSignature, setSelectedSignature] = useState("");

  const fetchSignatures = async () => {
    try {
      const result = await invoke<string[]>("list_signatures_cmd");
      setSignatures(result);
    } catch (err) {
      console.error("Error loading signatures:", err);
    }
  };
  
  useEffect(() => {
    fetchSignatures();
  }, []);

  const handleDigest = async () => {
    if (!inputPath) return setResultMessage("Choose a file for digest.");
    setResultMessage("Calculating digest...");
    try {
      const res = await invoke<string>("calculate_digest_and_save", { inputPath });
      setResultMessage(res);
    } catch (err: any) {
      setResultMessage(`ERROR: ${err}`);
    }
  };

  const handleSign = async () => {
    if (!inputPath) return setResultMessage("Choose a file for signing.");
    setResultMessage("Signing...");
    try {
      const res = await invoke<string>("digitally_sign", { filePath: inputPath });
      setResultMessage(res);
      fetchSignatures();
    } catch (err: any) {
      setResultMessage(`ERROR: ${err}`);
    }
  };

  const handleVerify = async () => {
    if (!inputPath) return setResultMessage("Choose a file for verification.");

    let sigPath = signaturePath;
    if (!useCustomSignature) {
      if (!selectedSignature) return setResultMessage("Choose a digital signature from the list.");
      sigPath = `signature/${selectedSignature}`;
    }

    setResultMessage("Verification in progress...");
    try {
      setResultMessage("sigPath: " + sigPath);
      const isValid = await invoke<boolean>("verify_signature", {
        filePath: inputPath,
        signaturePath: sigPath,
      });
        
      setResultMessage(
        isValid
          ? "✅ VERIFICATION SUCCESSFUL: The signature is valid!"
          : "❌ VERIFICATION FAILED: The signature is invalid."
      );
    } catch (err: any) {
      setResultMessage(`ERROR: ${err}`);
    }
  };

  return (
    <div className="space-y-6">
      <h2 className="text-xl font-semibold text-gray-800">
        Digital Signature (RSA PKCS#1 v1.5)
      </h2>

      <FileInput
        label="File for Signing/Verification"
        path={inputPath}
        setter={setInputPath}
        setResultMessage={setResultMessage}
      />

      <div className="flex items-center space-x-2 mt-2">
        <input
          id="customSig"
          type="checkbox"
          checked={useCustomSignature}
          onChange={(e) => setUseCustomSignature(e.target.checked)}
          className="w-4 h-4 accent-indigo-600"
        />
        <label htmlFor="customSig" className="text-sm font-medium text-gray-700">
          Use custom signature file
        </label>
      </div>

      {!useCustomSignature ? (
        <div className="mt-3">
          <label className="block text-sm font-medium text-gray-700 mb-1">
            Choose Signature from AppData/signature/
          </label>
          <select
            className="w-full border border-gray-300 rounded-lg p-2 bg-white shadow-sm focus:ring-2 focus:ring-indigo-500 focus:border-indigo-500"
            value={selectedSignature}
            onChange={(e) => setSelectedSignature(e.target.value)}
          >
            <option value="">-- Choose signature file --</option>
            {signatures.map((sig, i) => (
              <option key={i} value={sig}>
                {sig}
              </option>
            ))}
          </select>
        </div>
      ) : (
        <FileInput
          label="Custom Signature File (.txt)"
          path={signaturePath}
          setter={setSignaturePath}
          setResultMessage={setResultMessage}
        />
      )}

      <div className="grid grid-cols-3 gap-4 mt-4">
        <button
          onClick={handleDigest}
          className="py-3 text-md font-bold text-white bg-gray-500 rounded-xl shadow-lg hover:bg-gray-600"
        >
          CALCULATE DIGEST (SHA256)
        </button>
        <button
          onClick={handleSign}
          className="py-3 text-md font-bold text-white bg-indigo-600 rounded-xl shadow-lg hover:bg-indigo-700"
        >
          SIGN (.txt)
        </button>
        <button
          onClick={handleVerify}
          className="py-3 text-md font-bold text-white bg-green-600 rounded-xl shadow-lg hover:bg-green-700"
        >
          VERIFY
        </button>
      </div>
    </div>
  );
};
