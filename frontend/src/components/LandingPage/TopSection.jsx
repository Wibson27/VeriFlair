import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { useAuth } from '../../hooks/useAuth';
import bgImage from '../../assets/image/LandingPage/bgtop.png';
import ShootingStars from '../../styles/ShootingStars';

export default function TopSection() {
  const navigate = useNavigate();
  const { login, isAuthenticated, isLoading } = useAuth();
  const [isAuthenticating, setIsAuthenticating] = useState(false);

  const handleGetVerified = async () => {
    // If already authenticated, go directly to leaderboard
    if (isAuthenticated) {
      navigate('/leaderboard');
      return;
    }

    try {
      setIsAuthenticating(true);
      console.log('Starting Internet Identity login from landing page...');

      // Trigger Internet Identity login
      await login();

      // After successful login, redirect to leaderboard
      console.log('Login successful, redirecting to leaderboard...');
      navigate('/leaderboard');

    } catch (error) {
      console.error('Authentication failed:', error);
      // You could show an error message here
      alert('Authentication failed. Please try again.');
    } finally {
      setIsAuthenticating(false);
    }
  };

  const buttonLoading = isLoading || isAuthenticating;
  const buttonText = isAuthenticated
    ? 'Go to Leaderboard'
    : buttonLoading
      ? 'Authenticating...'
      : 'Get Verified Now';

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
            VeriFlair links your GitHub to your Internet Identity for verifiable, on-chain developer credentials.
          </p>

          <button
            onClick={handleGetVerified}
            disabled={buttonLoading}
            className="relative inline-block px-6 py-3 font-sfpro font-normal text-white rounded-full group overflow-hidden transition-all duration-300 disabled:opacity-50 disabled:cursor-not-allowed"
          >
            {/* Gradient border layer */}
            <span className="absolute inset-0 rounded-full bg-gradient-to-r from-purple-500 via-blue-500 to-cyan-500 opacity-0 group-hover:opacity-100 transition-opacity duration-300"></span>

            {/* Inner layer (solid bg) */}
            <span className="relative z-10 block bg-[#0E0E0E] group-hover:bg-white group-hover:text-[#1D2460] px-6 py-3 rounded-full border border-white transition-all duration-300">
              {buttonLoading && (
                <span className="inline-block animate-spin rounded-full h-4 w-4 border-b-2 border-white mr-2"></span>
              )}
              {buttonText}
            </span>
          </button>
        </div>
      </div>
    </section>
  );
}