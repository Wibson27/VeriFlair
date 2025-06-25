/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./src/**/*.{js,ts,jsx,tsx}", // Pastikan ini mencakup semua file
  ],
  theme: {
    extend: {
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
        }
      },
      animation: {
        'glow-pulse': 'glow 2s ease-in-out infinite',
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

