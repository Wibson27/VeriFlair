use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};
use ic_stable_structures::Storable;
use std::borrow::Cow;
use std::collections::HashMap;

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct UserProfile {
    pub user_principal: Principal,
    pub github_username: String,
    pub github_connected: bool,
    pub github_data: Option<GitHubData>,
    pub created_at: u64,
    pub updated_at: u64,
    pub last_github_sync: Option<u64>,
    pub reputation_score: u64,
    pub badges: Vec<Badge>,
    pub total_badges: u32,
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct GitHubData {
    pub login: String,
    pub name: Option<String>,
    pub avatar_url: String,
    pub bio: Option<String>,
    pub public_repos: u32,
    pub followers: u32,
    pub following: u32,
    pub created_at: String, // GitHub account creation date
    pub updated_at: String, // Last profile update
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct GitHubAnalysis {
    pub username: String,
    pub total_repos: u32,
    pub total_commits: u32,
    pub total_stars_received: u32,
    pub total_forks_received: u32,
    pub languages: HashMap<String, u32>, // Language -> lines of code
    pub repositories: Vec<Repository>,
    pub contributions_this_year: u32,
    pub account_age_days: u32,
    pub followers: u32,
    pub following: u32,
    pub analyzed_at: u64,
    pub commit_frequency_score: f32,
    pub code_quality_score: f32,
    pub community_engagement_score: f32,
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct Repository {
    pub name: String,
    pub full_name: String,
    pub description: Option<String>,
    pub language: Option<String>,
    pub stars: u32,
    pub forks: u32,
    pub size: u32,
    pub is_fork: bool,
    pub is_private: bool,
    pub created_at: String,
    pub updated_at: String,
    pub pushed_at: String,
    pub commits_count: Option<u32>,
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct Badge {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: BadgeCategory,
    pub tier: BadgeTier,
    pub earned_at: u64,
    pub criteria_met: Vec<String>,
    pub score_achieved: u32,
    pub metadata: BadgeMetadata,
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub enum BadgeCategory {
    Language(String),      // "Rust", "Python", "JavaScript", etc.
    Contribution(String),  // "OpenSource", "Documentation", "Community", etc.
    Achievement(String),   // "Streak", "Volume", "Quality", "Consistency", etc.
    Special(String),       // "EarlyAdopter", "Mentor", "Innovation", etc.
}

// Updated Badge Tier System: Bronze/Silver/Gold with levels 1-3
#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub enum BadgeTier {
    Bronze1,
    Bronze2,
    Bronze3,
    Silver1,
    Silver2,
    Silver3,
    Gold1,
    Gold2,
    Gold3,
}

impl BadgeTier {
    pub fn get_points(&self) -> u32 {
        match self {
            BadgeTier::Bronze1 => 10,
            BadgeTier::Bronze2 => 20,
            BadgeTier::Bronze3 => 30,
            BadgeTier::Silver1 => 50,
            BadgeTier::Silver2 => 75,
            BadgeTier::Silver3 => 100,
            BadgeTier::Gold1 => 150,
            BadgeTier::Gold2 => 200,
            BadgeTier::Gold3 => 300,
        }
    }

    pub fn get_color(&self) -> &'static str {
        match self {
            BadgeTier::Bronze1 | BadgeTier::Bronze2 | BadgeTier::Bronze3 => "#CD7F32",
            BadgeTier::Silver1 | BadgeTier::Silver2 | BadgeTier::Silver3 => "#C0C0C0",
            BadgeTier::Gold1 | BadgeTier::Gold2 | BadgeTier::Gold3 => "#FFD700",
        }
    }

    pub fn get_display_name(&self) -> &'static str {
        match self {
            BadgeTier::Bronze1 => "Bronze I",
            BadgeTier::Bronze2 => "Bronze II",
            BadgeTier::Bronze3 => "Bronze III",
            BadgeTier::Silver1 => "Silver I",
            BadgeTier::Silver2 => "Silver II",
            BadgeTier::Silver3 => "Silver III",
            BadgeTier::Gold1 => "Gold I",
            BadgeTier::Gold2 => "Gold II",
            BadgeTier::Gold3 => "Gold III",
        }
    }
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct BadgeMetadata {
    pub image_url: String,
    pub animation_url: Option<String>,
    pub attributes: Vec<BadgeAttribute>,
    pub rarity_score: u32,
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct BadgeAttribute {
    pub trait_type: String,
    pub value: String,
    pub display_type: Option<String>, // "number", "boost_percentage", "date", etc.
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct GitHubOAuthRequest {
    pub code: String,
    pub state: String,
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct GitHubOAuthResponse {
    pub access_token: String,
    pub token_type: String,
    pub scope: String,
}

// LLM Analysis Results
#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct LLMAnalysis {
    pub code_quality_score: f32,
    pub contribution_consistency: f32,
    pub community_impact: f32,
    pub technical_breadth: f32,
    pub innovation_score: f32,
    pub expertise_areas: Vec<String>,
    pub recommended_badges: Vec<String>,
    pub analysis_summary: String,
    pub strengths: Vec<String>,
    pub improvement_areas: Vec<String>,
}

// Admin and Analytics
#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct ProfileStats {
    pub total_users: u64,
    pub total_badges_earned: u64,
    pub total_repositories_analyzed: u64,
    pub average_reputation: f64,
    pub github_connected_users: u64,
    pub most_common_languages: Vec<(String, u32)>,
    pub badge_distribution: HashMap<String, u32>,
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub enum UserRole {
    User,
    Admin,
    Moderator,
}

// API Response Types
#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
    pub timestamp: u64,
}

// HTTP Types for external API calls
#[derive(CandidType, Serialize, Deserialize)]
pub struct HttpRequest {
    pub method: String,
    pub url: String,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}

#[derive(CandidType, Serialize, Deserialize)]
pub struct HttpResponse {
    pub status_code: u16,
    pub headers: Vec<HttpHeader>,
    pub body: Vec<u8>,
}

#[derive(CandidType, Serialize, Deserialize)]
pub struct HttpHeader {
    pub name: String,
    pub value: String,
}

// Implement Storable for persistence
impl Storable for UserProfile {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(candid::encode_one(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        candid::decode_one(&bytes).unwrap()
    }

    const BOUND: ic_stable_structures::storable::Bound = ic_stable_structures::storable::Bound::Bounded {
        max_size: 8192, // Increased for comprehensive profile data
        is_fixed_size: false,
    };
}

impl Storable for GitHubAnalysis {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(candid::encode_one(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        candid::decode_one(&bytes).unwrap()
    }

    const BOUND: ic_stable_structures::storable::Bound = ic_stable_structures::storable::Bound::Bounded {
        max_size: 4096,
        is_fixed_size: false,
    };
}