import React, { useState, useEffect } from 'react';
import Leaderboard from '../LeaderBoardPage/Leaderboard';
import { FiSearch } from 'react-icons/fi';

const leaderboardData = [
  // ... data Anda tetap di sini ...
  { place: 1, username: 'UserOne', points: 4500, badges: 8 },
  { place: 2, username: 'UserTwo', points: 4500, badges: 7 },
  { place: 3, username: 'UserThree', points: 4500, badges: 6 },
  { place: 4, username: 'UserFour', points: 4500, badges: 5 },
  { place: 5, username: 'UserFive', points: 4500, badges: 4 },
  { place: 6, username: 'UserSix', points: 4500, badges: 4 },
  { place: 7, username: 'UserSeven', points: 4500, badges: 3 },
  { place: 8, username: 'UserEight', points: 4500, badges: 2 },
  { place: 9, username: 'UserNine', points: 4500, badges: 2 },
  { place: 10, username: 'UserTen', points: 4500, badges: 2},
];

const TopContributorTab = () => {
  const [searchTerm, setSearchTerm] = useState('');
  const [filteredUsers, setFilteredUsers] = useState(leaderboardData);

  useEffect(() => {
    const results = leaderboardData.filter(user =>
      user.username.toLowerCase().includes(searchTerm.toLowerCase())
    );
    setFilteredUsers(results);
  }, [searchTerm]);

  const isSearching = searchTerm.length > 0;

  return (
    <div>
      <h3 className="text-xl font-bold text-center mb-4">Top Contributor</h3>
      
      <div className="flex justify-center items-center gap-4 mb-4">
        <div className="relative">
          {/* PERUBAHAN: Gaya input diubah */}
          <input 
            type="text" 
            placeholder="Search"
            value={searchTerm}
            onChange={(e) => setSearchTerm(e.target.value)}
            className="bg-slate-900 border border-blue-500 rounded-lg pl-10 pr-4 py-2 focus:outline-none focus:ring-2 focus:ring-blue-400"
          />
          <FiSearch className="absolute left-3 top-1/2 -translate-y-1/2 text-gray-400" />
        </div>
        {/* PERUBAHAN: Gaya select diubah */}
        <select className="bg-slate-900 border border-blue-500 rounded-lg px-4 py-2 focus:outline-none focus:ring-2 focus:ring-blue-400">
          <option>sort by</option>
        </select>
      </div>

      {isSearching && (
        <p className="text-center text-gray-400 text-sm mb-4">
          result for "{searchTerm}"
        </p>
      )}

      {/* PERUBAHAN: showPlace sekarang selalu true */}
      <Leaderboard 
        users={filteredUsers}
        showPlace={true} 
        applyRankStyling={!isSearching}
      />
    </div>
  );
};

export default TopContributorTab;   