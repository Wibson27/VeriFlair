mod github;
mod models;
// mod llm;
// mod utils;

// BLOK USE STATEMENT YANG SUDAH DIBERSIHKAN
use crate::models::{Repository, UserProfile, GitHubUser};
use candid::Principal;
use ic_cdk::api::management_canister::http_request::{HttpResponse, TransformArgs};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap};
use std::cell::RefCell;

// Mendefinisikan tipe data untuk memori dan penyimpanan
type Memory = VirtualMemory<DefaultMemoryImpl>;
type ProfileStore = StableBTreeMap<Principal, UserProfile, Memory>;

// Untuk manage memori dan simpan profil di 'thread_local'
thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    // Mendefinisikan tempat penyimpanan untuk PROFIL PENGGUNA
    static USER_PROFILES: RefCell<ProfileStore> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0)))
        )
    );
}

// Hapus fungsi-fungsi tes lama agar lebih bersih
// #[ic_cdk::update]
// async fn get_user_repositories_test...
// #[ic_cdk::update]
// async fn get_user_profile_test...

#[ic_cdk::update]
fn create_or_get_profile() -> UserProfile {
    let caller: Principal = ic_cdk::caller();

    let profile = USER_PROFILES.with(|profiles| {
        profiles.borrow().get(&caller)
    });

    match profile {
        Some(p) => p,
        None => {
            let new_profile = UserProfile {
                principal: caller,
                github_username: "".to_string(),
                //badges: Vec::new(),
                reputation_score: 0,
                last_analysis: None,
                created_at: ic_cdk::api::time(),
                updated_at: ic_cdk::api::time(),
            };
            USER_PROFILES.with(|profiles| {
                profiles.borrow_mut().insert(caller, new_profile.clone());
            });
            new_profile
        }
    }
}

ic_cdk::export_candid!();