use candid::{CandidType, Nat, Principal};
use ic_cdk::{init, post_upgrade, pre_upgrade, query, update};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::HashMap;

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
fn init(backend_canister: Principal) {
    BACKEND_CANISTER_ID.with(|id| *id.borrow_mut() = Some(backend_canister));
}

// ICRC-7 Standard Implementation

#[query]
fn icrc7_collection_metadata() -> Vec<(String, candid::types::Value)> {
    vec![
        ("icrc7:name".to_string(), candid::types::Value::Text(
            COLLECTION_NAME.with(|name| name.borrow().clone())
        )),
        ("icrc7:symbol".to_string(), candid::types::Value::Text(
            COLLECTION_SYMBOL.with(|symbol| symbol.borrow().clone())
        )),
        ("icrc7:description".to_string(), candid::types::Value::Text(
            "Verifiable developer achievement badges".to_string()
        )),
        ("icrc7:total_supply".to_string(), candid::types::Value::Nat(
            Nat::from(NEXT_TOKEN_ID.with(|id| *id.borrow() - 1))
        )),
    ]
}

#[query]
fn icrc7_name() -> String {
    COLLECTION_NAME.with(|name| name.borrow().clone())
}

#[query]
fn icrc7_symbol() -> String {
    COLLECTION_SYMBOL.with(|symbol| symbol.borrow().clone())
}

#[query]
fn icrc7_total_supply() -> Nat {
    Nat::from(NEXT_TOKEN_ID.with(|id| *id.borrow() - 1))
}

#[query]
fn icrc7_supply_cap() -> Option<Nat> {
    None // No supply cap
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
fn icrc7_default_take_value() -> Option<Nat> {
    Some(Nat::from(100u32))
}

#[query]
fn icrc7_max_take_value() -> Option<Nat> {
    Some(Nat::from(1000u32))
}

#[query]
fn icrc7_max_memo_size() -> Option<Nat> {
    Some(Nat::from(256u32))
}

#[query]
fn icrc7_atomic_batch_transfers() -> Option<bool> {
    Some(false)
}

#[query]
fn icrc7_tx_window() -> Option<Nat> {
    Some(Nat::from(86400u32)) // 24 hours
}

#[query]
fn icrc7_permitted_drift() -> Option<Nat> {
    Some(Nat::from(300u32)) // 5 minutes
}

#[query]
fn icrc7_owner_of(token_ids: Vec<u64>) -> Vec<Option<Principal>> {
    OWNERS.with(|owners| {
        token_ids.iter().map(|&token_id| {
            owners.borrow().get(&token_id)
        }).collect()
    })
}

#[query]
fn icrc7_balance_of(accounts: Vec<Principal>) -> Vec<Nat> {
    BALANCES.with(|balances| {
        accounts.iter().map(|&account| {
            Nat::from(balances.borrow().get(&account).unwrap_or(0))
        }).collect()
    })
}

#[query]
fn icrc7_tokens(prev: Option<u64>, take: Option<u64>) -> Vec<u64> {
    let start = prev.unwrap_or(0) + 1;
    let limit = take.unwrap_or(100).min(1000);
    let max_token_id = NEXT_TOKEN_ID.with(|id| *id.borrow());

    (start..max_token_id)
        .take(limit as usize)
        .collect()
}

#[query]
fn icrc7_tokens_of(account: Principal, prev: Option<u64>, take: Option<u64>) -> Vec<u64> {
    let limit = take.unwrap_or(100).min(1000);
    let mut tokens = Vec::new();
    let mut found = 0;
    let start_from = prev.unwrap_or(0);

    OWNERS.with(|owners| {
        let owners = owners.borrow();
        for (token_id, owner) in owners.iter() {
            if owner == account && token_id > start_from {
                tokens.push(token_id);
                found += 1;
                if found >= limit {
                    break;
                }
            }
        }
    });

    tokens
}

#[query]
fn icrc7_token_metadata(token_ids: Vec<u64>) -> Vec<Option<HashMap<String, candid::types::Value>>> {
    TOKENS.with(|tokens| {
        token_ids.iter().map(|&token_id| {
            tokens.borrow().get(&token_id).map(|metadata| {
                let mut map = HashMap::new();
                map.insert("name".to_string(), candid::types::Value::Text(metadata.name));
                map.insert("description".to_string(), candid::types::Value::Text(metadata.description));
                map.insert("image".to_string(), candid::types::Value::Text(metadata.image));

                let attrs: Vec<candid::types::Value> = metadata.attributes.iter().map(|attr| {
                    let mut attr_map = HashMap::new();
                    attr_map.insert("name".to_string(), candid::types::Value::Text(attr.name.clone()));
                    attr_map.insert("value".to_string(), candid::types::Value::Text(attr.value.clone()));
                    candid::types::Value::Record(attr_map)
                }).collect();

                map.insert("attributes".to_string(), candid::types::Value::Array(attrs));
                map
            })
        }).collect()
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
    let token_ids = icrc7_tokens_of(user, None, Some(1000));

    TOKENS.with(|tokens| {
        token_ids.iter().filter_map(|&token_id| {
            tokens.borrow().get(&token_id)
        }).collect()
    })
}

ic_cdk::export_candid!();