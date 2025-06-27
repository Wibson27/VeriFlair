mod github;
mod models;
// mod llm;
// mod utils;

// --- USE STATEMENTS ---
use crate::models::{Badge, BadgeAttribute, BadgeCategory, BadgeMetadata, BadgeTier, GitHubUser, Repository, UserProfile};
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

    // --- Langkah 1: Validasi Input & Ambil Profil Pengguna ---
    if github_username.is_empty() {
        return Err("GitHub username tidak boleh kosong.".to_string());
    }

    // Mengambil profil pengguna yang ada, atau membuat yang baru jika ini pertama kalinya.
    // Ini memastikan kita selalu bekerja dengan data yang benar.
    let mut user_profile = create_or_get_profile();

    // --- Langkah 2: Ambil Data dari API GitHub secara Paralel ---
    let (profile_result, repos_result) = futures::join!(
        github::fetch_user_profile(&github_username),
        github::fetch_user_repos(&github_username)
    );

    // Cek apakah pengambilan data berhasil. Tanda tanya (?) akan otomatis mengembalikan error jika ada.
    let github_profile = profile_result?;
    let github_repos = repos_result?;

    // --- Langkah 3: Lakukan Kalkulasi & Analisis Data ---
    let mut score: u32 = 0;
    let mut language_stats: std::collections::HashMap<String, u64> = std::collections::HashMap::new();

    // Kalkulasi skor dari data profil
    score += github_profile.followers * 5;
    score += github_profile.public_repos * 10;

    // Kalkulasi skor dan analisis bahasa dari setiap repositori
    for repo in github_repos {
        score += repo.stars * 2;
        if let Some(lang) = repo.language {
            // Menghitung penggunaan setiap bahasa
            *language_stats.entry(lang).or_insert(0) += 1;
        }
    }

    // Cari bahasa yang paling sering digunakan
    let top_language = language_stats.into_iter().max_by_key(|&(_, count)| count);

    // --- Langkah 4: Perbarui Profil Pengguna dengan Hasil Analisis ---
    user_profile.github_username = github_username.clone();
    user_profile.reputation_score = score;
    user_profile.last_analysis = Some(ic_cdk::api::time());
    user_profile.updated_at = ic_cdk::api::time();

    // Jika ada bahasa teratas, buatkan lencana (badge) pertama!
    if let Some((lang_name, _)) = top_language {
        let new_badge = crate::models::Badge {
            id: format!("{}-expert-bronze", lang_name.to_lowercase()),
            name: format!("{} Expert", lang_name),
            description: format!("Demonstrated foundational skills in {} programming.", lang_name),
            category: crate::models::BadgeCategory::Language(lang_name),
            tier: crate::models::BadgeTier::Bronze,
            earned_at: ic_cdk::api::time(),
            metadata: crate::models::BadgeMetadata {
                image_url: "".to_string(), // URL gambar bisa diisi nanti oleh tim frontend
                animation_url: None,
                attributes: vec![],
            },
        };

        // Hindari duplikasi badge, hanya tambahkan jika belum ada
        if !user_profile.badges.iter().any(|b| b.id == new_badge.id) {
            user_profile.badges.push(new_badge);
        }
    }

    // --- Langkah 5: Simpan Profil yang Sudah Diperbarui ke Memori Stabil ---
    USER_PROFILES.with(|profiles| {
        profiles.borrow_mut().insert(caller, user_profile.clone());
    });

    // --- Langkah 6: Kembalikan Profil yang Sudah Lengkap ---
    Ok(user_profile)
}

ic_cdk::export_candid!();