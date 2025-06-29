import React, { useEffect, useState } from 'react';
import { useNavigate, useSearchParams } from 'react-router-dom';
import { canisterService } from '../services/canisterService';

const GitHubCallback = () => {
  const [status, setStatus] = useState('processing');
  const [message, setMessage] = useState('Processing GitHub authentication...');
  const [profile, setProfile] = useState(null);
  const [badges, setBadges] = useState([]);
  const navigate = useNavigate();
  const [searchParams] = useSearchParams();

  useEffect(() => {
    const handleCallback = async () => {
      try {
        const code = searchParams.get('code');
        const state = searchParams.get('state');
        const error = searchParams.get('error');

        if (error) {
          setStatus('error');
          setMessage(`GitHub authentication failed: ${error}`);
          return;
        }

        if (!code || !state) {
          setStatus('error');
          setMessage('Missing OAuth parameters');
          return;
        }

        setMessage('🔄 Connecting to GitHub...');

        // Process the OAuth callback with your backend
        const userProfile = await canisterService.handleGitHubCallback(code, state);

        setMessage('🤖 Analyzing your GitHub profile with Azure OpenAI...');

        // The backend has already:
        // 1. Fetched your GitHub data
        // 2. Analyzed it with Azure OpenAI
        // 3. Generated badges
        // 4. Minted NFTs

        setProfile(userProfile);
        setBadges(userProfile.badges || []);
        setStatus('success');
        setMessage(`✅ Success! Generated ${userProfile.badges?.length || 0} badges`);

        // Redirect to profile page after 3 seconds
        setTimeout(() => {
          navigate('/profile', {
            state: {
              newProfile: userProfile,
              justConnected: true
            }
          });
        }, 3000);

      } catch (error) {
        console.error('GitHub callback error:', error);
        setStatus('error');
        setMessage(`❌ Error: ${error.message}`);
      }
    };

    handleCallback();
  }, [searchParams, navigate]);

  const getStatusColor = () => {
    switch (status) {
      case 'processing': return 'text-blue-400';
      case 'success': return 'text-green-400';
      case 'error': return 'text-red-400';
      default: return 'text-gray-400';
    }
  };

  const getStatusIcon = () => {
    switch (status) {
      case 'processing': return '🔄';
      case 'success': return '✅';
      case 'error': return '❌';
      default: return '⏳';
    }
  };

  return (
    <div className="min-h-screen bg-gradient-to-br from-slate-900 via-blue-900 to-slate-900 flex items-center justify-center p-4">
      <div className="bg-slate-800 rounded-2xl p-8 max-w-2xl w-full text-center border border-blue-500/20">

        {/* Status Icon */}
        <div className="text-6xl mb-6">
          {getStatusIcon()}
        </div>

        {/* Title */}
        <h1 className="text-3xl font-bold text-white mb-4">
          VeriFlair GitHub Integration
        </h1>

        {/* Status Message */}
        <p className={`text-lg mb-8 ${getStatusColor()}`}>
          {message}
        </p>

        {/* Processing Animation */}
        {status === 'processing' && (
          <div className="flex flex-col items-center space-y-4">
            <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-400"></div>
            <div className="text-gray-400">
              <p>🐙 Fetching your GitHub repositories...</p>
              <p>🤖 Azure OpenAI is analyzing your code...</p>
              <p>🏆 Generating your developer badges...</p>
              <p>🎨 Minting NFTs for your achievements...</p>
            </div>
          </div>
        )}

        {/* Success Summary */}
        {status === 'success' && profile && (
          <div className="bg-slate-700 rounded-xl p-6 mb-6">
            <h3 className="text-xl font-semibold text-white mb-4">
              Welcome, {profile.github_data?.name || profile.github_username}! 🎉
            </h3>

            <div className="grid grid-cols-2 gap-4 text-sm">
              <div className="text-center">
                <div className="text-2xl font-bold text-blue-400">
                  {profile.github_data?.public_repos || 0}
                </div>
                <div className="text-gray-400">Repositories</div>
              </div>

              <div className="text-center">
                <div className="text-2xl font-bold text-green-400">
                  {badges.length}
                </div>
                <div className="text-gray-400">Badges Earned</div>
              </div>
            </div>

            {badges.length > 0 && (
              <div className="mt-4">
                <p className="text-gray-300 mb-2">🏆 Badges Generated:</p>
                <div className="flex flex-wrap justify-center gap-2">
                  {badges.slice(0, 5).map((badge, index) => (
                    <span
                      key={index}
                      className="bg-blue-600 text-white px-3 py-1 rounded-full text-xs"
                    >
                      {badge.name}
                    </span>
                  ))}
                  {badges.length > 5 && (
                    <span className="text-gray-400 text-xs">
                      +{badges.length - 5} more
                    </span>
                  )}
                </div>
              </div>
            )}

            <p className="text-gray-400 text-sm mt-4">
              Redirecting to your profile in 3 seconds...
            </p>
          </div>
        )}

        {/* Error State */}
        {status === 'error' && (
          <div className="space-y-4">
            <button
              onClick={() => navigate('/')}
              className="bg-blue-600 hover:bg-blue-700 text-white px-6 py-3 rounded-lg transition-colors"
            >
              Back to Home
            </button>
            <p className="text-gray-400 text-sm">
              You can try connecting your GitHub account again from the main page.
            </p>
          </div>
        )}

        {/* Manual Navigation */}
        {status === 'success' && (
          <button
            onClick={() => navigate('/profile')}
            className="bg-green-600 hover:bg-green-700 text-white px-6 py-3 rounded-lg transition-colors"
          >
            View My Profile Now
          </button>
        )}
      </div>
    </div>
  );
};

export default GitHubCallback;