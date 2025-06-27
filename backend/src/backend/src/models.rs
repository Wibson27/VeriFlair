// BAGIAN 'USE' STATEMENT YANG SUDAH DIPERBAIKI
use candid::{CandidType, Principal};
use ic_stable_structures::{storable::Bound, Storable}; // Path 'Bound' yang benar
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::HashMap;

// DEFINISI 'BADGE' DAN KAWAN-KAWANNYA HARUS ADA SEBELUM 'USERPROFILE'
#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct BadgeAttribute {
    pub trait_type: String,
    pub value: String,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct BadgeMetadata {
    pub image_url: String,
    pub animation_url: Option<String>,
    pub attributes: Vec<BadgeAttribute>,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub enum BadgeTier {
    Bronze,
    Silver,
    Gold,
    Platinum,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub enum BadgeCategory {
    Language(String),
    Contribution(String),
    Achievement(String),
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Badge {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: BadgeCategory,
    pub tier: BadgeTier,
    pub earned_at: u64,
    pub metadata: BadgeMetadata,
}

// SEKARANG 'USERPROFILE' BISA DIDEFINISIKAN KARENA 'BADGE' SUDAH DIKENALI
#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct UserProfile {
    #[serde(rename = "Principal")] 
    pub Principal: Principal,
    pub github_username: String,
    pub badges: Vec<Badge>,
    pub reputation_score: u32,
    pub last_analysis: Option<u64>,
    pub created_at: u64,
    pub updated_at: u64,
}

// IMPLEMENTASI STORABLE UNTUK USERPROFILE
impl Storable for UserProfile {
    fn to_bytes(&self) -> Cow<[u8]> {
        candid::encode_one(self).unwrap().into()
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        candid::decode_one(&bytes).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}

// SISA STRUCT LAINNYA
#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct LLMAnalysis {
    pub code_quality_score: f32,
    pub contribution_type: String,
    pub expertise_areas: Vec<String>,
    pub recommended_badges: Vec<String>,
    pub analysis_summary: String,
}

#[derive(CandidType, serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Repository {
    pub name: String,
    pub description: Option<String>,
    pub language: Option<String>,
    #[serde(rename = "stargazers_count")]
    pub stars: u32,
    #[serde(rename = "forks_count")]
    pub forks: u32,
    #[serde(rename = "fork")]
    pub is_fork: bool,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct GitHubAnalysis {
    pub username: String,
    pub total_commits: u32,
    pub languages: HashMap<String, u32>,
    pub repositories: Vec<Repository>,
    pub llm_insights: LLMAnalysis,
    pub analyzed_at: u64,
}

#[derive(CandidType, serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct GitHubUser {
    pub login: String,
    pub name: Option<String>,
    pub bio: Option<String>,
    pub public_repos: u32,
    pub followers: u32,
    pub following: u32,
    pub created_at: String,
}

#[derive(CandidType, Deserialize)]
pub struct HttpRequest {
    pub method: String,
    pub url: String,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}

#[derive(CandidType, Serialize)]
pub struct HttpResponse {
    pub status_code: u16,
    pub headers: Vec<HttpHeader>,
    pub body: Vec<u8>,
}

#[derive(CandidType, Serialize)]
pub struct HttpHeader {
    pub name: String,
    pub value: String,
}