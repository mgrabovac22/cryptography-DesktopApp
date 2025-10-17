import React from "react";

interface AboutAppProps {
  onClose: () => void;
}

export const AboutApp: React.FC<AboutAppProps> = ({ onClose }) => {
  return (
    <div className="modal-overlay" onClick={onClose}>
      <div className="modal-box" onClick={(e) => e.stopPropagation()}>
        <h2 className="text-lg font-semibold mb-2 text-blue-300">About This App</h2>
        <p className="text-sm text-gray-300">
          ❄ <strong>Crypto-key</strong> v2.0 — a modern Tauri-powered desktop experience built
          with React and Rust for speed and beauty.
        </p>
        <button className="close-btn" onClick={onClose}>
          Close
        </button>
      </div>
    </div>
  );
};
