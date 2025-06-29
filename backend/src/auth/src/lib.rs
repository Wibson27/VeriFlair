use candid::{CandidType, Principal};
use ic_cdk::api::{time, msg_caller};
use ic_cdk::{init, post_upgrade, pre_upgrade, query, update};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap, Storable};
use std::borrow::Cow;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;

type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdStore = StableBTreeMap<Principal, UserSession, Memory>;

const USER_SESSIONS_MEMORY_ID: MemoryId = MemoryId::new(0);

// Session duration constants
const SESSION_DURATION_NS: u64 = 30 * 60 * 1_000_000_000; // 30 minutes
const MAX_SESSION_RENEWAL: u64 = 24 * 60 * 60 * 1_000_000_000; // 24 hours

#[derive(CandidType, Serialize, Deserialize, Clone)]
pub struct UserSession {
    pub user_principal: Principal,  // ← Updated field name to match Candid
    pub github_username: Option<String>,
    pub created_at: u64,
    pub last_active: u64,
    pub expires_at: u64,
    pub role: UserRole,
    pub is_verified: bool,
}

#[derive(CandidType, Serialize, Deserialize, Clone)]
pub enum UserRole {
    User,
    Admin,
    Moderator,
}

#[derive(CandidType, Serialize, Deserialize)]
pub enum AuthError {
    NotAuthenticated,
    SessionExpired,
    InvalidPrincipal,
    InternalError(String),
}

// Storable implementation for UserSession
impl Storable for UserSession {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(candid::encode_one(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        candid::decode_one(&bytes).unwrap()
    }

    const BOUND: ic_stable_structures::storable::Bound = ic_stable_structures::storable::Bound::Bounded {
        max_size: 1024, // Increased for expires_at field
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

// Internet Identity principal verification
fn require_authenticated_caller() -> Result<Principal, String> {
    let caller_principal = msg_caller();

    if caller_principal == Principal::anonymous() {
        return Err("Anonymous users cannot create sessions".to_string());
    }

    // Verify it's a self-authenticating principal (from Internet Identity)
    if !is_self_authenticating(&caller_principal) {
        return Err("Only Internet Identity authenticated users allowed".to_string());
    }

    Ok(caller_principal)
}

// Check if principal is self-authenticating (from II delegation)
fn is_self_authenticating(principal: &Principal) -> bool {
    let bytes = principal.as_slice();
    // Self-authenticating principals end with suffix byte 0x02
    bytes.len() > 0 && bytes[bytes.len() - 1] == 0x02
}

// Session validation with expiration check
fn validate_session_internal(principal: &Principal) -> Result<UserSession, AuthError> {
    USER_SESSIONS.with(|sessions| {
        let sessions = sessions.borrow();

        match sessions.get(principal) {
            Some(session) => {
                if !session.is_verified {
                    return Err(AuthError::NotAuthenticated);
                }

                let current_time = time();
                if current_time > session.expires_at {
                    return Err(AuthError::SessionExpired);
                }

                Ok(session)
            }
            None => Err(AuthError::NotAuthenticated),
        }
    })
}

#[init]
fn init() {
    ic_cdk::println!("VeriFlair Auth canister initialized for Internet Identity");
}

#[pre_upgrade]
fn pre_upgrade() {
    // Clear ephemeral sessions on upgrade
    USER_SESSIONS.with(|sessions| {
        let session_count = sessions.borrow().len();
        ic_cdk::println!("Pre-upgrade: {} active sessions will be cleared", session_count);
    });
}

#[post_upgrade]
fn post_upgrade() {
    ic_cdk::println!("VeriFlair Auth canister upgraded - sessions reset");
}

// Main Internet Identity authentication endpoint
#[update]
pub fn authenticate_user() -> Result<UserSession, String> {
    let principal = require_authenticated_caller()?;
    let current_time = time();

    let session = UserSession {
        user_principal: principal,
        github_username: None, // Will be set later when they connect GitHub
        created_at: current_time,
        last_active: current_time,
        expires_at: current_time + SESSION_DURATION_NS,
        role: UserRole::User,
        is_verified: true, // Verified through Internet Identity
    };

    USER_SESSIONS.with(|sessions| {
        sessions.borrow_mut().insert(principal, session.clone());
    });

    ic_cdk::println!("User authenticated via Internet Identity: {}", principal.to_text());
    Ok(session)
}

#[update]
fn create_test_session() -> Result<UserSession, String> {
    create_session(Some("test_user".to_string()))
}

// Create/update session (for authenticated users to set GitHub username)
#[update]
pub fn create_session(github_username: Option<String>) -> Result<UserSession, String> {
    let principal = require_authenticated_caller()?;

    // Check if user has an active session
    let mut session = USER_SESSIONS.with(|sessions| {
        let sessions = sessions.borrow();
        sessions.get(&principal)
    }).ok_or("No active session found. Please authenticate first.")?;

    // Validate session is not expired
    let current_time = time();
    if current_time > session.expires_at {
        USER_SESSIONS.with(|sessions| {
            sessions.borrow_mut().remove(&principal);
        });
        return Err("Session expired. Please authenticate again.".to_string());
    }

    // Update session with GitHub username
    session.github_username = github_username;
    session.last_active = current_time;

    USER_SESSIONS.with(|sessions| {
        sessions.borrow_mut().insert(principal, session.clone());
    });

    ic_cdk::println!("Session updated for user: {}", principal.to_text());
    Ok(session)
}

// Session renewal
#[update]
pub fn renew_session() -> Result<UserSession, String> {
    let principal = require_authenticated_caller()?;
    let current_time = time();

    USER_SESSIONS.with(|sessions| {
        let mut sessions = sessions.borrow_mut();

        match sessions.get(&principal) {
            Some(mut session) => {
                // Check if session can be renewed (within 24h of creation)
                if current_time > session.created_at + MAX_SESSION_RENEWAL {
                    sessions.remove(&principal);
                    return Err("Session cannot be renewed, please login again".to_string());
                }

                // Renew session
                session.expires_at = current_time + SESSION_DURATION_NS;
                session.last_active = current_time;
                sessions.insert(principal, session.clone());

                ic_cdk::println!("Session renewed for user: {}", principal.to_text());
                Ok(session)
            }
            None => Err("No session to renew".to_string()),
        }
    })
}

// Update last activity timestamp
#[update]
fn update_last_active() -> Result<(), String> {
    let principal = require_authenticated_caller()?;

    USER_SESSIONS.with(|sessions| {
        let mut sessions = sessions.borrow_mut();
        if let Some(mut session) = sessions.get(&principal) {
            let current_time = time();

            // Check if session is expired
            if current_time > session.expires_at {
                sessions.remove(&principal);
                return Err("Session expired".to_string());
            }

            session.last_active = current_time;
            sessions.insert(principal, session);
            Ok(())
        } else {
            Err("Session not found".to_string())
        }
    })
}

// Session validation for other canisters
#[query]
pub fn validate_session(user_principal: Principal) -> Result<UserSession, AuthError> {
    validate_session_internal(&user_principal)
}

// Get current user's session
#[query]
fn get_session() -> Option<UserSession> {
    let principal = msg_caller();

    // Return session only if it's not expired
    USER_SESSIONS.with(|sessions| {
        let sessions = sessions.borrow();
        if let Some(session) = sessions.get(&principal) {
            let current_time = time();
            if current_time <= session.expires_at && session.is_verified {
                Some(session)
            } else {
                None
            }
        } else {
            None
        }
    })
}

// Check if current caller is authenticated
#[query]
fn is_authenticated() -> bool {
    let principal = msg_caller();

    if principal == Principal::anonymous() {
        return false;
    }

    USER_SESSIONS.with(|sessions| {
        let sessions = sessions.borrow();
        if let Some(session) = sessions.get(&principal) {
            let current_time = time();
            session.is_verified && current_time <= session.expires_at
        } else {
            false
        }
    })
}

// Logout function
#[update]
pub fn logout() -> Result<String, String> {
    let principal = require_authenticated_caller()?;

    USER_SESSIONS.with(|sessions| {
        let mut sessions = sessions.borrow_mut();
        sessions.remove(&principal);
    });

    ic_cdk::println!("User logged out: {}", principal.to_text());
    Ok("Successfully logged out".to_string())
}

// Admin function: Set user role (requires admin privileges)
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

// Guard function: Check if caller is admin
fn is_admin() -> Result<(), String> {
    let principal = msg_caller();
    USER_SESSIONS.with(|sessions| {
        if let Some(session) = sessions.borrow().get(&principal) {
            let current_time = time();

            // Check session is valid and not expired
            if !session.is_verified || current_time > session.expires_at {
                return Err("Authentication required".to_string());
            }

            match session.role {
                UserRole::Admin => Ok(()),
                _ => Err("Admin access required".to_string()),
            }
        } else {
            Err("Authentication required".to_string())
        }
    })
}

// Health check for monitoring
#[query]
pub fn health_check() -> String {
    let (active_sessions, total_sessions) = USER_SESSIONS.with(|sessions| {
        let sessions = sessions.borrow();
        let current_time = time();

        let active = sessions.iter().filter(|(_, session)| {
            session.is_verified && current_time <= session.expires_at
        }).count();

        let total = sessions.len();

        (active, total)
    });

    format!(
        "VeriFlair Auth canister healthy. Active sessions: {} / Total sessions: {}",
        active_sessions, total_sessions
    )
}

// Development helper: Get all active sessions (remove in production)
#[query]
pub fn get_active_sessions_count() -> u64 {
    USER_SESSIONS.with(|sessions| {
        let sessions = sessions.borrow();
        let current_time = time();

        sessions.iter().filter(|(_, session)| {
            session.is_verified && current_time <= session.expires_at
        }).count() as u64
    })
}

// Development helper: Clean expired sessions
#[update]
pub fn cleanup_expired_sessions() -> Result<String, String> {
    let principal = require_authenticated_caller()?;

    // Only allow cleanup if user is admin or it's their own session
    let is_admin_user = USER_SESSIONS.with(|sessions| {
        sessions.borrow().get(&principal)
            .map(|session| matches!(session.role, UserRole::Admin))
            .unwrap_or(false)
    });

    if !is_admin_user {
        return Err("Only admins can cleanup expired sessions".to_string());
    }

    let removed_count = USER_SESSIONS.with(|sessions| {
        let mut sessions = sessions.borrow_mut();
        let current_time = time();
        let mut to_remove = Vec::new();

        for (principal, session) in sessions.iter() {
            if current_time > session.expires_at {
                to_remove.push(principal);
            }
        }

        for principal in &to_remove {
            sessions.remove(principal);
        }

        to_remove.len()
    });

    Ok(format!("Cleaned up {} expired sessions", removed_count))
}

// Export Candid interface
ic_cdk::export_candid!();