import React from 'react';

interface TabButtonProps {
  tabId: 'keys' | 'symmetric' | 'asymmetric' | 'signature';
  activeTab: 'keys' | 'symmetric' | 'asymmetric' | 'signature';
  setActiveTab: React.Dispatch<React.SetStateAction<'keys' | 'symmetric' | 'asymmetric' | 'signature'>>;
  setResultMessage: React.Dispatch<React.SetStateAction<string>>;
  children: React.ReactNode;
}

export const TabButton: React.FC<TabButtonProps> = ({
  tabId,
  activeTab,
  setActiveTab,
  setResultMessage,
  children,
}) => {
  const isActive = activeTab === tabId;

  return (
    <button
      onClick={() => {
        setActiveTab(tabId);
        setResultMessage('');
      }}
      className={`px-4 py-2 text-sm font-medium transition duration-150 ease-in-out rounded-t-lg focus:outline-none ${
        isActive
          ? 'bg-indigo-600 text-white shadow-lg'
          : 'bg-indigo-100 text-indigo-700 hover:bg-indigo-200'
      }`}
    >
      {children}
    </button>
  );
};
