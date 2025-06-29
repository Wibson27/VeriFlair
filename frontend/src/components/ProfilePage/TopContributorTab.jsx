import React, { useState, useEffect } from 'react';
import Leaderboard from '../LeaderBoardPage/Leaderboard';
import { FiSearch } from 'react-icons/fi';
import { canisterService } from '../../services/canisterService';

// Static fallback data (your original data)
const fallbackLeaderboardData = [
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
  const [leaderboardData, setLeaderboardData] = useState(fallbackLeaderboardData);
  const [filteredUsers, setFilteredUsers] = useState(fallbackLeaderboardData);
  const [loading, setLoading] = useState(true);
  const [isRealData, setIsRealData] = useState(false);
  const [sortBy, setSortBy] = useState('points');

  useEffect(() => {
    const loadLeaderboard = async () => {
      try {
        setLoading(true);

        // Try to get real leaderboard from canister
        await canisterService.initialize();
        const realLeaderboard = await canisterService.getLeaderboard(50); // Get top 50

        if (realLeaderboard && realLeaderboard.length > 0) {
          // Convert canister data to display format
          const formattedData = realLeaderboard.map((user, index) => ({
            place: index + 1,
            username: user.github_username || `User${index + 1}`,
            points: user.reputation_score || 0,
            badges: user.total_badges || 0,
            githubData: user.github_data,
            isReal: true,
            profile: user
          }));

          setLeaderboardData(formattedData);
          setIsRealData(true);
        } else {
          // Use fallback data
          setLeaderboardData(fallbackLeaderboardData);
          setIsRealData(false);
        }
      } catch (error) {
        console.error('Failed to load leaderboard:', error);
        setLeaderboardData(fallbackLeaderboardData);
        setIsRealData(false);
      } finally {
        setLoading(false);
      }
    };

    loadLeaderboard();
  }, []);

  useEffect(() => {
    // Filter and sort users based on search term and sort criteria
    let results = leaderboardData.filter(user =>
      user.username.toLowerCase().includes(searchTerm.toLowerCase())
    );

    // Sort results
    results = results.sort((a, b) => {
      switch (sortBy) {
        case 'points':
          return b.points - a.points;
        case 'badges':
          return b.badges - a.badges;
        case 'username':
          return a.username.localeCompare(b.username);
        default:
          return b.points - a.points;
      }
    });

    // Update place numbers after sorting
    results = results.map((user, index) => ({
      ...user,
      place: index + 1
    }));

    setFilteredUsers(results);
  }, [searchTerm, leaderboardData, sortBy]);

  const isSearching = searchTerm.length > 0;

  if (loading) {
    return (
      <div className="space-y-4">
        <h3 className="text-xl font-bold text-center mb-4">Top Contributor</h3>
        <div className="flex justify-center">
          <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-400"></div>
        </div>
        <p className="text-center text-gray-400">Loading leaderboard...</p>
      </div>
    );
  }

  return (
    <div>
      <h3 className="text-xl font-bold text-center mb-4">
        Top Contributor
        {isRealData && (
          <span className="block text-green-400 text-sm font-normal mt-1">
            🔥 Live data from verified developers
          </span>
        )}
      </h3>

      <div className="flex justify-center items-center gap-4 mb-4">
        <div className="relative">
          <input
            type="text"
            placeholder="Search users..."
            value={searchTerm}
            onChange={(e) => setSearchTerm(e.target.value)}
            className="bg-slate-900 border border-blue-500 rounded-lg pl-10 pr-4 py-2 focus:outline-none focus:ring-2 focus:ring-blue-400"
          />
          <FiSearch className="absolute left-3 top-1/2 -translate-y-1/2 text-gray-400" />
        </div>

        <select
          value={sortBy}
          onChange={(e) => setSortBy(e.target.value)}
          className="bg-slate-900 border border-blue-500 rounded-lg px-4 py-2 focus:outline-none focus:ring-2 focus:ring-blue-400"
        >
          <option value="points">Sort by Points</option>
          <option value="badges">Sort by Badges</option>
          <option value="username">Sort by Username</option>
        </select>
      </div>

      {/* Search Results Info */}
      {isSearching && (
        <p className="text-center text-gray-400 text-sm mb-4">
          {filteredUsers.length} result{filteredUsers.length !== 1 ? 's' : ''} for "{searchTerm}"
        </p>
      )}

      {/* Data Source Indicator */}
      {!isRealData && (
        <div className="text-center mb-4 p-3 bg-yellow-900/20 border border-yellow-500/30 rounded-lg">
          <p className="text-yellow-400 text-sm">
            📋 Showing demo data - Real leaderboard will appear as users connect their GitHub accounts
          </p>
        </div>
      )}

      {/* Real Data Statistics */}
      {isRealData && leaderboardData.length > 0 && (
        <div className="text-center mb-4 p-3 bg-green-900/20 border border-green-500/30 rounded-lg">
          <p className="text-green-400 text-sm">
            ✅ {leaderboardData.length} verified developers • Total {leaderboardData.reduce((sum, user) => sum + user.badges, 0)} badges earned
          </p>
        </div>
      )}

      {/* Empty Search Results */}
      {isSearching && filteredUsers.length === 0 && (
        <div className="text-center py-8">
          <p className="text-gray-400">No users found matching "{searchTerm}"</p>
          <button
            onClick={() => setSearchTerm('')}
            className="text-blue-400 hover:text-blue-300 text-sm mt-2"
          >
            Clear search
          </button>
        </div>
      )}

      {/* Leaderboard */}
      {filteredUsers.length > 0 && (
        <Leaderboard
          users={filteredUsers}
          showPlace={true}
          applyRankStyling={!isSearching}
        />
      )}

      {/* Additional Info for Real Data */}
      {isRealData && !isSearching && (
        <div className="mt-6 text-center">
          <p className="text-blue-400 text-sm">
            🤖 Rankings updated in real-time based on Azure OpenAI analysis
          </p>
        </div>
      )}

      {/* Refresh Button for Real Data */}
      {isRealData && (
        <div className="mt-4 text-center">
          <button
            onClick={() => window.location.reload()}
            className="text-blue-400 hover:text-blue-300 text-sm border border-blue-500/30 px-4 py-2 rounded-lg transition-colors"
          >
            🔄 Refresh Leaderboard
          </button>
        </div>
      )}
    </div>
  );
};

export default TopContributorTab;