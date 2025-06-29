use candid::{CandidType, Principal};
use ic_cdk::{init, post_upgrade, pre_upgrade, query, update};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap};
use serde::{Deserialize, Serialize};
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
fn init(auth_canister: Principal, nft_canister: Principal, github_client_id: String, github_client_secret: String) {
    AUTH_CANISTER_ID.with(|id| *id.borrow_mut() = Some(auth_canister));
    NFT_CANISTER_ID.with(|id| *id.borrow_mut() = Some(nft_canister));

    // Configure GitHub OAuth
    github::set_github_oauth_config(github_client_id, github_client_secret);

    ic_cdk::println!("VeriFlair Backend initialized with GitHub integration!");
}

#[update]
async fn create_initial_profile() -> Result<UserProfile, String> {
    let caller = ic_cdk::caller();

    // Verify authentication with auth canister
    verify_authenticated(caller).await?;

    // Check if profile already exists
    if USER_PROFILES.with(|profiles| profiles.borrow().contains_key(&caller)) {
        return Err("Profile already exists for this user".to_string());
    }

    let profile = UserProfile {
        user_principal: caller,
        github_username: String::new(),
        github_connected: false,
        github_data: None,
        created_at: ic_cdk::api::time(),
        updated_at: ic_cdk::api::time(),
        last_github_sync: None,
        reputation_score: 0,
        badges: Vec::new(),
        total_badges: 0,
    };

    USER_PROFILES.with(|profiles| {
        profiles.borrow_mut().insert(caller, profile.clone());
    });

    ic_cdk::println!("Initial profile created for user: {}", caller.to_text());
    Ok(profile)
}

#[update]
async fn connect_github_oauth(oauth_request: GitHubOAuthRequest) -> Result<UserProfile, String> {
    let caller = ic_cdk::caller();

    // Verify authentication
    verify_authenticated(caller).await?;

    // Get existing profile
    let mut profile = USER_PROFILES.with(|profiles| {
        profiles.borrow().get(&caller)
    }).ok_or("Profile not found. Please create a profile first.")?;

    ic_cdk::println!("Processing GitHub OAuth for user: {}", caller.to_text());

    // Exchange OAuth code for access token
    let oauth_response = github::exchange_oauth_code(oauth_request).await
        .map_err(|e| format!("GitHub OAuth failed: {}", e))?;

    // Fetch GitHub user data
    let github_data = github::fetch_github_user(&oauth_response.access_token).await
        .map_err(|e| format!("Failed to fetch GitHub user data: {}", e))?;

    // Perform comprehensive GitHub analysis
    let analysis = github::perform_comprehensive_analysis(&github_data.login, Some(&oauth_response.access_token)).await
        .map_err(|e| format!("GitHub analysis failed: {}", e))?;

    // Generate badges based on analysis
    let new_badges = generate_badges_from_analysis(&analysis);

    // Update profile
    profile.github_username = github_data.login.clone();
    profile.github_connected = true;
    profile.github_data = Some(github_data);
    profile.badges.extend(new_badges.clone());
    profile.total_badges = profile.badges.len() as u32;
    profile.reputation_score = calculate_reputation_score(&profile.badges);
    profile.last_github_sync = Some(ic_cdk::api::time());
    profile.updated_at = ic_cdk::api::time();

    // Store updated profile
    USER_PROFILES.with(|profiles| {
        profiles.borrow_mut().insert(caller, profile.clone());
    });

    // Cache the analysis
    GITHUB_ANALYSES.with(|cache| {
        cache.borrow_mut().insert(profile.github_username.clone(), analysis);
    });

    // Mint new badges as NFTs
    for badge in new_badges {
        if let Err(e) = mint_badge_nft(caller, &badge).await {
            ic_cdk::println!("Failed to mint badge NFT for {}: {}", badge.name, e);
        }
    }

    ic_cdk::println!("GitHub connected successfully for user: {}, badges earned: {}",
                     caller.to_text(), profile.badges.len());

    Ok(profile)
}

#[update]
async fn sync_github_data() -> Result<UserProfile, String> {
    let caller = ic_cdk::caller();

    // Get existing profile
    let mut profile = USER_PROFILES.with(|profiles| {
        profiles.borrow().get(&caller)
    }).ok_or("Profile not found")?;

    if !profile.github_connected {
        return Err("GitHub account not connected".to_string());
    }

    // Rate limiting: max 1 sync per hour per user
    if let Some(last_sync) = profile.last_github_sync {
        let now = ic_cdk::api::time();
        if now - last_sync < 3600_000_000_000 { // 1 hour in nanoseconds
            return Err("GitHub sync can only be triggered once per hour".to_string());
        }
    }

    ic_cdk::println!("Syncing GitHub data for user: {}", profile.github_username);

    // Perform fresh analysis (without access token for now - would need to store securely)
    let analysis = github::perform_comprehensive_analysis(&profile.github_username, None).await
        .map_err(|e| format!("GitHub sync failed: {}", e))?;

    // Generate new badges (only add if not already earned)
    let new_badges = generate_badges_from_analysis(&analysis);
    let existing_badge_ids: std::collections::HashSet<String> = profile.badges.iter()
        .map(|b| b.id.clone())
        .collect();

    let truly_new_badges: Vec<Badge> = new_badges.into_iter()
        .filter(|b| !existing_badge_ids.contains(&b.id))
        .collect();

    // Update profile
    profile.badges.extend(truly_new_badges.clone());
    profile.total_badges = profile.badges.len() as u32;
    profile.reputation_score = calculate_reputation_score(&profile.badges);
    profile.last_github_sync = Some(ic_cdk::api::time());
    profile.updated_at = ic_cdk::api::time();

    // Store updated profile
    USER_PROFILES.with(|profiles| {
        profiles.borrow_mut().insert(caller, profile.clone());
    });

    // Update cached analysis
    GITHUB_ANALYSES.with(|cache| {
        cache.borrow_mut().insert(profile.github_username.clone(), analysis);
    });

    // Mint new badges as NFTs
    for badge in truly_new_badges {
        if let Err(e) = mint_badge_nft(caller, &badge).await {
            ic_cdk::println!("Failed to mint badge NFT for {}: {}", badge.name, e);
        }
    }

    ic_cdk::println!("GitHub sync completed for user: {}, total badges: {}",
                     profile.github_username, profile.badges.len());

    Ok(profile)
}

#[update]
async fn disconnect_github() -> Result<UserProfile, String> {
    let caller = ic_cdk::caller();

    let mut profile = USER_PROFILES.with(|profiles| {
        profiles.borrow().get(&caller)
    }).ok_or("Profile not found")?;

    profile.github_connected = false;
    profile.github_data = None;
    profile.updated_at = ic_cdk::api::time();

    USER_PROFILES.with(|profiles| {
        profiles.borrow_mut().insert(caller, profile.clone());
    });

    ic_cdk::println!("GitHub disconnected for user: {}", caller.to_text());
    Ok(profile)
}

#[query]
fn get_profile(user: Option<Principal>) -> Option<UserProfile> {
    let target = user.unwrap_or_else(|| ic_cdk::caller());
    USER_PROFILES.with(|profiles| {
        profiles.borrow().get(&target)
    })
}

#[query]
fn get_badges(user: Option<Principal>) -> Vec<Badge> {
    let target = user.unwrap_or_else(|| ic_cdk::caller());
    USER_PROFILES.with(|profiles| {
        profiles.borrow().get(&target)
            .map(|profile| profile.badges)
            .unwrap_or_default()
    })
}

#[query]
fn get_leaderboard(limit: Option<u32>) -> Vec<UserProfile> {
    let limit = limit.unwrap_or(100).min(1000) as usize;

    USER_PROFILES.with(|profiles| {
        let mut all_profiles: Vec<UserProfile> = profiles
            .borrow()
            .iter()
            .filter(|(_, profile)| profile.github_connected) // Only include GitHub-connected users
            .map(|(_, profile)| profile)
            .collect();

        all_profiles.sort_by(|a, b| b.reputation_score.cmp(&a.reputation_score));
        all_profiles.truncate(limit);
        all_profiles
    })
}

#[query]
fn get_github_analysis(username: String) -> Option<GitHubAnalysis> {
    GITHUB_ANALYSES.with(|analyses| {
        analyses.borrow().get(&username)
    })
}

#[query]
fn get_badge_statistics() -> Vec<(String, u32)> {
    let mut badge_counts = std::collections::HashMap::new();

    USER_PROFILES.with(|profiles| {
        for (_, profile) in profiles.borrow().iter() {
            for badge in &profile.badges {
                *badge_counts.entry(badge.name.clone()).or_insert(0) += 1;
            }
        }
    });

    let mut stats: Vec<(String, u32)> = badge_counts.into_iter().collect();
    stats.sort_by(|a, b| b.1.cmp(&a.1)); // Sort by count descending
    stats.truncate(20); // Top 20 badges
    stats
}

#[update]
async fn validate_github_username(username: String) -> Result<bool, String> {
    github::validate_github_username(&username).await
}

#[update]
async fn get_github_oauth_url(state: String) -> Result<String, String> {
    // In a real implementation, you'd construct the OAuth URL
    // For now, return a placeholder
    Ok(format!("https://github.com/login/oauth/authorize?client_id=YOUR_CLIENT_ID&scope=user:email,public_repo&state={}", state))
}

// Admin functions
#[update(guard = "is_admin")]
async fn admin_force_github_sync(user: Principal) -> Result<String, String> {
    let mut profile = USER_PROFILES.with(|profiles| {
        profiles.borrow().get(&user)
    }).ok_or("User profile not found")?;

    if !profile.github_connected {
        return Err("User has no GitHub connection".to_string());
    }

    // Force analysis without rate limiting
    let analysis = github::perform_comprehensive_analysis(&profile.github_username, None).await
        .map_err(|e| format!("GitHub analysis failed: {}", e))?;

    let new_badges = generate_badges_from_analysis(&analysis);

    profile.badges.extend(new_badges);
    profile.total_badges = profile.badges.len() as u32;
    profile.reputation_score = calculate_reputation_score(&profile.badges);
    profile.last_github_sync = Some(ic_cdk::api::time());
    profile.updated_at = ic_cdk::api::time();

    let github_username = profile.github_username.clone();

    USER_PROFILES.with(|profiles| {
        profiles.borrow_mut().insert(user, profile);
    });

    GITHUB_ANALYSES.with(|cache| {
        cache.borrow_mut().insert(github_username, analysis);
    });

    Ok(format!("Force sync completed for user: {}", user.to_text()))
}

// Utility functions

async fn verify_authenticated(caller: Principal) -> Result<(), String> {
    let auth_canister = AUTH_CANISTER_ID.with(|id| *id.borrow())
        .ok_or("Auth canister not configured")?;

    let (is_auth,): (bool,) = ic_cdk::call(auth_canister, "is_authenticated", ())
        .await
        .map_err(|e| format!("Failed to verify authentication: {:?}", e))?;

    if !is_auth {
        return Err("Authentication required".to_string());
    }

    Ok(())
}

async fn mint_badge_nft(user: Principal, badge: &Badge) -> Result<u64, String> {
    let nft_canister = NFT_CANISTER_ID.with(|id| *id.borrow())
        .ok_or("NFT canister not configured")?;

    let metadata = serde_json::to_string(badge)
        .map_err(|e| format!("Failed to serialize badge: {}", e))?;

    let (token_id,): (u64,) = ic_cdk::call(nft_canister, "mint", (user, metadata))
        .await
        .map_err(|e| format!("Failed to mint NFT: {:?}", e))?;

    Ok(token_id)
}

fn is_admin() -> Result<(), String> {
    let caller = ic_cdk::caller();

    // Check if caller is admin (you'd implement proper admin checking)
    // For now, just verify they're authenticated
    if caller == Principal::anonymous() {
        return Err("Admin access required".to_string());
    }

    Ok(())
}

// Health and info endpoints
#[query]
fn health_check() -> String {
    "VeriFlair Backend with GitHub integration is healthy".to_string()
}

#[query]
fn get_stats() -> ProfileStats {
    let (total_users, github_connected, total_badges, total_analyses) = USER_PROFILES.with(|profiles| {
        let profiles = profiles.borrow();
        let total = profiles.len() as u64;
        let connected = profiles.iter().filter(|(_, p)| p.github_connected).count() as u64;
        let badges = profiles.iter().map(|(_, p)| p.badges.len() as u64).sum::<u64>();

        (total, connected, badges, 0u64)
    });

    let total_repo_analyses = GITHUB_ANALYSES.with(|analyses| analyses.borrow().len() as u64);

    let avg_reputation = if total_users > 0 {
        USER_PROFILES.with(|profiles| {
            let total_rep: u64 = profiles.borrow().iter()
                .map(|(_, p)| p.reputation_score)
                .sum();
            total_rep as f64 / total_users as f64
        })
    } else {
        0.0
    };

    ProfileStats {
        total_users,
        total_badges_earned: total_badges,
        total_repositories_analyzed: total_repo_analyses,
        average_reputation: avg_reputation,
        github_connected_users: github_connected,
        most_common_languages: vec![], // TODO: Implement
        badge_distribution: std::collections::HashMap::new(), // TODO: Implement
    }
}

#[query]
fn get_api_info() -> String {
    "VeriFlair Backend API v3.0 with GitHub Integration".to_string()
}

// CORS handling for frontend integration
#[query]
fn http_request(request: HttpRequest) -> HttpResponse {
    let headers = vec![
        HttpHeader {
            name: "Access-Control-Allow-Origin".to_string(),
            value: "*".to_string(), // In production, use specific domain
        },
        HttpHeader {
            name: "Access-Control-Allow-Methods".to_string(),
            value: "GET, POST, OPTIONS".to_string(),
        },
        HttpHeader {
            name: "Access-Control-Allow-Headers".to_string(),
            value: "Content-Type, Authorization".to_string(),
        },
    ];

    HttpResponse {
        status_code: 200,
        headers,
        body: "VeriFlair Backend API with GitHub Integration".as_bytes().to_vec(),
    }
}

ic_cdk::export_candid!();