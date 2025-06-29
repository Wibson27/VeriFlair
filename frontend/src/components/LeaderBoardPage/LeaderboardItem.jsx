import React from 'react';
import { Link } from 'react-router-dom';

// PERUBAHAN: Tambahkan props 'showPlace' dan 'applyRankStyling'
const LeaderboardItem = ({ place, username, points, badges, showPlace = true, applyRankStyling = true }) => {
  
  const getRankBg = () => {
    // Jika styling rank tidak diaplikasikan, langsung kembalikan warna standar
    if (!applyRankStyling) {
      return 'bg-brand-card border-brand-border';
    }
    
    switch (place) {
      case 1:
        return 'bg-rank-1 border-yellow-500/50';
      case 2:
        return 'bg-rank-2 border-slate-400/50';
      case 3:
        return 'bg-rank-3 border-orange-400/50';
      default:
        return 'bg-brand-card border-brand-border';
    }
  };

  return (
    <Link to={`/users/${username}`} className="block">
      <div className={`flex items-center p-3 rounded-lg border ${getRankBg()} mb-2 hover:border-blue-500 transition-colors`}>
        
        {/* PERUBAHAN: Tampilkan kolom 'Place' hanya jika showPlace bernilai true */}
        {showPlace && (
          <div className="w-16 text-lg font-bold text-gray-300 text-center">#{place}</div>
        )}
        
        <div className="flex-1 flex items-center gap-4">
          {/* PERUBAHAN: Tambahkan ml-16 jika kolom place disembunyikan agar lurus */}
          <div className={`w-10 h-10 bg-gray-600 rounded-full ${!showPlace ? 'ml-16' : ''}`}></div>
          <span className="text-white font-medium">@{username}</span>
        </div>

        <div className="w-32 text-center text-white font-semibold">{points}</div>
        <div className="w-24 text-center text-white font-semibold">{badges}</div>
      </div>
    </Link>
  );
};

export default LeaderboardItem;