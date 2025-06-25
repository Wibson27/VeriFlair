import React from 'react';

const TabNavigator = ({ activeTab, setActiveTab }) => {
  const activeClass = 'text-white border-b-2 border-blue-500';
  const inactiveClass = 'text-gray-500 border-b-2 border-transparent'; // Tambahkan border transparan agar tinggi sama

  return (
    // PERUBAHAN: Container utama sekarang memiliki border bawah abu-abu
    <div className="flex justify-center border-b border-gray-800">
      <button 
        // PERUBAHAN: flex-1 membuat tombol memenuhi setengah ruang
        className={`flex-1 text-center py-3 text-lg font-semibold transition-colors ${activeTab === 'my-badges' ? activeClass : inactiveClass}`}
        onClick={() => setActiveTab('my-badges')}
      >
        My Badges
      </button>
      <button 
        // PERUBAHAN: flex-1 membuat tombol memenuhi setengah ruang
        className={`flex-1 text-center py-3 text-lg font-semibold transition-colors ${activeTab === 'top-contributor' ? activeClass : inactiveClass}`}
        onClick={() => setActiveTab('top-contributor')}
      >
        Top Contributor
      </button>
    </div>
  );
};

export default TabNavigator;