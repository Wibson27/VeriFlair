import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { useAuth } from '../../hooks/useAuth';
import bgImage from '../../assets/image/LandingPage/bgtop.png';
import ShootingStars from '../../styles/ShootingStars';

export default function TopSection() {
  const navigate = useNavigate();
  const { login, logout, isAuthenticated, isLoading } = useAuth();
  const [isAuthenticating, setIsAuthenticating] = useState(false);
  const [authStatus, setAuthStatus] = useState('');

  const handleGetVerified = async () => {
    if (isAuthenticated) {
      // If already authenticated, go directly to leaderboard
      console.log('✅ Already authenticated, navigating to leaderboard');
      navigate('/leaderboard');
      return;
    }

    try {
      setIsAuthenticating(true);
      setAuthStatus('Connecting to Internet Identity...');
      console.log('🚀 Starting Internet Identity login...');

      await login();

      // Give a small delay to show success message
      setAuthStatus('Login successful! Redirecting...');
      setTimeout(() => {
        console.log('🎉 Login successful, navigating to leaderboard');
        navigate('/leaderboard');
      }, 1000);

    } catch (error) {
      console.error('❌ Login failed:', error);
      setAuthStatus('Login failed. Please try again.');

      // Show error for 3 seconds then clear
      setTimeout(() => {
        setAuthStatus('');
      }, 3000);
    } finally {
      setIsAuthenticating(false);
    }
  };

  const isButtonLoading = isLoading || isAuthenticating;

  return (
    <section
      className="relative w-full bg-no-repeat bg-top bg-[length:100%_1025px] text-white overflow-hidden"
      style={{
        backgroundImage: `url(${bgImage})`,
        minHeight: '1025px',
      }}
    >
      {/* Shooting Stars */}
      <ShootingStars count={20} />

      {/* Navbar */}
      <nav className="flex justify-between items-center px-6 py-6 text-white relative z-10">
        <div className="flex items-center gap-2">
          <div className="bg-white text-black font-bold px-2 py-1 rounded">logo</div>
          <span className="text-lg font-semibold">VeriFlair</span>
        </div>

        <div className="flex items-center gap-4">
          {/* Auth Status & Logout */}
          {isAuthenticated && (
            <button
              onClick={async () => {
                await logout();
                setAuthStatus('Logged out successfully');
                setTimeout(() => setAuthStatus(''), 2000);
              }}
              className="text-sm px-3 py-1 border border-white/30 rounded-full hover:bg-white/10 transition-colors"
            >
              Logout
            </button>
          )}

          {/* Debug Info (remove in production) */}
          {process.env.REACT_APP_DEBUG && (
            <div className="text-xs text-gray-400">
              Auth: {isAuthenticated ? '✅' : '❌'} | Loading: {isLoading ? '⏳' : '✅'}
            </div>
          )}
        </div>
      </nav>

      {/* Hero Text */}
      <div
        className="flex justify-center items-center text-center px-6 relative z-10"
        style={{ minHeight: 'calc(1025px - 96px)' }}
      >
        <div>
          <h1 className="font-sfpro font-bold text-7xl md:text-7xl mb-4">
            Build your on-chain reputation
          </h1>
          <p className="font-sfpro font-normal max-w-2xl mx-auto text-gray-200 text-lg mb-6">
            VeriFlair links your GitHub to your Internet Identity, creating verifiable badges
            that prove your coding expertise on the blockchain.
            {isAuthenticated && (
              <span className="block mt-2 text-green-400 text-base">
                ✅ You're already authenticated! Click below to continue to your dashboard.
              </span>
            )}
          </p>

          {/* Status Message */}
          {authStatus && (
            <div className="mb-4 p-3 bg-black/50 rounded-lg backdrop-blur-sm">
              <p className="text-sm text-blue-300">{authStatus}</p>
            </div>
          )}

          <button
            onClick={handleGetVerified}
            disabled={isButtonLoading}
            className="relative inline-block px-6 py-3 font-sfpro font-normal text-white rounded-full group overflow-hidden transition-all duration-300 disabled:opacity-50 disabled:cursor-not-allowed"
          >
            {/* Gradient border layer */}
            <span className="absolute inset-0 rounded-full bg-gradient-to-r from-purple-500 via-blue-500 to-cyan-500 opacity-0 group-hover:opacity-100 transition-opacity duration-300"></span>

            {/* Inner layer (solid bg) */}
            <span className="relative z-10 block bg-[#0E0E0E] group-hover:bg-white group-hover:text-[#1D2460] px-6 py-3 rounded-full border border-white transition-all duration-300">
              {isButtonLoading ? (
                <span className="flex items-center gap-2">
                  <div className="animate-spin rounded-full h-4 w-4 border-b-2 border-current"></div>
                  {isAuthenticated ? 'Redirecting...' :
                   isAuthenticating ? 'Connecting...' : 'Loading...'}
                </span>
              ) : (
                isAuthenticated ? 'Enter Dashboard' : 'Get Verified Now'
              )}
            </span>
          </button>

          {/* Development Info */}
          {process.env.REACT_APP_DEBUG && (
            <div className="mt-6 text-xs text-gray-500 max-w-md mx-auto space-y-1">
              <p>🔧 Development Mode: Using live Internet Identity with local canisters</p>
              <p>Network: {process.env.REACT_APP_DFX_NETWORK || 'local'}</p>
              {isAuthenticated && (
                <p>💡 Already logged in! Click "Logout" above to test the full login flow.</p>
              )}
            </div>
          )}
        </div>
      </div>
    </section>
  );
}