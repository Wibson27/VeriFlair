import React, { useState, useEffect } from 'react';
import { Link } from 'react-router-dom';
import BadgeCard from './BadgeCard';
import { FaGithub, FaSpinner } from 'react-icons/fa';
import { canisterService } from '../../services/canisterService';

const Header = () => {
  const [isConnecting, setIsConnecting] = useState(false);
  const [isAuthenticated, setIsAuthenticated] = useState(false);
  const [userProfile, setUserProfile] = useState(null);
  const [topUsers, setTopUsers] = useState([
    { id: 2, username: 'Python', rank: 'Silver 1', points: 3800 },
    { id: 1, username: 'Ruby', rank: 'Gold 1', points: 4500 },
    { id: 3, username: 'JavaScript', rank: 'Bronze 1', points: 3500 },
  ]);

  useEffect(() => {
    // Initialize canister service and check authentication status
    const checkAuthStatus = async () => {
      try {
        await canisterService.initialize();
        const authenticated = await canisterService.isAuthenticated();
        setIsAuthenticated(authenticated);

        if (authenticated) {
          // Try to get user profile
          const profile = await canisterService.getProfile();
          setUserProfile(profile);
        }

        // Load real leaderboard data
        const leaderboard = await canisterService.getLeaderboard(3);
        if (leaderboard && leaderboard.length > 0) {
          const formattedLeaderboard = leaderboard.map((user, index) => ({
            id: index + 1,
            username: user.github_username || `User ${index + 1}`,
            rank: `Points: ${user.reputation_score}`,
            points: user.total_badges,
            profile: user
          }));
          setTopUsers(formattedLeaderboard);
        }
      } catch (error) {
        console.error('Failed to initialize:', error);
      }
    };

    checkAuthStatus();
  }, []);

  const handleConnectGitHub = async () => {
    try {
      setIsConnecting(true);

      // Step 1: Authenticate with Internet Identity first (if not already)
      if (!isAuthenticated) {
        console.log('🔐 Authenticating with Internet Identity...');
        const loginSuccess = await canisterService.login();
        if (!loginSuccess) {
          throw new Error('Internet Identity authentication failed');
        }
        setIsAuthenticated(true);
      }

      // Step 2: Create initial profile if needed
      try {
        await canisterService.createProfile();
        console.log('✅ Profile created');
      } catch (error) {
        // Profile might already exist, that's okay
        console.log('Profile may already exist:', error.message);
      }

      // Step 3: Start GitHub OAuth flow
      console.log('🐙 Starting GitHub OAuth flow...');
      canisterService.initiateGitHubOAuth();

    } catch (error) {
      console.error('❌ GitHub connection failed:', error);
      alert(`Failed to connect GitHub: ${error.message}`);
      setIsConnecting(false);
    }
  };

  const getConnectButtonContent = () => {
    if (isConnecting) {
      return (
        <>
          <FaSpinner className="animate-spin" size={20} />
          <span>Connecting...</span>
        </>
      );
    }

    if (userProfile?.github_connected) {
      return (
        <>
          <span>Connected to @{userProfile.github_username}</span>
          <FaGithub size={20} />
        </>
      );
    }

    return (
      <>
        <span>Connect Github</span>
        <FaGithub size={20} />
      </>
    );
  };

  const getConnectButtonClass = () => {
    if (userProfile?.github_connected) {
      return "w-full bg-green-600 hover:bg-green-700 transition-colors duration-200 text-white font-semibold py-6 px-6 rounded-full flex items-center justify-center gap-3";
    }

    return "w-full bg-blue-800 hover:bg-blue-700 transition-colors duration-200 text-white font-semibold py-6 px-6 rounded-full flex items-center justify-center gap-3 disabled:opacity-50 disabled:cursor-not-allowed";
  };

  return (
    <header className="mb-12">
      <div className="flex flex-col md:flex-row gap-8 md:items-start">

        {/* Left Column */}
        <div className="flex-1 flex flex-col justify-center">
          <div className="mb-24">
            <h1 className="text-2xl font-bold text-white">VeriFlair</h1>
            <p className="text-gray-400 text-sm">
              {userProfile?.github_connected
                ? `Welcome back, ${userProfile.github_username}! 🎉`
                : "Verify your developer skills with AI-powered badges"
              }
            </p>
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

            {userProfile?.github_connected ? (
              <Link to="/profile" className="w-full">
                <button className={getConnectButtonClass()}>
                  {getConnectButtonContent()}
                </button>
              </Link>
            ) : (
              <button
                onClick={handleConnectGitHub}
                disabled={isConnecting}
                className={getConnectButtonClass()}
              >
                {getConnectButtonContent()}
              </button>
            )}

            {/* Status Messages */}
            {!isAuthenticated && (
              <p className="text-yellow-400 text-sm mt-2 text-center">
                ⚡ Internet Identity authentication required first
              </p>
            )}

            {userProfile?.github_connected && (
              <div className="mt-4 text-center">
                <p className="text-green-400 text-sm">
                  ✅ Connected • {userProfile.total_badges} badges earned
                </p>
                <p className="text-gray-400 text-xs">
                  Reputation Score: {userProfile.reputation_score}
                </p>
              </div>
            )}
          </div>
        </div>

        {/* Right Column - Top Weekly Highlights */}
        <div className="flex-1">
          <div className="text-center mb-20">
            <h2 className="text-4xl font-bold text-white">Top Weekly Highlights</h2>
            <p className="text-gray-400 mt-3 text-lg">
              Real skills. Real proof. Updated live.
              {topUsers.some(user => user.profile) && (
                <span className="block text-green-400 text-sm mt-1">
                  🔥 Live data from verified developers
                </span>
              )}
            </p>
          </div>

          {/* Podium Layout */}
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
                    // For leaderboard, show username
                    username={user.username}
                    rank={user.rank}
                    points={user.points}
                    contentClassName="pt-[10rem]"
                    pointsClassName={pointsStyle}
                    // Use real badge data if available
                    badge={user.profile?.badges?.[0]} // Show their top badge
                    isLeaderboard={true}
                  />
                </div>
              );
            })}
          </div>

          {/* Real-time status */}
          {topUsers.some(user => user.profile) && (
            <div className="text-center mt-6">
              <p className="text-blue-400 text-sm">
                🤖 Powered by Azure OpenAI analysis
              </p>
            </div>
          )}
        </div>
      </div>
    </header>
  );
};

export default Header;