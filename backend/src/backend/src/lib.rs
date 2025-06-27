mod github;
mod models;

use crate::models::{Badge, BadgeCategory, BadgeTier, BadgeMetadata, GitHubUser, Repository, UserProfile};
use candid::Principal;
use futures::join;
use ic_cdk::api::management_canister::http_request::{HttpResponse, TransformArgs};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap};
use std::cell::RefCell;
use std::collections::HashMap;


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

static OAUTH_CLIENT_ID: RefCell<String> = RefCell::new(String::new());
static OAUTH_CLIENT_SECRET: RefCell<String> = RefCell::new(String::new());
static NFT_CANISTER_ID: RefCell<Principal> = RefCell::new(Principal::anonymous());

#[ic_cdk::init]
fn init(client_id: String, client_secret: String, nft_canister_id: Principal) {
    OAUTH_CLIENT_ID.with(|id| {
        *id.borrow_mut() = client_id;
    });
    OAUTH_CLIENT_SECRET.with(|secret| {
        *secret.borrow_mut() = client_secret;
    });
    // Simpan ID canister NFT
    NFT_CANISTER_ID.with(|id| {
        *id.borrow_mut() = nft_canister_id;
    });
}

#[ic_cdk::update]
async fn link_and_analyze_github(github_username: String) -> Result<UserProfile, String> {
    let caller = ic_cdk::caller();

    if github_username.is_empty() {
        return Err("GitHub username tidak boleh kosong.".to_string());
    }

    // buat call API GitHub
    let (profile_result, repos_result) = join!(
        github::fetch_user_profile(&github_username),
        github::fetch_user_repos(&github_username)
    );

    let github_profile = profile_result?;
    let github_repos = repos_result?;

    // untuk hitung reputasi dan analisis bahasa
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

    // buat atau update profil user
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
            let nft_canister = NFT_CANISTER_ID.with(|id| *id.borrow());

            // Ubah metadata badge menjadi format JSON (text) sesuai permintaan fungsi mint di canister NFT
            let metadata_json = serde_json::to_string(&new_badge.metadata).expect("Failed to serialize badge metadata.");

            // Lakukan panggilan ke fungsi 'mint' di canister NFT
            let mint_result: Result<(u64,), _> = ic_cdk::call(
                nft_canister,
                "mint", // Nama fungsi di canister NFT
                (caller, metadata_json) // Argumen yang dikirim: Principal user dan metadata
            ).await;

            if let Ok((token_id,)) = mint_result {
                ic_cdk::println!("Successfully minted badge for user {} with token ID {}", caller, token_id);
                // Hanya tambahkan badge ke profil JIKA minting berhasil
                user_profile.badges.push(new_badge);
            } else {
                // Jika minting gagal, kita bisa mencatat errornya
                ic_cdk::println!("Failed to mint badge for user {}: {:?}", caller, mint_result.err());
            }
        }
    }

    // simpan profil yang sudah diupdate
    USER_PROFILES.with(|profiles| {
        profiles.borrow_mut().insert(caller, user_profile.clone());
    });

    Ok(user_profile)
}

#[ic_cdk::query]
fn get_github_oauth_client_id() -> String {
    OAUTH_CLIENT_ID.with(|id| id.borrow().clone())
}

ic_cdk::export_candid!();