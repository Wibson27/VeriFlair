/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./src/**/*.{js,ts,jsx,tsx}", // Pastikan ini mencakup semua file
  ],
  theme: {
    extend: {},
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

