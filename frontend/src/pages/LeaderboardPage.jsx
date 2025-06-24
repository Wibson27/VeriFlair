import React from 'react';
// import { BrowserRouter } from 'react-router-dom'; <-- DIHAPUS

// Sesuaikan path ke komponen Anda jika perlu
import Header from '../components/LeaderBoardPage/Header'; 
import Leaderboard from '../components/LeaderBoardPage/Leaderboard';

// Data dummy ini bisa Anda pindahkan ke sini atau impor dari file lain
const globalLeaderboardData = [
    { place: 1, username: 'UserOne', points: 4500, badges: 8 },
    { place: 2, username: 'UserTwo', points: 4500, badges: 7 },
    { place: 3, username: 'UserThree', points: 4500, badges: 6 },
    { place: 4, username: 'UserFour', points: 4500, badges: 5 },
    { place: 5, username: 'UserFive', points: 4500, badges: 4 },
    { place: 6, username: 'UserSix', points: 4500, badges: 4 },
    { place: 7, username: 'UserSeven', points: 4500, badges: 3 },
    { place: 8, username: 'UserEight', points: 4500, badges: 2 },
    { place: 9, username: 'UserNine', points: 4500, badges: 2 },
    { place: 10, username: 'UserTen', points: 4500, badges: 2},
];


function LeaderboardPage() {
  return (
    // Kita bisa gunakan div pembungkus utama ini atau React Fragment <>
    <div className="bg-brand-dark min-h-screen font-sans text-white p-4 sm:p-8">
      {/* Kontainer utama untuk membatasi lebar dan memusatkan konten */}
      <div className="max-w-7xl mx-auto">
        <Header />
        <main>
          <Leaderboard users={globalLeaderboardData} />
        </main>
      </div>
    </div>
  )
}

export default LeaderboardPage;