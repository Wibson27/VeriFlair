  // src/components/LeaderBoardPage/Header.jsx

  import React from 'react';
  import { Link } from 'react-router-dom';
  import BadgeCard from './BadgeCard';
  import { FaGithub } from 'react-icons/fa';
  import logoImage from '../../assets/image/logo.png'; // Ganti 'logo.png' dengan nama file Anda


  // Impor semua aset gambar yang kita butuhkan untuk setiap rank
  import frameGold from '../../assets/image/Badges/frameGold.png'; // Ganti dengan path Anda
  import frameSilver from '../../assets/image/Badges/frameSilver.png'; // Ganti dengan path Anda
  import frameBronze from '../../assets/image/Badges/frameBronze.png'; // Ganti dengan path Anda

  import badgePython from '../../assets/image/Badges/pythonBronze1.png'; // Asumsi badge skill sama

  import rankGold from '../../assets/image/Badges/rankGold.png'; // Ganti dengan path Anda
  import rankSilver from '../../assets/image/Badges/rankSilver.png'; // Ganti dengan path Anda
  import rankBronze from '../../assets/image/Badges/rankBronze.png'; // Ganti dengan path Anda



  const Header = () => {
      const topUsers = [
      { id: 2, username: 'UserTwo', rank: 'Silver', points: 3800 },
      { id: 1, username: 'UserOne', rank: 'Gold', points: 4500 },
      { id: 3, username: 'UserThree', rank: 'Bronze', points: 3500 },
    ];

    // Helper function untuk memilih set gambar berdasarkan rank
  const getBadgeAssets = (rank) => {
    switch (rank) {
      case 'Gold':
        return { frame: frameGold, rankBg: rankGold };
      case 'Silver':
        return { frame: frameSilver, rankBg: rankSilver };
      case 'Bronze':
        return { frame: frameBronze, rankBg: rankBronze };
      default:
        // Fallback jika rank tidak dikenal
        return { frame: frameBronze, rankBg: rankBronze };
    }
  };

    return (
      <header className="mb-12">
        {/* PERUBAHAN 1: Tambahkan 'md:items-start' untuk meratakan kedua kolom ke atas */}
        <div className="flex flex-col md:flex-row gap-8 md:items-start">
          
          {/* Kolom Kiri */}
          <div className="flex-1 flex flex-col justify-center">
            <div className="mb-24">
              <div className="flex items-center gap-4">
                {/* Gambar Logo Anda */}
                <img src={logoImage} alt="VeriFlair Logo" className="h-12 w-auto" /> 
              {/* Teks di samping logo */}
            <div>
              <h1 className="text-2xl font-bold text-white">VeriFlair</h1>
              {/* <p className="text-gray-400 text-sm">Lorem ipsum dolor sit amet</p> */}
            </div>
          </div>
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
            <div className="flex-1">
              <div className="text-center mb-12">
                <h2 className="text-4xl font-bold text-white">Top Weekly Highlights</h2>
                  <p className="text-gray-400 mt-3 text-lg">Real skills. Real proof. Updated live.</p>
              </div>

              <div className="flex justify-center items-end gap-x-4 md:gap-x-8">
                {topUsers.map((user) => {
                  const isCenterCard = user.rank === 'Gold';
                  const assets = getBadgeAssets(user.rank);
                  
                  // Strategi podium paling stabil: menggunakan margin dan width
                  const podiumWrapperClasses = isCenterCard ? "mb-6" : "";

                  return (
                      <div key={user.id} className={podiumWrapperClasses}>
                          <BadgeCard 
                              id={user.id}
                              username={user.username}
                              rank={user.rank}
                              points={user.points}
                              frameSrc={assets.frame}
                              badgeSrc={badgePython}
                              rankBgSrc={assets.rankBg}
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