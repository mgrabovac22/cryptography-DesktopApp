import React from "react";

interface DeveloperInfoProps {
  onClose: () => void;
}

export const DeveloperInfo: React.FC<DeveloperInfoProps> = ({ onClose }) => {
  return (
    <div className="modal-overlay" onClick={onClose}>
      <div className="modal-box" onClick={(e) => e.stopPropagation()}>
        <h2 className="text-lg font-semibold mb-2 text-blue-300">Developer Info</h2>
        <p className="text-sm text-gray-300">
          👨‍💻 Developed by <strong>Marin Grabovac</strong>.
        </p>
        <p className="text-sm text-gray-400 mt-1">
          Passionate about creating clean, fast, and secure Tauri apps.
        </p>
        <button className="close-btn" onClick={onClose}>
          Close
        </button>
      </div>
    </div>
  );
};
