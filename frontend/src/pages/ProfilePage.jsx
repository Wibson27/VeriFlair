import React, { useState } from 'react';
import ProfileHeader from '../components/ProfilePage/ProfileHeader';
import TabNavigator from '../components/ProfilePage/TabNavigator';
import BadgeGrid from '../components/ProfilePage/BadgeGrid';
import TopContributorTab from '../components/ProfilePage/TopContributorTab'; // <-- Tambahkan import ini

// Data dummy untuk profil pengguna yang sedang login
const loggedInUser = {
  name: 'zeroentropy',
  avatarUrl: 'https://via.placeholder.com/150', // Ganti dengan URL avatar
  title: 'Verified Contributor',
  badgesEarned: 7,
  points: 2300,
};

const ProfilePage = () => {
  // State untuk melacak tab yang aktif
  const [activeTab, setActiveTab] = useState('my-badges'); // 'my-badges' atau 'top-contributor'

  return (
    // TAMBAHKAN PEMBUNGKUS LAYOUT DI SINI
    <div className="bg-brand-dark min-h-screen font-sans text-white">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        
        {/* Konten halaman Anda yang sudah ada sebelumnya */}
        <ProfileHeader user={loggedInUser} />
        
        <main className="mt-8">
          <TabNavigator activeTab={activeTab} setActiveTab={setActiveTab} />
          
          <div className="mt-8">
            {activeTab === 'my-badges' && <BadgeGrid />}
            {activeTab === 'top-contributor' && <TopContributorTab />}
          </div>
        </main>

      </div>
    </div>
  );
};

export default ProfilePage;