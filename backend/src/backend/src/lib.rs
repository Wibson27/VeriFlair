mod github;
mod models;
// mod llm;
// mod utils;

// --- USE STATEMENTS ---
use crate::models::{GitHubUser, Repository, UserProfile}; // Impor semua model yang kita butuhkan
use candid::Principal;
use futures::join;
use ic_cdk::api::management_canister::http_request::{HttpResponse, TransformArgs};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap};
use std::cell::RefCell;

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
fn create_or_get_profile() -> UserProfile {
    let caller: Principal = ic_cdk::caller();

    USER_PROFILES.with(|profiles| {
        profiles.borrow().get(&caller)
    }).unwrap_or_else(|| { // Menggunakan unwrap_or_else agar lebih ringkas
        let new_profile = UserProfile {
            Principal: caller,
            github_username: "".to_string(),
            badges: Vec::new(), // <-- FIX: Baris ini tidak lagi dikomentari
            reputation_score: 0,
            last_analysis: None,
            created_at: ic_cdk::api::time(),
            updated_at: ic_cdk::api::time(),
        };
        USER_PROFILES.with(|profiles| {
            profiles.borrow_mut().insert(caller, new_profile.clone());
        });
        new_profile
    })
}

#[ic_cdk::update]
async fn link_and_analyze_github(github_username: String) -> Result<UserProfile, String> {
    let caller = ic_cdk::caller();

    if github_username.is_empty() {
        return Err("GitHub username tidak boleh kosong.".to_string());
    }

    // Langkah 1: Ambil profil user saat ini (atau buat jika belum ada)
    // Kita panggil saja fungsi yang sudah kita buat agar tidak duplikasi kode
    let mut user_profile = create_or_get_profile();

    // Langkah 2: Panggil API GitHub secara paralel
    let (profile_result, repos_result) = join!(
        github::fetch_user_profile(&github_username),
        github::fetch_user_repos(&github_username)
    );

    let github_profile = profile_result?;
    let github_repos = repos_result?;

    // Langkah 3: Lakukan kalkulasi reputasi sederhana
    let mut score: u32 = 0;
    score += github_profile.followers * 5;
    score += github_profile.public_repos * 10;
    for repo in github_repos {
        score += repo.stars * 2;
    }

    // Langkah 4: Perbarui profil user dengan data baru
    user_profile.github_username = github_username;
    user_profile.reputation_score = score;
    user_profile.last_analysis = Some(ic_cdk::api::time());
    user_profile.updated_at = ic_cdk::api::time();

    // Langkah 5: Simpan profil yang sudah diperbarui ke state
    USER_PROFILES.with(|profiles| {
        profiles.borrow_mut().insert(caller, user_profile.clone());
    });

    Ok(user_profile)
}

ic_cdk::export_candid!();