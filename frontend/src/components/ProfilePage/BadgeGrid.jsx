// src/components/BadgeGrid.jsx

import React from 'react';
import BadgeCard from '../LeaderBoardPage/BadgeCard';

// Impor semua aset gambar yang dibutuhkan untuk setiap rank
import frameGold from '../../assets/image/Badges/frameGold.png';
import frameSilver from '../../assets/image/Badges/frameSilver.png';
import frameBronze from '../../assets/image/Badges/frameBronze.png';

import pythonBadge from '../../assets/image/Badges/pythonBronze1.png'; // Ganti nama agar lebih umum

import rankGold from '../../assets/image/Badges/rankGold.png';
import rankSilver from '../../assets/image/Badges/rankSilver.png';
import rankBronze from '../../assets/image/Badges/rankBronze.png';

const badges = [
  // ... data badges Anda
  { id: 1, username: 'UserOne', rank: 'Gold', points: 4500 },
  { id: 2, username: 'UserTwo', rank: 'Silver', points: 3800 },
  { id: 3, username: 'UserThree', rank: 'Bronze', points: 3500 },
  { id: 4, username: 'UserFour', rank: 'Bronze', points: 3400 }, // Contoh rank lain
  { id: 5, username: 'UserFive', rank: 'Silver', points: 3300 },
];

const BadgeGrid = () => {
  // Helper function untuk memilih aset gambar berdasarkan rank
  const getBadgeAssets = (rank) => {
    switch (rank) {
      case 'Gold':
        return { frame: frameGold, rankBg: rankGold };
      case 'Silver':
        return { frame: frameSilver, rankBg: rankSilver };
      case 'Bronze':
        return { frame: frameBronze, rankBg: rankBronze };
      default:
        // Fallback untuk rank yang tidak dikenal (misal: 'Rank')
        return { frame: frameBronze, rankBg: rankBronze };
    }
  };

  return (
    <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 gap-8 p-4">
      {badges.map(badge => {
        // Panggil helper function untuk mendapatkan aset yang benar
        const assets = getBadgeAssets(badge.rank);
        
        return (
          <BadgeCard 
            key={badge.id}
            id={badge.id}
            username={badge.username}
            rank={badge.rank}
            points={badge.points}
            
            // Gunakan aset dinamis dari helper function
            frameSrc={assets.frame}
            badgeSrc={pythonBadge}
            rankBgSrc={assets.rankBg}
          />
        );
      })}
    </div>
  );
};

export default BadgeGrid; 