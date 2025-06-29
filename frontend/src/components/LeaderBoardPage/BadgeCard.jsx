import React from 'react';
import { getBadgeImage, getTierDisplayName, getCategoryDisplayName } from '../../services/canisterService';

// Import your badge images
import bronze1 from '../../assets/image/Badges/bronze1.png';
import bronze2 from '../../assets/image/Badges/bronze2.png';
import bronze3 from '../../assets/image/Badges/bronze3.png';
import silver1 from '../../assets/image/Badges/silver1.png';
import silver2 from '../../assets/image/Badges/silver2.png';
import silver3 from '../../assets/image/Badges/silver3.png';
import gold1 from '../../assets/image/Badges/gold1.png';
import gold2 from '../../assets/image/Badges/gold2.png';
import gold3 from '../../assets/image/Badges/gold3.png';

const BadgeCard = ({
  // Legacy props for backward compatibility
  username,
  rank,
  points,
  contentClassName = "",
  pointsClassName = "",
  imageClassName = "",

  // New props for real badge data
  badge = null,
  isLeaderboard = false,
  showFullBadgeInfo = false
}) => {

  // Create a mapping for imported images
  const badgeImages = {
    'Bronze1': bronze1,
    'Bronze2': bronze2,
    'Bronze3': bronze3,
    'Silver1': silver1,
    'Silver2': silver2,
    'Silver3': silver3,
    'Gold1': gold1,
    'Gold2': gold2,
    'Gold3': gold3,
  };

  // Function to get the correct badge image
  const getCorrectBadgeImage = () => {
    if (badge?.tier) {
      const tierKey = Object.keys(badge.tier)[0];
      return badgeImages[tierKey] || gold3; // Default to gold3 if not found
    }

    // Legacy fallback - map rank to image
    if (rank) {
      if (rank.includes('Gold')) return gold3;
      if (rank.includes('Silver')) return silver1;
      if (rank.includes('Bronze')) return bronze1;
    }

    return gold3; // Default
  };

  // Function to get display name
  const getDisplayName = () => {
    if (badge) {
      if (isLeaderboard) {
        // For leaderboard, show username from the profile
        return username || 'Developer';
      } else {
        // For badge display, show the category (language/achievement name)
        return getCategoryDisplayName(badge.category);
      }
    }

    // Legacy fallback
    return username || 'Developer';
  };

  // Function to get rank/tier display
  const getRankDisplay = () => {
    if (badge?.tier) {
      return getTierDisplayName(badge.tier);
    }

    // Legacy fallback
    return rank || 'Bronze I';
  };

  // Function to get points/score display
  const getPointsDisplay = () => {
    if (badge) {
      if (isLeaderboard) {
        // For leaderboard, show total reputation score
        return points || badge.score_achieved || 0;
      } else {
        // For individual badges, show the badge score
        return badge.score_achieved || 0;
      }
    }

    // Legacy fallback
    return points || 0;
  };

  // Function to get badge rarity glow
  const getBadgeGlow = () => {
    if (!badge?.tier) return '';

    const tierKey = Object.keys(badge.tier)[0];

    if (tierKey.startsWith('Gold')) {
      return 'drop-shadow-[0_0_20px_rgba(255,215,0,0.6)]'; // Gold glow
    } else if (tierKey.startsWith('Silver')) {
      return 'drop-shadow-[0_0_15px_rgba(192,192,192,0.5)]'; // Silver glow
    } else if (tierKey.startsWith('Bronze')) {
      return 'drop-shadow-[0_0_10px_rgba(205,127,50,0.4)]'; // Bronze glow
    }

    return '';
  };

  // Enhanced image className with rarity glow
  const enhancedImageClassName = `w-full h-auto transition-all duration-300 ${imageClassName} ${getBadgeGlow()}`;

  return (
    <div className="relative w-37 h-auto group">
      {/* Badge Image with Enhanced Effects */}
      <img
        src={getCorrectBadgeImage()}
        alt="Badge Frame"
        className={enhancedImageClassName}
      />

      {/* Badge Content */}
      <div className={`absolute inset-0 flex flex-col items-center text-center ${contentClassName}`}>
        {/* Display Name */}
        <p className="text-white text-base font-semibold leading-tight">
          {getDisplayName()}
        </p>

        {/* Rank/Tier */}
        <p className="text-gray-300 text-base font-normal">
          {getRankDisplay()}
        </p>

        {/* Points/Score */}
        <div className={`text-black text-sm font-semibold mt-auto ${pointsClassName}`}>
          {getPointsDisplay()}
        </div>
      </div>

      {/* Enhanced Badge Info on Hover (optional) */}
      {showFullBadgeInfo && badge && (
        <div className="absolute top-full left-1/2 transform -translate-x-1/2 mt-2 bg-slate-800 border border-blue-500/30 rounded-lg p-3 opacity-0 group-hover:opacity-100 transition-opacity duration-200 z-10 w-64">
          <h4 className="text-white font-semibold text-sm">{badge.name}</h4>
          <p className="text-gray-300 text-xs mt-1">{badge.description}</p>

          {badge.criteria_met && badge.criteria_met.length > 0 && (
            <div className="mt-2">
              <p className="text-blue-400 text-xs font-medium">Criteria Met:</p>
              <ul className="text-gray-400 text-xs">
                {badge.criteria_met.slice(0, 2).map((criteria, index) => (
                  <li key={index}>• {criteria}</li>
                ))}
              </ul>
            </div>
          )}

          <div className="mt-2 flex justify-between text-xs">
            <span className="text-green-400">Score: {badge.score_achieved}</span>
            <span className="text-purple-400">Rarity: {badge.metadata?.rarity_score || 0}</span>
          </div>
        </div>
      )}

      {/* AI Badge Indicator */}
      {badge?.name?.includes('AI') && (
        <div className="absolute top-2 right-2 bg-gradient-to-r from-purple-500 to-pink-500 text-white text-xs px-2 py-1 rounded-full">
          🤖 AI
        </div>
      )}

      {/* New Badge Indicator */}
      {badge && isRecentBadge(badge.earned_at) && (
        <div className="absolute top-2 left-2 bg-green-500 text-white text-xs px-2 py-1 rounded-full animate-pulse">
          NEW
        </div>
      )}
    </div>
  );
};

// Helper function to check if badge is recent (earned within last 24 hours)
const isRecentBadge = (earnedAt) => {
  if (!earnedAt) return false;

  const now = Date.now() * 1000000; // Convert to nanoseconds
  const twentyFourHours = 24 * 60 * 60 * 1000 * 1000000; // 24 hours in nanoseconds

  return (now - earnedAt) < twentyFourHours;
};

export default BadgeCard;