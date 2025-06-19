import React from 'react';

const generateStars = (count) => {
  const stars = [];

  for (let i = 0; i < count; i++) {
    const top = Math.floor(Math.random() * 100); // persen dari atas
    const left = Math.floor(Math.random() * 100); // persen dari kiri
    const delay = Math.random() * 3; // antara 0s - 3s

    stars.push(
      <div
        key={i}
        className="shooting-star"
        style={{
          top: `${top}%`,
          left: `${left}%`,
          animationDelay: `${delay}s`
        }}
      />
    );
  }

  return stars;
};

export default function ShootingStars({ count = 10 }) {
  return <>{generateStars(count)}</>;
}
