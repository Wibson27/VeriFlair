use candid::Principal;
use ic_cdk::{init, query, update};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap};
use std::cell::RefCell;

mod github;
mod llm;
mod models;
mod utils;

use models::*;
use utils::*;

type Memory = VirtualMemory<DefaultMemoryImpl>;
type ProfileStore = StableBTreeMap<Principal, UserProfile, Memory>;
type AnalysisStore = StableBTreeMap<String, GitHubAnalysis, Memory>;

const PROFILES_MEMORY_ID: MemoryId = MemoryId::new(0);
const ANALYSIS_MEMORY_ID: MemoryId = MemoryId::new(1);

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    static USER_PROFILES: RefCell<ProfileStore> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(PROFILES_MEMORY_ID)),
        )
    );

    static GITHUB_ANALYSES: RefCell<AnalysisStore> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(ANALYSIS_MEMORY_ID)),
        )
    );

    static AUTH_CANISTER_ID: RefCell<Option<Principal>> = RefCell::new(None);
    static NFT_CANISTER_ID: RefCell<Option<Principal>> = RefCell::new(None);
}

#[init]
fn init() {
    ic_cdk::println!("VeriFlair Backend initialized!");
}

#[update]
async fn create_profile(github_username: String) -> Result<UserProfile, String> {
    let caller = ic_cdk::caller();

    // Check if profile already exists
    let existing = USER_PROFILES.with(|profiles| {
        profiles.borrow().get(&caller)
    });

    if existing.is_some() {
        return Err("Profile already exists for this user".to_string());
    }

    // Verify GitHub username
    github::verify_github_username(&github_username).await?;

    // Fetch GitHub data
    let github_analysis = github::fetch_github_data(&github_username).await?;

    // Generate badges based on analysis
    let badges = generate_badges_from_analysis(&github_analysis);

    // Calculate reputation score
    let reputation_score = calculate_reputation_score(&badges);

    let profile = UserProfile {
        user_principal: caller,
        github_username: github_username.clone(),
        created_at: ic_cdk::api::time(),
        reputation_score,
        badges,
        github_data: Some(GitHubData {
            repos: github_analysis.total_repos,
            commits: github_analysis.total_commits,
            stars: 0, // TODO: Fetch from GitHub API
            followers: github_analysis.followers,
            languages: github_analysis.languages.clone(),
        }),
    };

    // Store profile
    USER_PROFILES.with(|profiles| {
        profiles.borrow_mut().insert(caller, profile.clone());
    });

    // Cache the analysis
    GITHUB_ANALYSES.with(|cache| {
        cache.borrow_mut().insert(github_username, github_analysis);
    });

    Ok(profile)
}

#[query]
fn get_profile(user: Option<Principal>) -> Option<UserProfile> {
    let target_user = user.unwrap_or_else(|| ic_cdk::caller());

    USER_PROFILES.with(|profiles| {
        profiles.borrow().get(&target_user)
    })
}

#[query]
fn get_badges(user: Option<Principal>) -> Vec<Badge> {
    let target_user = user.unwrap_or_else(|| ic_cdk::caller());

    USER_PROFILES.with(|profiles| {
        profiles.borrow().get(&target_user)
            .map(|profile| profile.badges)
            .unwrap_or_default()
    })
}

#[query]
fn get_leaderboard(limit: Option<u32>) -> Vec<UserProfile> {
    USER_PROFILES.with(|profiles| {
        let mut all_profiles: Vec<UserProfile> = profiles
            .borrow()
            .iter()
            .map(|(_, profile)| profile)
            .collect();

        // Sort by reputation score (descending)
        all_profiles.sort_by(|a, b| b.reputation_score.cmp(&a.reputation_score));

        // Apply limit
        if let Some(limit) = limit {
            all_profiles.truncate(limit as usize);
        }

        all_profiles
    })
}

#[update]
async fn refresh_github_data(github_username: String) -> Result<UserProfile, String> {
    let caller = ic_cdk::caller();

    // Get existing profile
    let mut profile = USER_PROFILES.with(|profiles| {
        profiles.borrow().get(&caller)
    }).ok_or("Profile not found")?;

    // Verify the username matches
    if profile.github_username != github_username {
        return Err("GitHub username doesn't match profile".to_string());
    }

    // Fetch fresh GitHub data
    let analysis = github::fetch_github_data(&github_username).await?;

    // Generate new badges
    let new_badges = generate_badges_from_analysis(&analysis);

    // Update profile
    profile.badges = new_badges;
    profile.reputation_score = calculate_reputation_score(&profile.badges);
    profile.github_data = Some(GitHubData {
        repos: analysis.total_repos,
        commits: analysis.total_commits,
        stars: 0, // TODO: Fetch from GitHub API
        followers: analysis.followers,
        languages: analysis.languages.clone(),
    });

    // Store updated profile
    USER_PROFILES.with(|profiles| {
        profiles.borrow_mut().insert(caller, profile.clone());
    });

    Ok(profile)
}

#[update]
async fn perform_github_analysis(username: String) -> Result<GitHubAnalysis, String> {
    github::fetch_github_data(&username).await
}

#[update]
async fn get_llm_analysis(github_username: String) -> Result<String, String> {
    // Get GitHub data first
    let github_data = github::fetch_github_data(&github_username).await?;

    // Analyze with LLM
    llm::analyze_with_llm(&github_data).await
}

async fn verify_authenticated(_caller: Principal) -> Result<(), String> {
    // For now, just check that caller is not anonymous
    if _caller == Principal::anonymous() {
        return Err("Authentication required".to_string());
    }
    Ok(())
}

async fn mint_badge_nft(user: Principal, badge: &Badge) -> Result<u64, String> {
    // Placeholder for NFT minting integration
    ic_cdk::println!("Minting NFT badge '{}' for user {}", badge.name, user);

    // TODO: Integrate with NFT canister
    Ok(1) // Return mock token ID
}

#[query]
fn greet(name: String) -> String {
    format!("Hello, {}! This is the VeriFlair backend canister.", name)
}

#[query]
fn health_check() -> String {
    "VeriFlair Backend canister is healthy".to_string()
}

#[query]
fn get_stats() -> ProfileStats {
    let total_users = USER_PROFILES.with(|profiles| profiles.borrow().len());
    let total_analyses = GITHUB_ANALYSES.with(|cache| cache.borrow().len());

    let (total_badges, avg_reputation) = USER_PROFILES.with(|profiles| {
        let profiles = profiles.borrow();
        let mut total_badges = 0u64;
        let mut total_reputation = 0u64;
        let mut user_count = 0u64;

        for (_, profile) in profiles.iter() {
            total_badges += profile.badges.len() as u64;
            total_reputation += profile.reputation_score;
            user_count += 1;
        }

        let avg_rep = if user_count > 0 {
            total_reputation as f64 / user_count as f64
        } else {
            0.0
        };

        (total_badges, avg_rep)
    });

    ProfileStats {
        total_users: total_users as u64,
        total_badges_earned: total_badges,
        total_repositories_analyzed: total_analyses as u64,
        average_reputation: avg_reputation,
    }
}

// Simple HTTP handler that doesn't use complex types
#[query]
fn get_api_info() -> String {
    "VeriFlair Backend API is running".to_string()
}

ic_cdk::export_candid!();