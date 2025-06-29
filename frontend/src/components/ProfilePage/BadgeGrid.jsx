import React, { useState, useEffect } from 'react';
import BadgeCard from '../LeaderBoardPage/BadgeCard';
import { canisterService } from '../../services/canisterService';

const BadgeGrid = ({ forceRefresh = 0 }) => {
  const [badges, setBadges] = useState([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);

  // Static fallback badges (your original data) - shown while loading or if no real badges
  const fallbackBadges = [
    { id: 1, username: 'Python', rank: 'Gold', points: 4500 },
    { id: 2, username: 'Java', rank: 'Silver', points: 3800 },
    { id: 3, username: 'Rust', rank: 'Bronze', points: 3500 },
    { id: 4, username: 'Javascript', rank: 'Rank', points: 3400 },
    { id: 5, username: 'Golang', rank: 'Rank', points: 3300 },
  ];

  useEffect(() => {
    const loadBadges = async () => {
      try {
        setLoading(true);
        setError(null);

        // Try to get real badges from the canister
        await canisterService.initialize();
        const realBadges = await canisterService.getBadges();

        if (realBadges && realBadges.length > 0) {
          setBadges(realBadges);
        } else {
          // Use fallback badges if no real badges yet
          setBadges(fallbackBadges);
        }
      } catch (err) {
        console.error('Failed to load badges:', err);
        setError(err.message);
        // Use fallback on error
        setBadges(fallbackBadges);
      } finally {
        setLoading(false);
      }
    };

    loadBadges();
  }, [forceRefresh]); // Refresh when forceRefresh prop changes

  // Convert real badge data to display format
  const getDisplayBadges = () => {
    return badges.map((badge, index) => {
      // Check if this is a real badge (has tier property) or fallback
      if (badge.tier) {
        // Real badge from canister
        const categoryKey = Object.keys(badge.category)[0];
        const categoryValue = badge.category[categoryKey];

        return {
          id: `real-${index}`,
          username: categoryValue, // Show language/category name
          rank: getTierDisplayName(badge.tier),
          points: badge.score_achieved,
          realBadge: badge, // Pass the full badge data
          isReal: true
        };
      } else {
        // Fallback badge (static data)
        return {
          ...badge,
          realBadge: null,
          isReal: false
        };
      }
    });
  };

  // Helper function to get tier display name
  const getTierDisplayName = (tier) => {
    const tierKey = Object.keys(tier)[0];
    const displayMap = {
      'Bronze1': 'Bronze I',
      'Bronze2': 'Bronze II',
      'Bronze3': 'Bronze III',
      'Silver1': 'Silver I',
      'Silver2': 'Silver II',
      'Silver3': 'Silver III',
      'Gold1': 'Gold I',
      'Gold2': 'Gold II',
      'Gold3': 'Gold III',
    };
    return displayMap[tierKey] || 'Bronze I';
  };

  const glowEffectClasses = "flex justify-center rounded-2xl shadow-white-glow animate-glow-pulse transition-all duration-300 hover:shadow-white-glow-strong";

  const displayBadges = getDisplayBadges();

  if (loading) {
    return (
      <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 gap-6">
        {[...Array(5)].map((_, index) => (
          <div key={index} className="flex justify-center">
            <div className="w-37 h-auto bg-slate-700 animate-pulse rounded-2xl">
              <div className="aspect-square"></div>
            </div>
          </div>
        ))}
      </div>
    );
  }

  return (
    <div className="space-y-4">
      {/* Status Indicator */}
      <div className="text-center">
        {error ? (
          <p className="text-red-400 text-sm">
            ⚠️ Using demo badges - {error}
          </p>
        ) : badges.some(b => b.tier) ? (
          <p className="text-green-400 text-sm">
            ✅ Showing {badges.filter(b => b.tier).length} real badges from your GitHub profile
          </p>
        ) : (
          <p className="text-yellow-400 text-sm">
            📋 Demo badges - Connect GitHub to see your real achievements
          </p>
        )}
      </div>

      {/* Badge Grid */}
      <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 gap-6">
        {displayBadges.map(badge => (
          <div key={badge.id} className="flex justify-center">
            <BadgeCard
              username={badge.username}
              rank={badge.rank}
              points={badge.points}
              contentClassName="pt-[13rem]"
              pointsClassName="pb-[0.95rem]"
              imageClassName={glowEffectClasses}
              // Pass real badge data if available
              badge={badge.realBadge}
              isLeaderboard={false}
              showFullBadgeInfo={badge.isReal}
            />

            {/* Real badge indicator */}
            {badge.isReal && (
              <div className="absolute top-2 right-2 bg-green-500 text-white text-xs px-2 py-1 rounded-full">
                REAL
              </div>
            )}
          </div>
        ))}
      </div>

      {/* Additional Info for Real Badges */}
      {badges.some(b => b.tier) && (
        <div className="mt-6 p-4 bg-slate-800 rounded-lg border border-blue-500/20">
          <h4 className="text-white font-semibold mb-2">🤖 AI-Powered Analysis</h4>
          <p className="text-gray-300 text-sm">
            Your badges were generated using Azure OpenAI analysis of your GitHub repositories.
            Each badge represents verified skills and achievements.
          </p>

          {badges.filter(b => b.name?.includes('AI')).length > 0 && (
            <p className="text-purple-400 text-sm mt-2">
              🧠 {badges.filter(b => b.name?.includes('AI')).length} badges enhanced with AI insights
            </p>
          )}
        </div>
      )}
    </div>
  );
};

export default BadgeGrid;