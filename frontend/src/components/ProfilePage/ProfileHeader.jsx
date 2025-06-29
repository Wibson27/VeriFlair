import React, { useState, useEffect } from 'react';
import { FiLogOut, FiRefreshCw, FiGithub } from 'react-icons/fi';
import { useNavigate } from 'react-router-dom';
import { canisterService } from '../../services/canisterService';

const ProfileHeader = ({ user: staticUser, onRefresh }) => {
  const [profile, setProfile] = useState(null);
  const [loading, setLoading] = useState(true);
  const [syncing, setSyncing] = useState(false);
  const navigate = useNavigate();

  useEffect(() => {
    const loadProfile = async () => {
      try {
        await canisterService.initialize();
        const userProfile = await canisterService.getProfile();
        setProfile(userProfile);
      } catch (error) {
        console.error('Failed to load profile:', error);
      } finally {
        setLoading(false);
      }
    };

    loadProfile();
  }, []);

  const handleSyncGitHub = async () => {
    try {
      setSyncing(true);
      const updatedProfile = await canisterService.syncGitHubData();
      setProfile(updatedProfile);

      // Notify parent component to refresh
      if (onRefresh) {
        onRefresh();
      }

      alert(`✅ Sync complete! You now have ${updatedProfile.badges?.length || 0} badges.`);
    } catch (error) {
      console.error('GitHub sync failed:', error);
      alert(`❌ Sync failed: ${error.message}`);
    } finally {
      setSyncing(false);
    }
  };

  const handleConnectGitHub = async () => {
    try {
      // Redirect to GitHub OAuth
      canisterService.initiateGitHubOAuth();
    } catch (error) {
      console.error('GitHub connection failed:', error);
      alert(`❌ Failed to connect GitHub: ${error.message}`);
    }
  };

  const handleLogout = async () => {
    try {
      await canisterService.logout();
      navigate('/');
    } catch (error) {
      console.error('Logout failed:', error);
      // Force navigation anyway
      navigate('/');
    }
  };

  // Use real profile data if available, otherwise fall back to static user data
  const displayUser = profile?.github_connected ? {
    name: profile.github_data?.name || profile.github_username,
    title: profile.github_data?.bio || 'Developer',
    avatarUrl: profile.github_data?.avatar_url || staticUser?.avatarUrl || '/default-avatar.png',
    badgesEarned: profile.total_badges || 0,
    points: profile.reputation_score || 0,
    githubUsername: profile.github_username,
    repositories: profile.github_data?.public_repos || 0,
    isConnected: true
  } : staticUser || {
    name: 'Connect GitHub',
    title: 'Get started with VeriFlair',
    avatarUrl: '/default-avatar.png',
    badgesEarned: 0,
    points: 0,
    isConnected: false
  };

  if (loading) {
    return (
      <header className="flex items-center justify-between">
        <div className="flex items-center gap-6">
          <div className="w-24 h-24 rounded-full bg-slate-700 animate-pulse"></div>
          <div className="space-y-2">
            <div className="h-8 w-48 bg-slate-700 animate-pulse rounded"></div>
            <div className="h-4 w-32 bg-slate-700 animate-pulse rounded"></div>
            <div className="h-4 w-40 bg-slate-700 animate-pulse rounded"></div>
          </div>
        </div>
        <div className="flex items-center gap-4">
          <div className="h-12 w-32 bg-slate-700 animate-pulse rounded-full"></div>
          <div className="h-12 w-24 bg-slate-700 animate-pulse rounded-full"></div>
        </div>
      </header>
    );
  }

  return (
    <header className="flex items-center justify-between">
      <div className="flex items-center gap-6">
        <div className="relative">
          <img
            src={displayUser.avatarUrl}
            alt={displayUser.name}
            className="w-24 h-24 rounded-full border-2 border-gray-700"
          />
          {displayUser.isConnected && (
            <div className="absolute -bottom-2 -right-2 bg-green-500 border-2 border-slate-800 rounded-full p-1">
              <FiGithub className="text-white" size={16} />
            </div>
          )}
        </div>

        <div>
          <h1 className="text-3xl font-bold text-white">{displayUser.name}</h1>
          <p className="text-gray-400">{displayUser.title}</p>

          {displayUser.isConnected ? (
            <div className="space-y-1">
              <p className="text-sm">
                <span className="text-green-400">@{displayUser.githubUsername}</span>
                <span className="text-gray-500"> • </span>
                <span className="text-white">{displayUser.repositories}</span>
                <span className="text-gray-500"> repositories</span>
              </p>
              <p className="text-sm">
                <span className="text-white font-semibold">{displayUser.badgesEarned}</span>
                <span className="text-gray-500"> badges earned | </span>
                <span className="text-white font-semibold">{displayUser.points.toLocaleString()}</span>
                <span className="text-blue-400"> points</span>
              </p>
            </div>
          ) : (
            <p className="text-yellow-400 text-sm mt-1">
              Connect your GitHub account to start earning badges
            </p>
          )}
        </div>
      </div>

      <div className="flex items-center gap-4">
        {displayUser.isConnected ? (
          <>
            <button
              onClick={handleSyncGitHub}
              disabled={syncing}
              className="bg-blue-900 hover:bg-blue-800 disabled:opacity-50 text-white font-semibold py-3 px-6 rounded-full flex items-center gap-2 transition-colors"
            >
              <span>{syncing ? 'Syncing...' : 'Sync with GitHub'}</span>
              <FiRefreshCw className={syncing ? 'animate-spin' : ''} />
            </button>
            <button
              onClick={handleLogout}
              className="bg-red-600 hover:bg-red-700 text-white font-semibold py-3 px-6 rounded-full flex items-center gap-2 transition-colors"
            >
              <span>Logout</span>
              <FiLogOut />
            </button>
          </>
        ) : (
          <button
            onClick={handleConnectGitHub}
            className="bg-blue-600 hover:bg-blue-700 text-white font-semibold py-3 px-6 rounded-full flex items-center gap-2 transition-colors"
          >
            <span>Connect GitHub</span>
            <FiGithub />
          </button>
        )}
      </div>
    </header>
  );
};

export default ProfileHeader;