// src/components/LeaderBoardPage/LeaderboardItem.jsx

import React from 'react';
import { Link } from 'react-router-dom';

const LeaderboardItem = ({ place, username, points, badges, showPlace = true, applyRankStyling = true }) => {
  
  const getRankClasses = () => {
    // Jika styling rank tidak diaplikasikan, langsung kembalikan warna standar
    if (!applyRankStyling) {
      return 'bg-brand-card border-brand-border';
    }
    
    // Tentukan kelas dasar dan kelas glow secara terpisah
    let baseClasses = '';
    let glowClasses = '';

    switch (place) {
      case 1:
        baseClasses = 'bg-rank-1 border-yellow-500/50';
        // Gunakan glow yang lebih kuat untuk peringkat 1
        glowClasses = 'gold-glow animate-glow-pulse';
        break;
      case 2:
        baseClasses = 'bg-rank-2 border-slate-400/50';
        glowClasses = 'silver-glow animate-glow-pulse';
        break;
      case 3:
        baseClasses = 'bg-rank-3 border-orange-400/50';
        glowClasses = 'bronze-glow animate-glow-pulse';
        break;
      default:
        baseClasses = 'bg-brand-card border-brand-border';
        // Peringkat 4 ke bawah tidak memiliki glow
        glowClasses = '';
        break;
    }
    
    return `${baseClasses} ${glowClasses}`;
  };

  return (
    <Link to={`/users/${username}`} className="block">
      <div className={`flex items-center p-3 rounded-lg border mb-2 hover:border-blue-500 transition-all duration-300 ${getRankClasses()}`}>
        
        {showPlace && (
          <div className="w-16 text-lg font-bold text-gray-300 text-center">#{place}</div>
        )}
        
        <div className="flex-1 flex items-center gap-4">
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
