import React from 'react';
import BadgeCard from '../LeaderBoardPage/BadgeCard';

const badges = [
  // ... data badges Anda
  { id: 1, username: 'UserOne', rank: 'Gold', points: 4500 },
  { id: 2, username: 'UserTwo', rank: 'Silver', points: 3800 },
  { id: 3, username: 'UserThree', rank: 'Bronze', points: 3500 },
  { id: 4, username: 'UserFour', rank: 'Rank', points: 3400 },
  { id: 5, username: 'UserFive', rank: 'Rank', points: 3300 },
];

const BadgeGrid = () => {
  // PERUBAHAN 3: Definisikan kelas efek glow di sini.
  // Kita menggunakan filter drop-shadow, bukan box-shadow.
  // '[--tw-drop-shadow...]' adalah cara Tailwind untuk filter.
  const glowEffectClasses = "flex justify-center rounded-2xl shadow-white-glow animate-glow-pulse transition-all duration-300 hover:shadow-white-glow-strong";

  return (
    <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 gap-6">
      {badges.map(badge => (
        <BadgeCard 
          key={badge.id}
          username={badge.username}
          rank={badge.rank}
          points={badge.points}
          contentClassName="pt-[13rem]"
          pointsClassName="pb-[0.95rem]"
          // PERUBAHAN 4: Kirim kelas efek sebagai prop ke gambar.
          imageClassName={glowEffectClasses}
        />
      ))}
    </div>
  );
};

export default BadgeGrid;