import React, { useEffect, useState } from "react";

interface StatusModalProps {
  isOpen: boolean;
  message: string;
  onClose: () => void;
}

export const StatusModal: React.FC<StatusModalProps> = ({ isOpen, message, onClose }) => {
  const [showStatus, setShowStatus] = useState(false);
  const [isFinished, setIsFinished] = useState(false);
  const [showLoader, setShowLoader] = useState(true);

  useEffect(() => {
    if (isOpen) {
      setShowStatus(false);
      setIsFinished(false);
      setShowLoader(true);

      const showStatusTimeout = setTimeout(() => setShowStatus(true), 1500);

      const finishTimeout = setTimeout(() => {
        if (message && !message.includes("...")) {
          setIsFinished(true);
          setShowLoader(false);
        }
      }, 2000);

      return () => {
        clearTimeout(showStatusTimeout);
        clearTimeout(finishTimeout);
      };
    }
  }, [isOpen, message]);

  if (!isOpen) return null;

  return (
    <div className="modal-overlay">
      <div className="modal-box">
        {showLoader && <div className="loader"></div>}

        {showStatus && (
          <p className="status-text">
            {message || "Processing..."}
          </p>
        )}

        {isFinished && (
          <button className="close-btn" onClick={onClose}>
            Zatvori
          </button>
        )}
      </div>
    </div>
  );
};
