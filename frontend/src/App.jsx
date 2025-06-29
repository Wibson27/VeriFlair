import React from 'react';
import { BrowserRouter, Routes, Route } from 'react-router-dom';

// Impor semua halaman Anda
import LandingPage from './pages/LandingPage';
import LeaderboardPage from './pages/LeaderboardPage';
import ProfilePage from './pages/ProfilePage';
import UserDetailPage from './pages/UserDetailPage';

function App() {
  return (
    <BrowserRouter>
      {/* TIDAK ADA div pembungkus di sini sama sekali */}
      <Routes>
        {/* Setiap Route sekarang bertanggung jawab penuh atas layout halamannya */}
        <Route path="/" element={<LandingPage />} />
        <Route path="/leaderboard" element={<LeaderboardPage />} />
        <Route path="/profile" element={<ProfilePage />} />
        <Route path="/users/:username" element={<UserDetailPage />} />
      </Routes>
    </BrowserRouter>
  );
}

export default App;