// src/pages/UserDetailPage.jsx

import React from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import BadgeGrid from '../components/ProfilePage/BadgeGrid';
import { FiArrowLeft } from 'react-icons/fi';

const dummyUserData = {
  avatarUrl: 'https://via.placeholder.com/150',
  title: 'Verified Contributor',
  badgesEarned: 7,
  points: 2300,
};

const UserDetailPage = () => {
  const { username } = useParams();
  const navigate = useNavigate();

  return (
    <div className="bg-brand-dark min-h-screen font-sans text-white py-8">
      {/* Container utama yang membatasi lebar */}
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        
        {/* Tombol Kembali */}
        <button onClick={() => navigate(-1)} className="flex items-center gap-2 text-gray-400 hover:text-white mb-6">
          <FiArrowLeft />
          Back
        </button>

        {/* Kontainer Profil */}
        <div className="border-2 border-blue-600 rounded-2xl p-8 flex flex-col items-center text-center">
          <img src={dummyUserData.avatarUrl} alt={username} className="w-28 h-28 rounded-full border-2 border-gray-700" />
          <h1 className="text-3xl font-bold text-white mt-4">@{username}</h1>
          <p className="text-gray-400">{dummyUserData.title}</p>
          <p className="text-sm mt-2">
            <span className="text-white font-semibold">{dummyUserData.badgesEarned}</span>
            <span className="text-gray-500"> badges earned | </span>
            <span className="text-white font-semibold">{dummyUserData.points.toLocaleString()}</span>
            <span className="text-blue-400"> points</span>
          </p>
        </div>

        {/* PERUBAHAN: Koleksi Badge sekarang ada DI DALAM container utama */}
        <div className="mt-12">
          <h2 className="text-2xl font-semibold mb-6 text-center md:text-left">Collection Badges</h2>
          <BadgeGrid />
        </div>
        
      </div>
    </div>
  );
};

export default UserDetailPage;
