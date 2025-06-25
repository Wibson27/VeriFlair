// src/components/LeaderBoardPage/Header.jsx

import React from 'react';
import { Link } from 'react-router-dom';
import BadgeCard from './BadgeCard';
import { FaGithub } from 'react-icons/fa';

const Header = () => {
    const topUsers = [
    { id: 2, username: 'UserTwo', rank: 'Silver', points: 3800 },
    { id: 1, username: 'UserOne', rank: 'Gold', points: 4500 },
    { id: 3, username: 'UserThree', rank: 'Bronze', points: 3500 },
  ];

  return (
    <header className="mb-12">
      {/* PERUBAHAN 1: Tambahkan 'md:items-start' untuk meratakan kedua kolom ke atas */}
      <div className="flex flex-col md:flex-row gap-8 md:items-start">
        
        {/* Kolom Kiri */}
        <div className="flex-1 flex flex-col justify-center">
          <div className="mb-24">
            <h1 className="text-2xl font-bold text-white">VeriFlair</h1>
            <p className="text-gray-400 text-sm">Lorem ipsum dolor sit amet</p>
          </div>
          
          <div className="flex-grow flex flex-col justify-center">
            
            <input 
              type="text" 
              placeholder="Search for users, badges, or languages..."
              className="w-full bg-slate-900 border border-blue-500 rounded-full px-6 py-6 text-white placeholder-gray-500 focus:outline-none focus:ring-2 focus:ring-blue-400"
            />

            <div className="flex items-center gap-4 my-4">
              <div className="h-px bg-gray-700 flex-grow"></div>
              <span className="text-gray-400">or</span>
              <div className="h-px bg-gray-700 flex-grow"></div>
            </div>
            
            <Link to="/profile" className="w-full">
              <button className="w-full bg-blue-800 hover:bg-blue-700 transition-colors duration-200 text-white font-semibold py-6 px-6 rounded-full flex items-center justify-center gap-3">
                <span>Connect Github</span>
                <FaGithub size={20} />
              </button>
            </Link>
          </div>
        </div>

        {/* Kolom Kanan */}
        {/* PERUBAHAN 2: Ganti <section> menjadi <div> dengan flex-1 agar konsisten */}
        <div className="flex-1">
          {/* Bagian Judul */}
          {/* PERUBAHAN 3: Kurangi margin bawah dari mb-20 menjadi mb-12 */}
          <div className="text-center mb-20">
            <h2 className="text-4xl font-bold text-white">Top Weekly Highlights</h2>
            <p className="text-gray-400 mt-3 text-lg">Real skills. Real proof. Updated live.</p>
          </div>

          {/* Kontainer untuk Layout Podium */}
          <div className="flex justify-center items-end gap-x-4 md:gap-x-8">
            {topUsers.map((user, index) => {
              const isCenterCard = index === 1;
              const podiumClasses = isCenterCard 
                ? "relative scale-105 md:scale-110 -translate-y-6 z-10 shadow-white-glow-strong animate-glow-pulse" 
                : "scale-95 md:scale-100 shadow-white-glow animate-glow-pulse"; 

              const pointsStyle = !isCenterCard ? "pb-[0.6rem] mt-2" : "pb-[0.65rem]";

              return (
                <div key={user.id} className={`transition-all duration-300 rounded-2xl ${podiumClasses}`}>
                  <BadgeCard 
                    username={user.username}
                    rank={user.rank}
                    points={user.points}
                    contentClassName="pt-[10rem]"
                    pointsClassName={pointsStyle}
                  />
                </div>
              );
            })}
          </div>
        </div>
      </div>
    </header>
  );
};

export default Header;
