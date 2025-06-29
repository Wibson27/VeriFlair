/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./src/**/*.{js,ts,jsx,tsx}", // Pastikan ini mencakup semua file
  ],
  theme: {
    extend: {
      backgroundImage: {
        'gradient-border': 'conic-gradient(from var(--gradient-angle), #ff5733, #33ff57, #3357ff, #ff5733)',
      },
      colors: {
        'brand-dark': '#0D1117', // Latar belakang utama seperti GitHub
        'brand-card': '#161B22', // Warna kartu/baris
        'brand-border': '#30363D',
        'brand-primary-button': '#238636',
        'brand-secondary-button': '#373E47',
        'rank-1': 'rgba(191, 142, 6, 0.2)', // Emas dengan transparansi
        'rank-2': 'rgba(138, 151, 161, 0.2)', // Perak dengan transparansi
        'rank-3': 'rgba(172, 106, 75, 0.2)', // Perunggu dengan transparansi
        'highlight-card-bg': '#312A21', // Latar belakang kartu highlight
      },
       // Tambahkan seluruh blok di bawah ini
      boxShadow: {
        'white-glow': '0 0 15px 4px rgba(255, 255, 255, 0.5)',
        'white-glow-strong': '0 0 25px 8px rgba(255, 255, 255, 0.8)',

      },
      keyframes: {
        glow: {
          '0%, 100%': { opacity: 0.7 },
          '50%': { opacity: 1 },
        },

        'gold-glow': {
          '0%, 100%': { filter: 'drop-shadow(0 0 6px rgba(255, 215, 0, 0.8))' },
          '50%': { filter: 'drop-shadow(0 0 24px rgba(255, 225, 102, 1))' },
        },
        'silver-glow': {
          '0%, 100%': { filter: 'drop-shadow(0 0 6px rgba(192, 192, 192, 0.8))' },
          '50%': { filter: 'drop-shadow(0 0 12px rgba(230, 230, 230, 0.8))' },
        },
        'bronze-glow': {
          '0%, 100%': { filter: 'drop-shadow(0 0 6px rgba(205, 127, 50, 0.8))' },
          '50%': { filter: 'drop-shadow(0 0 12px rgba(210, 180, 140, 0.8))' },
        },

        'border-spin': {
          '0%': { '--gradient-angle': '0deg' },
          '100%': { '--gradient-angle': '360deg' },
        },

        shimmer: {
          '0%': { transform: 'translateX(-100%)' },
          '100%': { transform: 'translateX(100%)' },
        }
      },
      animation: {
        'glow-pulse': 'glow 2s ease-in-out infinite',
        'border-spin': 'border-spin 6s linear infinite',
        'gold-pulse': 'gold-glow 3s ease-in-out infinite',
        'silver-pulse': 'silver-glow 3s ease-in-out infinite',
        'bronze-pulse': 'bronze-glow 3s ease-in-out infinite',
        shimmer: 'shimmer 4s ease-in-out infinite', 
      }
      // Akhir dari blok tambahan
    },
  },
  plugins: [],
}

// tailwind.config.js
export default {
  theme: {
    extend: {
      fontFamily: {
        sfpro: ['SF Pro Display', 'Inter', 'sans-serif'],
      },
    },
  },
  // ...
}

