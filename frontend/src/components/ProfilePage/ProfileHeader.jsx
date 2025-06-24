import React from 'react';
import { FiLogOut, FiRefreshCw } from 'react-icons/fi';

const ProfileHeader = ({ user }) => {
  return (
    <header className="flex items-center justify-between">
      <div className="flex items-center gap-6">
        <img src={user.avatarUrl} alt={user.name} className="w-24 h-24 rounded-full border-2 border-gray-700" />
        <div>
          <h1 className="text-3xl font-bold text-white">{user.name}</h1>
          <p className="text-gray-400">{user.title}</p>
          <p className="text-sm mt-1">
            <span className="text-white font-semibold">{user.badgesEarned}</span>
            <span className="text-gray-500"> badges earned | </span>
            <span className="text-white font-semibold">{user.points.toLocaleString()}</span>
            <span className="text-blue-400"> points</span>
          </p>
        </div>
      </div>
      <div className="flex items-center gap-4">
        {/* PERUBAHAN: Padding diubah dari py-2 px-5 menjadi py-3 px-6 */}
        <button className="bg-blue-900 hover:bg-blue-800 text-white font-semibold py-3 px-6 rounded-full flex items-center gap-2 transition-colors">
          <span>Sync with GitHub</span>
          <FiRefreshCw />
        </button>
        {/* PERUBAHAN: Padding diubah dan warna diganti menjadi merah terang */}
        <button className="bg-red-600 hover:bg-red-700 text-white font-semibold py-3 px-6 rounded-full flex items-center gap-2 transition-colors">
          <span>Logout</span>
          <FiLogOut />
        </button>
      </div>
    </header>
  );
};

export default ProfileHeader;