mod github;
mod models;
// mod llm;
// mod utils;

// --- USE STATEMENTS ---
use crate::models::{Badge, BadgeCategory, BadgeTier, BadgeMetadata, GitHubUser, Repository, UserProfile};
use candid::Principal;
use futures::join;
use ic_cdk::api::management_canister::http_request::{HttpResponse, TransformArgs};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap};
use std::cell::RefCell;
use std::collections::HashMap;

// --- TIPE DATA & STATE MANAGEMENT ---
type Memory = VirtualMemory<DefaultMemoryImpl>;
type ProfileStore = StableBTreeMap<Principal, UserProfile, Memory>;

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    static USER_PROFILES: RefCell<ProfileStore> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0)))
        )
    );
}

// --- FUNGSI PUBLIK ---
#[ic_cdk::update]
async fn link_and_analyze_github(github_username: String) -> Result<UserProfile, String> {
    let caller = ic_cdk::caller();

    if github_username.is_empty() {
        return Err("GitHub username tidak boleh kosong.".to_string());
    }

    // Panggil API GitHub secara paralel
    let (profile_result, repos_result) = join!(
        github::fetch_user_profile(&github_username),
        github::fetch_user_repos(&github_username)
    );

    let github_profile = profile_result?;
    let github_repos = repos_result?;

    // Lakukan kalkulasi reputasi dan analisis bahasa
    let mut score: u32 = 0;
    let mut language_stats: HashMap<String, u64> = HashMap::new();

    score += github_profile.followers * 5;
    score += github_profile.public_repos * 10;

    for repo in github_repos {
        score += repo.stars * 2;
        if let Some(lang) = repo.language {
            *language_stats.entry(lang).or_insert(0) += 1;
        }
    }

    let top_language = language_stats.into_iter().max_by_key(|&(_, count)| count);

    // Buat atau perbarui profil pengguna
    let mut user_profile = USER_PROFILES.with(|p| p.borrow().get(&caller)).unwrap_or_else(|| UserProfile {
        Principal: caller,
        github_username: "".to_string(),
        badges: vec![],
        reputation_score: 0,
        last_analysis: None,
        created_at: ic_cdk::api::time(),
        updated_at: ic_cdk::api::time(),
    });

    user_profile.github_username = github_username;
    user_profile.reputation_score = score;
    user_profile.last_analysis = Some(ic_cdk::api::time());
    user_profile.updated_at = ic_cdk::api::time();

    if let Some((lang_name, _)) = top_language {
        let new_badge = Badge {
            id: format!("{}-expert-bronze", lang_name.to_lowercase()),
            name: format!("{} Expert", lang_name),
            description: format!("Demonstrated foundational skills in {} programming.", lang_name),
            category: BadgeCategory::Language(lang_name),
            tier: BadgeTier::Bronze,
            earned_at: ic_cdk::api::time(),
            metadata: BadgeMetadata { image_url: "".to_string(), animation_url: None, attributes: vec![] },
        };
        if !user_profile.badges.iter().any(|b| b.id == new_badge.id) {
            user_profile.badges.push(new_badge);
        }
    }

    // Simpan profil yang sudah diperbarui & kembalikan
    USER_PROFILES.with(|profiles| {
        profiles.borrow_mut().insert(caller, user_profile.clone());
    });

    Ok(user_profile)
}

ic_cdk::export_candid!();