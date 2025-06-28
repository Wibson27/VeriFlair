// src/App.jsx

import React from 'react';
import { BrowserRouter, Routes, Route } from 'react-router-dom';
import { AuthProvider } from './hooks/useAuth';

// Import all pages and layout components
import LandingPage from './pages/LandingPage';
import LeaderboardPage from './pages/LeaderboardPage';
import ProfilePage from './pages/ProfilePage';
import UserDetailPage from './pages/UserDetailPage';
import MainLayout from './components/MainLayout';
import ProtectedRoute from './components/ProtectedRoute';
import AuthTest from './components/AuthTest';

function App() {
  return (
    <AuthProvider>
      <BrowserRouter>
        <Routes>
          {/* ROUTE 1: LandingPage rendered alone, without any wrapper. */}
          <Route path="/" element={<LandingPage />} />

          {/* Auth Test Route - for development/testing Internet Identity */}
          <Route path="/auth-test" element={<AuthTest />} />

          {/* ROUTE 2: LeaderboardPage PROTECTED and WRAPPED by MainLayout. */}
          <Route
            path="/leaderboard"
            element={
              <ProtectedRoute>
                <MainLayout>
                  <LeaderboardPage />
                </MainLayout>
              </ProtectedRoute>
            }
          />

          {/* ROUTE 3: ProfilePage PROTECTED and WRAPPED by MainLayout. */}
          <Route
            path="/profile"
            element={
              <ProtectedRoute>
                <MainLayout>
                  <ProfilePage />
                </MainLayout>
              </ProtectedRoute>
            }
          />

          {/* ROUTE 4: UserDetailPage PROTECTED and WRAPPED by MainLayout. */}
          <Route
            path="/users/:username"
            element={
              <ProtectedRoute>
                <MainLayout>
                  <UserDetailPage />
                </MainLayout>
              </ProtectedRoute>
            }
          />
        </Routes>
      </BrowserRouter>
    </AuthProvider>
  );
}

export default App;