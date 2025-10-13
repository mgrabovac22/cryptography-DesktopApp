import React from 'react';

interface TabButtonProps {
  tabId: 'keys' | 'symmetric' | 'asymmetric' | 'signature';
  activeTab: 'keys' | 'symmetric' | 'asymmetric' | 'signature';
  setActiveTab: (id: 'keys' | 'symmetric' | 'asymmetric' | 'signature') => void;
  children: React.ReactNode;
}

export const TabButton: React.FC<TabButtonProps> = ({ tabId, activeTab, setActiveTab, children }) => {
  return (
    <button
      className={`tab-button ${activeTab === tabId ? 'tab-button-active' : ''}`}
      onClick={() => setActiveTab(tabId)}
    >
      {children}
    </button>
  );
};
