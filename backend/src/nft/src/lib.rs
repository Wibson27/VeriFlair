use candid::{CandidType, Nat, Principal};
use ic_cdk::{init, post_upgrade, pre_upgrade, query, update};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap, Storable};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::HashMap;

// Helper function to safely convert Nat to u64
fn nat_to_u64(nat: &Nat) -> u64 {
    let nat_str = nat.to_string();
    nat_str.parse::<u64>().unwrap_or(0)
}

type Memory = VirtualMemory<DefaultMemoryImpl>;
type TokenStore = StableBTreeMap<u64, TokenMetadata, Memory>;
type OwnerStore = StableBTreeMap<u64, Principal, Memory>;
type BalanceStore = StableBTreeMap<Principal, u64, Memory>;

const TOKENS_MEMORY_ID: MemoryId = MemoryId::new(0);
const OWNERS_MEMORY_ID: MemoryId = MemoryId::new(1);
const BALANCES_MEMORY_ID: MemoryId = MemoryId::new(2);

#[derive(CandidType, Serialize, Deserialize, Clone)]
pub struct TokenMetadata {
    pub token_id: u64,
    pub name: String,
    pub description: String,
    pub image: String,
    pub attributes: Vec<Attribute>,
    pub created_at: u64,
}

impl Storable for TokenMetadata {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(candid::encode_one(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        candid::decode_one(&bytes).unwrap()
    }

    const BOUND: ic_stable_structures::storable::Bound = ic_stable_structures::storable::Bound::Bounded {
        max_size: 2048,
        is_fixed_size: false,
    };
}

#[derive(CandidType, Serialize, Deserialize, Clone)]
pub struct Attribute {
    pub name: String,
    pub value: String,
}

#[derive(CandidType, Serialize, Deserialize)]
pub struct TransferArgs {
    pub from: Principal,
    pub to: Principal,
    pub token_id: u64,
}

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    static TOKENS: RefCell<TokenStore> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(TOKENS_MEMORY_ID)),
        )
    );

    static OWNERS: RefCell<OwnerStore> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(OWNERS_MEMORY_ID)),
        )
    );

    static BALANCES: RefCell<BalanceStore> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(BALANCES_MEMORY_ID)),
        )
    );

    static NEXT_TOKEN_ID: RefCell<u64> = RefCell::new(1);
    static BACKEND_CANISTER_ID: RefCell<Option<Principal>> = RefCell::new(None);
    static COLLECTION_NAME: RefCell<String> = RefCell::new("VeriFlair Badges".to_string());
    static COLLECTION_SYMBOL: RefCell<String> = RefCell::new("VFB".to_string());
}

#[init]
fn init() {
    // Initialize without backend canister - will be set later
    ic_cdk::println!("VeriFlair NFT canister initialized");
}

// Function to set backend canister after deployment
#[update]
fn set_backend_canister(backend_canister: Principal) -> Result<(), String> {
    let caller = ic_cdk::caller();

    // Only allow setting once, and only by controller/admin
    if BACKEND_CANISTER_ID.with(|id| id.borrow().is_some()) {
        return Err("Backend canister already set".to_string());
    }

    BACKEND_CANISTER_ID.with(|id| *id.borrow_mut() = Some(backend_canister));
    ic_cdk::println!("Backend canister set to: {}", backend_canister.to_text());
    Ok(())
}

// ICRC-7 Standard Implementation

#[query]
fn icrc7_collection_metadata() -> Vec<(String, String)> {
    let mut metadata = Vec::new();

    COLLECTION_NAME.with(|name| {
        metadata.push(("icrc7:name".to_string(), name.borrow().clone()));
    });

    COLLECTION_SYMBOL.with(|symbol| {
        metadata.push(("icrc7:symbol".to_string(), symbol.borrow().clone()));
    });

    metadata.push(("icrc7:description".to_string(),
        "Soulbound NFT badges representing verified developer skills and achievements".to_string()));

    metadata.push(("icrc7:max_supply".to_string(),
        u64::MAX.to_string()));

    metadata.push(("icrc7:supply_cap".to_string(),
        u64::MAX.to_string()));

    metadata.push(("icrc7:transfer_restrictions".to_string(),
        "Soulbound - transfers not allowed".to_string()));

    metadata
}

#[query]
fn icrc7_total_supply() -> Nat {
    NEXT_TOKEN_ID.with(|id| Nat::from(*id.borrow() - 1))
}

#[query]
fn icrc7_supply_cap() -> Option<Nat> {
    Some(Nat::from(u64::MAX))
}

#[query]
fn icrc7_max_query_batch_size() -> Option<Nat> {
    Some(Nat::from(1000u32))
}

#[query]
fn icrc7_max_update_batch_size() -> Option<Nat> {
    Some(Nat::from(100u32))
}

#[query]
fn icrc7_max_take_value() -> Option<Nat> {
    Some(Nat::from(1000u32))
}

#[query]
fn icrc7_default_take_value() -> Option<Nat> {
    Some(Nat::from(100u32))
}

#[query]
fn icrc7_permitted_drift() -> Option<Nat> {
    Some(Nat::from(60u32)) // 60 seconds
}

#[query]
fn icrc7_tx_window() -> Option<Nat> {
    Some(Nat::from(86400u32)) // 24 hours
}

#[query]
fn icrc7_balance_of(account: Principal) -> Nat {
    BALANCES.with(|balances| {
        Nat::from(balances.borrow().get(&account).unwrap_or(0))
    })
}

#[query]
fn icrc7_owner_of(token_ids: Vec<Nat>) -> Vec<Option<Principal>> {
    OWNERS.with(|owners| {
        token_ids.iter().map(|token_id| {
            // Convert Nat to u64 safely
            let id: u64 = nat_to_u64(token_id);
            owners.borrow().get(&id)
        }).collect()
    })
}

#[query]
fn icrc7_token_metadata(token_ids: Vec<Nat>) -> Vec<Option<Vec<(String, String)>>> {
    TOKENS.with(|tokens| {
        token_ids.iter().map(|token_id| {
            let id: u64 = nat_to_u64(token_id);
            tokens.borrow().get(&id).map(|metadata| {
                let mut map = Vec::new();
                map.push(("name".to_string(), metadata.name.clone()));
                map.push(("description".to_string(), metadata.description.clone()));
                map.push(("image".to_string(), metadata.image.clone()));

                // Convert attributes to simple string format
                for attr in &metadata.attributes {
                    map.push((format!("attr_{}", attr.name), attr.value.clone()));
                }

                map.push(("token_id".to_string(), metadata.token_id.to_string()));
                map.push(("created_at".to_string(), metadata.created_at.to_string()));

                map
            })
        }).collect()
    })
}

#[query]
fn icrc7_tokens_of(account: Principal, prev: Option<Nat>, take: Option<Nat>) -> Vec<Nat> {
    let take_value = take.unwrap_or(Nat::from(100u32));
    let take_usize: usize = nat_to_u64(&take_value) as usize;

    OWNERS.with(|owners| {
        let mut user_tokens = Vec::new();
        let start_from = prev.map(|p| nat_to_u64(&p)).unwrap_or(0u64);

        for (token_id, owner) in owners.borrow().range((start_from + 1)..) {
            if user_tokens.len() >= take_usize {
                break;
            }
            if owner == account {
                user_tokens.push(Nat::from(token_id));
            }
        }

        user_tokens
    })
}

// Soulbound: Transfers are not allowed
#[update]
fn icrc7_transfer(_args: Vec<TransferArgs>) -> Vec<Option<String>> {
    // All transfers return error as these are soulbound tokens
    _args.iter().map(|_| {
        Some("Soulbound tokens cannot be transferred".to_string())
    }).collect()
}

// Custom minting function (only callable by backend canister)
#[update(guard = "is_backend_canister")]
fn mint(to: Principal, metadata_json: String) -> Result<u64, String> {
    let metadata: TokenMetadata = serde_json::from_str(&metadata_json)
        .map_err(|e| format!("Invalid metadata JSON: {}", e))?;

    let token_id = NEXT_TOKEN_ID.with(|id| {
        let current = *id.borrow();
        *id.borrow_mut() = current + 1;
        current
    });

    let token_metadata = TokenMetadata {
        token_id,
        name: metadata.name,
        description: metadata.description,
        image: metadata.image,
        attributes: metadata.attributes,
        created_at: ic_cdk::api::time(),
    };

    // Store token metadata
    TOKENS.with(|tokens| {
        tokens.borrow_mut().insert(token_id, token_metadata);
    });

    // Set owner
    OWNERS.with(|owners| {
        owners.borrow_mut().insert(token_id, to);
    });

    // Update balance
    BALANCES.with(|balances| {
        let mut balances = balances.borrow_mut();
        let current_balance = balances.get(&to).unwrap_or(0);
        balances.insert(to, current_balance + 1);
    });

    Ok(token_id)
}

fn is_backend_canister() -> Result<(), String> {
    let backend_canister = BACKEND_CANISTER_ID.with(|id| *id.borrow())
        .ok_or("Backend canister not configured")?;

    if ic_cdk::caller() == backend_canister {
        Ok(())
    } else {
        Err("Only backend canister can mint tokens".to_string())
    }
}

#[query]
fn get_user_badges(user: Principal) -> Vec<TokenMetadata> {
    let token_ids = icrc7_tokens_of(user, None, Some(Nat::from(1000u32)));

    TOKENS.with(|tokens| {
        token_ids.iter().filter_map(|token_id| {
            let id: u64 = nat_to_u64(token_id);
            tokens.borrow().get(&id)
        }).collect()
    })
}

#[query]
fn health_check() -> String {
    "VeriFlair NFT canister is healthy".to_string()
}

// Canister lifecycle
#[pre_upgrade]
fn pre_upgrade() {
    ic_cdk::println!("Preparing NFT canister for upgrade...");
}

#[post_upgrade]
fn post_upgrade() {
    ic_cdk::println!("NFT canister upgrade completed");
}

ic_cdk::export_candid!();