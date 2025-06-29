// src/components/MainLayout.jsx

import React from 'react';

// Komponen ini hanya bertugas membungkus halaman dengan layout standar
const MainLayout = ({ children }) => {
  return (
    <div className="bg-brand-dark min-h-screen font-sans text-white">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        {/* Halaman yang dibungkus akan dirender di sini */}
        {children}
      </div>
    </div>
  );
};

export default MainLayout;
