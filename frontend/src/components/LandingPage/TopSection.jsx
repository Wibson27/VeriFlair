import React from 'react';
import { Link } from 'react-router-dom'; // 1. Impor Link
import bgImage from '../../assets/image/LandingPage/bgtop.png';
import ShootingStars from '../../styles/ShootingStars'; // sesuaikan path 

export default function TopSection() {
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
            VeriFlair links your GitHub to your Internet Identity...
          </p>
          <Link
            to="/leaderboard"
            className="relative inline-block px-6 py-3 font-sfpro font-normal text-white rounded-full group overflow-hidden transition-all duration-300"
            >
              {/* Gradient border layer */}
              <span className="absolute inset-0 rounded-full bg-gradient-to-r from-purple-500 via-blue-500 to-cyan-500 opacity-0 group-hover:opacity-100 transition-opacity duration-300"></span>
              
              {/* Inner layer (solid bg) */}
              <span className="relative z-10 block bg-[#0E0E0E] group-hover:bg-white group-hover:text-[#1D2460] px-6 py-3 rounded-full border border-white transition-all duration-300">
                  Get Verified Now
              </span>
            </Link>

        </div>
      </div>
    </section>
  );
}
