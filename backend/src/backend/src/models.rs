use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};
use ic_stable_structures::Storable;
use std::borrow::Cow;

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct UserProfile {
    pub user_principal: Principal,
    pub github_username: String,
    pub created_at: u64,
    pub reputation_score: u64,
    pub badges: Vec<Badge>,
    pub github_data: Option<GitHubData>,
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct GitHubData {
    pub repos: u32,
    pub commits: u32,
    pub stars: u32,
    pub followers: u32,
    pub languages: Vec<String>,
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct GitHubAnalysis {
    pub username: String,
    pub total_repos: u32,
    pub total_commits: u32,
    pub languages: Vec<String>,
    pub contributions_this_year: u32,
    pub account_age_days: u32,
    pub followers: u32,
    pub following: u32,
    pub analyzed_at: u64,
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct Badge {
    pub id: String,
    pub name: String,
    pub description: String,
    pub earned_at: u64,
    pub rarity: BadgeRarity,
    pub criteria: String,
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub enum BadgeRarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub enum UserRole {
    User,
    Admin,
    Moderator,
}

// NFT-related structures
#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct TokenMetadata {
    pub token_id: u64,
    pub owner: Principal,
    pub badge_id: String,
    pub badge_name: String,
    pub description: String,
    pub image_url: Option<String>,
    pub earned_at: u64,
    pub rarity: BadgeRarity,
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct MintRequest {
    pub to: Principal,
    pub badge: Badge,
}

// API Response structures
#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct LeaderboardEntry {
    pub user_principal: Principal,
    pub github_username: String,
    pub reputation_score: u64,
    pub badge_count: u32,
    pub rank: u32,
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct ProfileStats {
    pub total_users: u64,
    pub total_badges_earned: u64,
    pub total_repositories_analyzed: u64,
    pub average_reputation: f64,
}

// Implement Storable for UserProfile
impl Storable for UserProfile {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(candid::encode_one(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        candid::decode_one(&bytes).unwrap()
    }

    const BOUND: ic_stable_structures::storable::Bound = ic_stable_structures::storable::Bound::Bounded {
        max_size: 4096, // Larger size for profiles with multiple badges
        is_fixed_size: false,
    };
}

// Implement Storable for GitHubAnalysis
impl Storable for GitHubAnalysis {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(candid::encode_one(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        candid::decode_one(&bytes).unwrap()
    }

    const BOUND: ic_stable_structures::storable::Bound = ic_stable_structures::storable::Bound::Bounded {
        max_size: 2048, // GitHub analysis data
        is_fixed_size: false,
    };
}

// Implement Storable for TokenMetadata (for NFT canister)
impl Storable for TokenMetadata {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(candid::encode_one(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        candid::decode_one(&bytes).unwrap()
    }

    const BOUND: ic_stable_structures::storable::Bound = ic_stable_structures::storable::Bound::Bounded {
        max_size: 1024, // NFT metadata
        is_fixed_size: false,
    };
}