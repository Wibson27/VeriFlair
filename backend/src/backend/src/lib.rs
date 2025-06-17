use candid::{CandidType, Principal};
use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod, HttpResponse, TransformArgs,
};
use ic_cdk::{init, post_upgrade, pre_upgrade, query, update};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap, StableVec};
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
fn init(auth_canister: Principal, nft_canister: Principal) {
    AUTH_CANISTER_ID.with(|id| *id.borrow_mut() = Some(auth_canister));
    NFT_CANISTER_ID.with(|id| *id.borrow_mut() = Some(nft_canister));
    
    // Setup periodic analysis timer (every 6 hours)
    ic_cdk_timers::set_timer_interval(
        std::time::Duration::from_secs(6 * 60 * 60),
        || ic_cdk::spawn(periodic_analysis())
    );
}

#[update]
async fn create_profile(github_username: String) -> Result<UserProfile, String> {
    let caller = ic_cdk::caller();
    
    // Verify authentication
    verify_authenticated(caller).await?;
    
    // Check if profile already exists
    if USER_PROFILES.with(|profiles| profiles.borrow().contains_key(&caller)) {
        return Err("Profile already exists".to_string());
    }

    // Validate GitHub username
    github::validate_github_user(&github_username).await?;

    let profile = UserProfile {
        principal: caller,
        github_username: github_username.clone(),
        badges: Vec::new(),
        reputation_score: 0,
        last_analysis: None,
        created_at: ic_cdk::api::time(),
        updated_at: ic_cdk::api::time(),
    };

    USER_PROFILES.with(|profiles| {
        profiles.borrow_mut().insert(caller, profile.clone());
    });

    // Trigger initial analysis
    ic_cdk::spawn(analyze_user_github(caller, github_username));

    Ok(profile)
}

#[update]
async fn trigger_analysis() -> Result<String, String> {
    let caller = ic_cdk::caller();
    
    let profile = USER_PROFILES.with(|profiles| {
        profiles.borrow().get(&caller)
    }).ok_or("Profile not found")?;

    // Rate limiting: max 1 analysis per hour per user
    if let Some(last_analysis) = profile.last_analysis {
        let now = ic_cdk::api::time();
        if now - last_analysis < 3600_000_000_000 { // 1 hour in nanoseconds
            return Err("Analysis can only be triggered once per hour".to_string());
        }
    }

    ic_cdk::spawn(analyze_user_github(caller, profile.github_username.clone()));
    
    Ok("Analysis triggered successfully".to_string())
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
            .map(|(_, profile)| profile)
            .collect();
        
        all_profiles.sort_by(|a, b| b.reputation_score.cmp(&a.reputation_score));
        all_profiles.truncate(limit);
        all_profiles
    })
}

async fn analyze_user_github(user: Principal, github_username: String) {
    match perform_github_analysis(&github_username).await {
        Ok(analysis) => {
            // Store analysis
            GITHUB_ANALYSES.with(|analyses| {
                analyses.borrow_mut().insert(github_username.clone(), analysis.clone());
            });

            // Generate badges based on analysis
            let new_badges = generate_badges_from_analysis(&analysis);
            
            // Update user profile
            USER_PROFILES.with(|profiles| {
                let mut profiles = profiles.borrow_mut();
                if let Some(mut profile) = profiles.get(&user) {
                    profile.badges.extend(new_badges.clone());
                    profile.reputation_score = calculate_reputation_score(&profile.badges);
                    profile.last_analysis = Some(ic_cdk::api::time());
                    profile.updated_at = ic_cdk::api::time();
                    profiles.insert(user, profile);
                }
            });

            // Mint new badges as NFTs
            for badge in new_badges {
                if let Err(e) = mint_badge_nft(user, &badge).await {
                    ic_cdk::println!("Failed to mint badge NFT: {}", e);
                }
            }
        }
        Err(e) => {
            ic_cdk::println!("GitHub analysis failed for {}: {}", github_username, e);
        }
    }
}

async fn perform_github_analysis(username: &str) -> Result<GitHubAnalysis, String> {
    // Fetch GitHub data
    let github_data = github::fetch_user_data(username).await?;
    
    // Analyze with LLM
    let llm_analysis = llm::analyze_github_data(&github_data).await?;
    
    Ok(GitHubAnalysis {
        username: username.to_string(),
        total_commits: github_data.total_commits,
        languages: github_data.languages,
        repositories: github_data.repositories,
        llm_insights: llm_analysis,
        analyzed_at: ic_cdk::api::time(),
    })
}

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

async fn periodic_analysis() {
    ic_cdk::println!("Starting periodic analysis for all users");
    
    let all_users: Vec<(Principal, String)> = USER_PROFILES.with(|profiles| {
        profiles
            .borrow()
            .iter()
            .map(|(principal, profile)| (principal, profile.github_username.clone()))
            .collect()
    });

    for (user, github_username) in all_users {
        // Add delay between analyses to avoid rate limits
        ic_cdk_timers::set_timer(
            std::time::Duration::from_secs(30),
            move || ic_cdk::spawn(analyze_user_github(user, github_username))
        );
    }
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
        body: "VeriFlair Backend API".as_bytes().to_vec(),
    }
}

ic_cdk::export_candid!();
```