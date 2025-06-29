import React from 'react';
import LeaderboardItem from './LeaderboardItem';

// PERUBAHAN: Komponen ini sekarang menerima props
const Leaderboard = ({ users, showPlace = true, applyRankStyling = true }) => {
  return (
    <section>
      {/* Header Tabel */}
      <div className="flex items-center px-3 py-2 text-gray-400 text-sm font-bold">
        {/* PERUBAHAN: Tampilkan header kolom 'Place' secara kondisional */}
        {showPlace && <div className="w-16 text-center">Rank</div>}
        
        {/* PERUBAHAN: Tambahkan ml-16 jika kolom place disembunyikan */}
        <div className={`flex-1 ${!showPlace ? 'ml-16' : 'ml-14'}`}>Username</div>
        <div className="w-32 text-center">Point</div>
        <div className="w-24 text-center">Badges</div>
      </div>

      {/* List Item */}
      <div>
        {/* PERUBAHAN: Gunakan data dari props, bukan data statis di sini */}
        {users.map(user => (
          <LeaderboardItem 
            key={user.place}
            place={user.place}
            username={user.username}
            points={user.points}
            badges={user.badges}
            // PERUBAHAN: Teruskan props fleksibilitas ke anak
            showPlace={showPlace}
            applyRankStyling={applyRankStyling}
          />
        ))}
      </div>
    </section>
  );
};

export default Leaderboard;