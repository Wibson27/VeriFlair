import React from 'react';
import badgeImage from '../../assets/image/Badges/bronze.png'; // Pastikan path benar

// PERUBAHAN 1: Tambahkan prop baru 'imageClassName' untuk gaya spesifik pada gambar
const BadgeCard = ({ username, rank, points, contentClassName = "", pointsClassName = "", imageClassName = "" }) => {
  return (
    <div className="relative w-37 h-auto">
      {/* PERUBAHAN 2: Terapkan 'imageClassName' langsung ke tag <img>.
        Ini memungkinkan kita mengirim filter 'drop-shadow' dari luar
        yang akan menempel sempurna pada kontur gambar.
      */}
      <img 
        src={badgeImage} 
        alt="Badge Frame" 
        className={`w-full h-auto transition-all duration-300 ${imageClassName}`} 
      />

      <div className={`absolute inset-0 flex flex-col items-center text-center ${contentClassName}`}>
        <p className="text-white text-base font-semibold">{username}</p>
        <p className="text-gray-300 text-base font-normal">{rank}</p>
        <div className={`text-black text-sm font-semibold mt-auto ${pointsClassName}`}>
          {points}
        </div>
      </div>
    </div>
  );
};

export default BadgeCard;