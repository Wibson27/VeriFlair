use candid::{CandidType, Principal};
use ic_cdk::api::time;
use ic_cdk::{init, post_upgrade, pre_upgrade, query, update};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap, Storable};
use std::borrow::Cow;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;

type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdStore = StableBTreeMap<Principal, UserSession, Memory>;

const USER_SESSIONS_MEMORY_ID: MemoryId = MemoryId::new(0);

#[derive(CandidType, Serialize, Deserialize, Clone)]
pub struct UserSession {
    pub principal: Principal,
    pub github_username: Option<String>,
    pub created_at: u64,
    pub last_active: u64,
    pub role: UserRole,
    pub is_verified: bool,
}

#[derive(CandidType, Serialize, Deserialize, Clone)]
pub enum UserRole {
    User,
    Admin,
    Moderator,
}

// Correct Storable implementation for ic-stable-structures 0.6
impl Storable for UserSession {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(candid::encode_one(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        candid::decode_one(&bytes).unwrap()
    }

    const BOUND: ic_stable_structures::storable::Bound = ic_stable_structures::storable::Bound::Bounded {
        max_size: 512,
        is_fixed_size: false,
    };
}

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    static USER_SESSIONS: RefCell<IdStore> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(USER_SESSIONS_MEMORY_ID)),
        )
    );
}

#[init]
fn init() {
    ic_cdk::println!("Auth canister initialized");
}

#[pre_upgrade]
fn pre_upgrade() {
    // Stable storage is automatically handled
}

#[post_upgrade]
fn post_upgrade() {
    ic_cdk::println!("Auth canister upgraded");
}

#[update]
fn create_session(github_username: Option<String>) -> Result<UserSession, String> {
    let caller = ic_cdk::caller();

    if caller == Principal::anonymous() {
        return Err("Anonymous users cannot create sessions".to_string());
    }

    let now = time();
    let session = UserSession {
        principal: caller,
        github_username,
        created_at: now,
        last_active: now,
        role: UserRole::User,
        is_verified: false,
    };

    USER_SESSIONS.with(|sessions| {
        sessions.borrow_mut().insert(caller, session.clone());
    });

    Ok(session)
}

#[update]
fn update_last_active() -> Result<(), String> {
    let caller = ic_cdk::caller();

    USER_SESSIONS.with(|sessions| {
        let mut sessions = sessions.borrow_mut();
        if let Some(mut session) = sessions.get(&caller) {
            session.last_active = time();
            sessions.insert(caller, session);
            Ok(())
        } else {
            Err("Session not found".to_string())
        }
    })
}

#[query]
fn get_session() -> Option<UserSession> {
    let caller = ic_cdk::caller();
    USER_SESSIONS.with(|sessions| {
        sessions.borrow().get(&caller)
    })
}

#[query]
fn is_authenticated() -> bool {
    let caller = ic_cdk::caller();
    caller != Principal::anonymous() &&
    USER_SESSIONS.with(|sessions| {
        sessions.borrow().contains_key(&caller)
    })
}

#[update(guard = "is_admin")]
fn set_user_role(user: Principal, role: UserRole) -> Result<(), String> {
    USER_SESSIONS.with(|sessions| {
        let mut sessions = sessions.borrow_mut();
        if let Some(mut session) = sessions.get(&user) {
            session.role = role;
            sessions.insert(user, session);
            Ok(())
        } else {
            Err("User session not found".to_string())
        }
    })
}

fn is_admin() -> Result<(), String> {
    let caller = ic_cdk::caller();
    USER_SESSIONS.with(|sessions| {
        if let Some(session) = sessions.borrow().get(&caller) {
            match session.role {
                UserRole::Admin => Ok(()),
                _ => Err("Admin access required".to_string()),
            }
        } else {
            Err("Authentication required".to_string())
        }
    })
}

// Export Candid interface
ic_cdk::export_candid!();