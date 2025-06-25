// src/App.jsx

import React from 'react';
import { BrowserRouter, Routes, Route } from 'react-router-dom';

// Impor semua halaman dan komponen layout
import LandingPage from './pages/LandingPage';
import LeaderboardPage from './pages/LeaderboardPage';
import ProfilePage from './pages/ProfilePage';
import UserDetailPage from './pages/UserDetailPage';
import MainLayout from './components/MainLayout'; // <-- Impor layout kita

function App() {
  return (
    <BrowserRouter>
      <Routes>
        {/* RUTE 1: LandingPage dirender sendirian, tanpa pembungkus apa pun. */}
        <Route path="/" element={<LandingPage />} />

        {/* RUTE 2: LeaderboardPage DIBUNGKUS oleh MainLayout. */}
        <Route 
          path="/leaderboard" 
          element={
            <MainLayout>
              <LeaderboardPage />
            </MainLayout>
          } 
        />
        
        {/* RUTE 3: ProfilePage DIBUNGKUS oleh MainLayout. */}
        <Route 
          path="/profile" 
          element={
            <MainLayout>
              <ProfilePage />
            </MainLayout>
          } 
        />
        
        {/* RUTE 4: UserDetailPage DIBUNGKUS oleh MainLayout. */}
        <Route 
          path="/users/:username" 
          element={
            <MainLayout>
              <UserDetailPage />
            </MainLayout>
          } 
        />
      </Routes>
    </BrowserRouter>
  );
}

export default App;
