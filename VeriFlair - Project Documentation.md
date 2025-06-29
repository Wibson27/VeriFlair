# VeriFlair: Complete Implementation & Technical Design Guide v3.0

**Version:** 3.0  
**Status:** Production Ready  
**Last Updated:** June 2025  

## Purpose
This document serves as the definitive single source of truth for VeriFlair development. It provides comprehensive guidance for all team members, from initial development setup through production deployment and maintenance.

---

## Table of Contents

1. [Project Overview & Architecture](#1-project-overview--architecture)
2. [Development Environment Setup](#2-development-environment-setup)
3. [Backend Implementation Guide](#3-backend-implementation-guide)
4. [Frontend Implementation Guide](#4-frontend-implementation-guide)
5. [Security & Authentication](#5-security--authentication)
6. [Testing Strategies](#6-testing-strategies)
7. [Deployment & DevOps](#7-deployment--devops)
8. [Monitoring & Maintenance](#8-monitoring--maintenance)
9. [Troubleshooting Guide](#9-troubleshooting-guide)

---

## 1. Project Overview & Architecture

### 1.1 Vision Statement

VeriFlair is a gamified, on-chain reputation system that automatically analyzes developers' GitHub contributions and awards verifiable Soulbound NFT badges. These badges create a trusted, collectible professional identity that showcases technical expertise across different programming languages and contribution types.

### 1.2 Core Features

- **Automated GitHub Analysis**: AI-powered evaluation of code contributions, commit quality, and project involvement
- **Dynamic Badge System**: Tiered badges (Bronze, Silver, Gold, Platinum) across multiple categories (Rust Expert, Python Guru, Open Source Champion, etc.)
- **Soulbound NFTs**: Non-transferable on-chain credentials using ICRC-7 standard
- **Portfolio Integration**: Public profiles showcasing verified achievements
- **Reputation Scoring**: Comprehensive developer scoring system

### 1.3 Technical Architecture

#### Hybrid Web2/Web3 Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                        Client Layer                              │
├─────────────────────────────────────────────────────────────────┤
│ React Frontend (Web2)                                            │
│ ├── Vercel/Netlify Hosting                                       │
│ ├── Internet Identity Authentication                             │
│ ├── @dfinity/agent for ICP Communication                         │
│ └── Progressive Web App (PWA) Support                            │
└─────────────────────────────────────────────────────────────────┘
                                │
                                ▼ (HTTPS + Agent-JS)
┌─────────────────────────────────────────────────────────────────┐
│                    Internet Computer (ICP)                       │
├─────────────────────────────────────────────────────────────────┤
│ ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐   │
│ │  Auth Canister  │  │ Backend Canister│  │  NFT Canister   │   │
│ │                 │  │                 │  │                 │   │
│ │ • II Integration│  │ • GitHub API    │  │ • ICRC-7 NFTs   │   │
│ │ • Session Mgmt  │  │ • LLM Analysis  │  │ • Badge Minting │   │
│ │ • Role Control  │  │ • User Profiles │  │ • Metadata      │   │
│ └─────────────────┘  │ • Badge Logic   │  │ • Ownership     │   │
│                      │ • CORS Handling │  └─────────────────┘   │
│                      └─────────────────┘                        │
└─────────────────────────────────────────────────────────────────┘
                                │
                                ▼ (HTTPS Outcalls)
┌─────────────────────────────────────────────────────────────────┐
│                      External Services                           │
├─────────────────────────────────────────────────────────────────┤
│ • GitHub API (Code Analysis)                                     │
│ • OpenAI/Anthropic (AI Analysis)                                 │
│ • IPFS (Metadata Storage)                                        │
│ • Analytics Services                                             │
└─────────────────────────────────────────────────────────────────┘
```

#### Key Architecture Decisions

1. **Hybrid Approach**: Web2 frontend for UX, Web3 backend for trust and ownership
2. **Multi-Canister Design**: Separation of concerns for better security and scalability
3. **Soulbound Tokens**: Using ICRC-7 standard for non-transferable achievements
4. **AI Integration**: LLM-powered analysis for nuanced code evaluation
5. **Progressive Enhancement**: Core features work without wallet, enhanced features with authentication

---

## 2. Development Environment Setup

### 2.1 Prerequisites

#### Required Software
```bash
# Node.js (v18+)
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash
nvm install 18
nvm use 18

# Rust (latest stable)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup target add wasm32-unknown-unknown

# DFX SDK (latest)
sh -ci "$(curl -fsSL https://internetcomputer.org/install.sh)"

# Git and development tools
sudo apt-get install git build-essential pkg-config libssl-dev
```

#### Optional but Recommended
```bash
# Vessel (Package manager for Motoko - if using Motoko)
npm install -g vessel

# ic-repl (Testing tool)
npm install -g ic-repl

# Development tools
npm install -g typescript ts-node nodemon
```

### 2.2 Project Structure

```
veriflair/
├── backend/                    # ICP Canisters
│   ├── .dfx/                  # DFX build artifacts
│   ├── .vessel/               # Vessel cache (if using Motoko)
│   ├── src/
│   │   ├── auth/              # Authentication canister
│   │   │   ├── src/
│   │   │   │   └── lib.rs
│   │   │   ├── Cargo.toml
│   │   │   └── auth.did
│   │   ├── backend/           # Main backend canister
│   │   │   ├── src/
│   │   │   │   ├── lib.rs
│   │   │   │   ├── github/
│   │   │   │   ├── llm/
│   │   │   │   ├── models/
│   │   │   │   └── utils/
│   │   │   ├── Cargo.toml
│   │   │   └── backend.did
│   │   └── nft/               # NFT/Badge canister
│   │       ├── src/
│   │       │   └── lib.rs
│   │       ├── Cargo.toml
│   │       └── nft.did
│   ├── dfx.json
│   ├── Cargo.toml             # Workspace configuration
│   └── scripts/               # Deployment and utility scripts
├── frontend/                  # React Application
│   ├── public/
│   │   ├── assets/
│   │   │   ├── badges/        # Badge SVG components
│   │   │   ├── icons/         # Tech stack icons
│   │   │   └── frames/        # Badge frame designs
│   │   └── manifest.json      # PWA manifest
│   ├── src/
│   │   ├── components/        # Reusable components
│   │   ├── pages/             # Page components
│   │   ├── hooks/             # Custom React hooks
│   │   ├── services/          # API and ICP integration
│   │   ├── store/             # State management
│   │   ├── types/             # TypeScript definitions
│   │   ├── utils/             # Utility functions
│   │   └── styles/            # Styling (CSS/SCSS)
│   ├── package.json
│   ├── vite.config.ts
│   └── tsconfig.json
├── shared/                    # Shared types and utilities
│   ├── types/
│   └── constants/
├── docs/                      # Additional documentation
├── tests/                     # Integration tests
├── .github/                   # GitHub Actions workflows
├── docker-compose.yml         # Local development environment
└── README.md
```

### 2.3 Initial Setup

#### Clone and Initialize
```bash
# Create project structure
mkdir veriflair && cd veriflair

# Initialize backend
mkdir backend && cd backend
dfx new . --no-frontend --type=rust

# Initialize frontend
cd .. && mkdir frontend && cd frontend
npm create vite@latest . -- --template react-ts
npm install

# Return to root
cd ..
```

#### Backend Dependencies Setup
```bash
cd backend

# Add to Cargo.toml (workspace)
cat >> Cargo.toml << EOF
[workspace]
members = [
    "src/auth",
    "src/backend", 
    "src/nft"
]
resolver = "2"

[workspace.dependencies]
candid = "0.10"
ic-cdk = "0.12"
ic-cdk-timers = "0.6"
ic-stable-structures = "0.6"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1.0", features = ["full"] }
reqwest = { version = "0.11", features = ["json"] }
EOF
```

#### Frontend Dependencies Setup
```bash
cd frontend

npm install \
  @dfinity/agent \
  @dfinity/auth-client \
  @dfinity/candid \
  @dfinity/principal \
  @dfinity/identity \
  @tanstack/react-query \
  zustand \
  react-router-dom \
  @heroicons/react \
  clsx \
  tailwind-merge

# Development dependencies
npm install -D \
  @types/node \
  eslint \
  prettier \
  @typescript-eslint/eslint-plugin \
  @testing-library/react \
  @testing-library/jest-dom \
  vitest \
  jsdom
```

---

## 3. Backend Implementation Guide

### 3.1 Canister Architecture Overview

#### 3.1.1 Auth Canister (`src/auth/`)

**Purpose**: Manages user authentication, sessions, and role-based access control.

**Key Responsibilities**:
- Internet Identity integration
- Session management
- Role-based permissions
- User profile initialization

#### 3.1.2 Backend Canister (`src/backend/`)

**Purpose**: Core business logic and external integrations.

**Key Responsibilities**:
- GitHub API integration
- LLM analysis coordination
- Badge eligibility calculation
- User profile management
- CORS handling for frontend

#### 3.1.3 NFT Canister (`src/nft/`)

**Purpose**: Soulbound NFT management using ICRC-7 standard.

**Key Responsibilities**:
- Badge minting and ownership
- Metadata management
- Transfer restrictions (soulbound)
- Badge enumeration

### 3.2 DFX Configuration

#### `dfx.json`
```json
{
  "version": 1,
  "canisters": {
    "auth": {
      "type": "rust",
      "candid": "src/auth/auth.did",
      "package": "auth"
    },
    "backend": {
      "type": "rust", 
      "candid": "src/backend/backend.did",
      "package": "backend"
    },
    "nft": {
      "type": "rust",
      "candid": "src/nft/nft.did", 
      "package": "nft"
    },
    "internet_identity": {
      "type": "pull",
      "id": "rdmx6-jaaaa-aaaah-qdrqq-cai"
    }
  },
  "defaults": {
    "build": {
      "args": "",
      "packtool": ""
    }
  },
  "networks": {
    "local": {
      "bind": "127.0.0.1:4943",
      "type": "ephemeral"
    },
    "ic": {
      "providers": ["https://icp-api.io"],
      "type": "persistent"
    }
  },
  "output_env_file": ".env"
}
```

### 3.3 Auth Canister Implementation

#### `src/auth/Cargo.toml`
```toml
[package]
name = "auth"
version = "0.1.0"
edition = "2021"

[dependencies]
candid.workspace = true
ic-cdk.workspace = true
ic-stable-structures.workspace = true
serde.workspace = true
serde_json.workspace = true

[lib]
crate-type = ["cdylib"]
```

#### `src/auth/src/lib.rs`
```rust
use candid::{CandidType, Principal};
use ic_cdk::api::time;
use ic_cdk::{init, post_upgrade, pre_upgrade, query, update};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap};
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
```

#### `src/auth/auth.did`
```
type UserRole = variant {
  User;
  Admin; 
  Moderator;
};

type UserSession = record {
  principal: principal;
  github_username: opt text;
  created_at: nat64;
  last_active: nat64;
  role: UserRole;
  is_verified: bool;
};

service : {
  create_session: (opt text) -> (variant { Ok: UserSession; Err: text });
  update_last_active: () -> (variant { Ok; Err: text });
  get_session: () -> (opt UserSession) query;
  is_authenticated: () -> (bool) query;
  set_user_role: (principal, UserRole) -> (variant { Ok; Err: text });
}
```

### 3.4 Backend Canister Implementation

#### `src/backend/Cargo.toml`
```toml
[package]
name = "backend"
version = "0.1.0" 
edition = "2021"

[dependencies]
candid.workspace = true
ic-cdk.workspace = true
ic-cdk-timers.workspace = true
ic-stable-structures.workspace = true
serde.workspace = true
serde_json.workspace = true
reqwest.workspace = true
base64 = "0.21"
sha2 = "0.10"

[lib]
crate-type = ["cdylib"]
```

#### `src/backend/src/lib.rs`
```rust
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

### 3.5 Models Definition

#### `src/backend/src/models.rs`
```rust
use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(CandidType, Serialize, Deserialize, Clone)]
pub struct UserProfile {
    pub principal: Principal,
    pub github_username: String,
    pub badges: Vec<Badge>,
    pub reputation_score: u32,
    pub last_analysis: Option<u64>,
    pub created_at: u64,
    pub updated_at: u64,
}

#[derive(CandidType, Serialize, Deserialize, Clone)]
pub struct Badge {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: BadgeCategory,
    pub tier: BadgeTier,
    pub earned_at: u64,
    pub metadata: BadgeMetadata,
}

#[derive(CandidType, Serialize, Deserialize, Clone)]
pub enum BadgeCategory {
    Language(String), // "Rust", "Python", etc.
    Contribution(String), // "OpenSource", "Documentation", etc.
    Achievement(String), // "Streak", "Volume", etc.
}

#[derive(CandidType, Serialize, Deserialize, Clone)]
pub enum BadgeTier {
    Bronze,
    Silver, 
    Gold,
    Platinum,
}

#[derive(CandidType, Serialize, Deserialize, Clone)]
pub struct BadgeMetadata {
    pub image_url: String,
    pub animation_url: Option<String>,
    pub attributes: Vec<BadgeAttribute>,
}

#[derive(CandidType, Serialize, Deserialize, Clone)]
pub struct BadgeAttribute {
    pub trait_type: String,
    pub value: String,
}

#[derive(CandidType, Serialize, Deserialize, Clone)]
pub struct GitHubAnalysis {
    pub username: String,
    pub total_commits: u32,
    pub languages: HashMap<String, u32>, // Language -> lines of code
    pub repositories: Vec<Repository>,
    pub llm_insights: LLMAnalysis,
    pub analyzed_at: u64,
}

#[derive(CandidType, Serialize, Deserialize, Clone)]
pub struct Repository {
    pub name: String,
    pub description: Option<String>,
    pub language: Option<String>,
    pub stars: u32,
    pub forks: u32,
    pub commits: u32,
    pub is_fork: bool,
}

#[derive(CandidType, Serialize, Deserialize, Clone)]
pub struct LLMAnalysis {
    pub code_quality_score: f32,
    pub contribution_type: String,
    pub expertise_areas: Vec<String>,
    pub recommended_badges: Vec<String>,
    pub analysis_summary: String,
}

#[derive(CandidType, Serialize, Deserialize)]
pub struct HttpRequest {
    pub method: String,
    pub url: String,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}

#[derive(CandidType, Serialize, Deserialize)]
pub struct HttpResponse {
    pub status_code: u16,
    pub headers: Vec<HttpHeader>,
    pub body: Vec<u8>,
}

#[derive(CandidType, Serialize, Deserialize)]
pub struct HttpHeader {
    pub name: String,
    pub value: String,
}
```

### 3.6 NFT Canister Implementation (ICRC-7 Compliant)

#### `src/nft/Cargo.toml`
```toml
[package]
name = "nft"
version = "0.1.0"
edition = "2021"

[dependencies]
candid.workspace = true
ic-cdk.workspace = true
ic-stable-structures.workspace = true
serde.workspace = true
serde_json.workspace = true

[lib]
crate-type = ["cdylib"]
```

#### `src/nft/src/lib.rs`
```rust
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
```

---

## 4. Frontend Implementation Guide

### 4.1 Project Setup & Configuration

#### `package.json`
```json
{
  "name": "veriflair-frontend",
  "private": true,
  "version": "0.0.0",
  "type": "module",
  "scripts": {
    "dev": "vite",
    "build": "tsc && vite build",
    "preview": "vite preview",
    "test": "vitest",
    "lint": "eslint . --ext ts,tsx --report-unused-disable-directives --max-warnings 0",
    "format": "prettier --write \"src/**/*.{ts,tsx}\""
  },
  "dependencies": {
    "@dfinity/agent": "^1.1.0",
    "@dfinity/auth-client": "^1.1.0",
    "@dfinity/candid": "^1.1.0",
    "@dfinity/principal": "^1.1.0",
    "@dfinity/identity": "^1.1.0",
    "@tanstack/react-query": "^5.0.0",
    "@heroicons/react": "^2.0.18",
    "clsx": "^2.0.0",
    "react": "^18.2.0",
    "react-dom": "^18.2.0",
    "react-router-dom": "^6.15.0",
    "tailwind-merge": "^1.14.0",
    "zustand": "^4.4.1"
  },
  "devDependencies": {
    "@types/react": "^18.2.15",
    "@types/react-dom": "^18.2.7",
    "@typescript-eslint/eslint-plugin": "^6.0.0",
    "@typescript-eslint/parser": "^6.0.0",
    "@vitejs/plugin-react": "^4.0.3",
    "autoprefixer": "^10.4.14",
    "eslint": "^8.45.0",
    "eslint-plugin-react-hooks": "^4.6.0",
    "eslint-plugin-react-refresh": "^0.4.3",
    "postcss": "^8.4.27",
    "prettier": "^3.0.0",
    "tailwindcss": "^3.3.0",
    "typescript": "^5.0.2",
    "vite": "^4.4.5",
    "vitest": "^0.34.0"
  }
}
```

#### `vite.config.ts`
```typescript
import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import { resolve } from 'path'

export default defineConfig({
  plugins: [react()],
  resolve: {
    alias: {
      '@': resolve(__dirname, './src'),
    },
  },
  server: {
    port: 3000,
    open: true,
  },
  build: {
    outDir: 'dist',
    sourcemap: true,
  },
  define: {
    global: 'globalThis',
  },
  optimizeDeps: {
    include: ['@dfinity/agent', '@dfinity/candid', '@dfinity/principal'],
  },
})
```

#### `tailwind.config.js`
```javascript
/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        primary: {
          50: '#eff6ff',
          500: '#3b82f6',
          600: '#2563eb',
          700: '#1d4ed8',
        },
        badge: {
          bronze: '#cd7f32',
          silver: '#c0c0c0',
          gold: '#ffd700',
          platinum: '#e5e4e2',
        }
      },
      animation: {
        'float': 'float 6s ease-in-out infinite',
        'glow': 'glow 2s ease-in-out infinite alternate',
      },
      keyframes: {
        float: {
          '0%, 100%': { transform: 'translateY(0px)' },
          '50%': { transform: 'translateY(-20px)' },
        },
        glow: {
          'from': { boxShadow: '0 0 10px #3b82f6' },
          'to': { boxShadow: '0 0 20px #3b82f6, 0 0 30px #3b82f6' },
        }
      }
    },
  },
  plugins: [],
}
```

### 4.2 Core Services & Utilities

#### `src/services/icp.ts`
```typescript
import { Actor, HttpAgent, Identity } from '@dfinity/agent';
import { AuthClient } from '@dfinity/auth-client';
import { Principal } from '@dfinity/principal';

// Import generated declarations
import { idlFactory as backendIdlFactory, _SERVICE as BackendService } from '../declarations/backend';
import { idlFactory as authIdlFactory, _SERVICE as AuthService } from '../declarations/auth';
import { idlFactory as nftIdlFactory, _SERVICE as NftService } from '../declarations/nft';

const isDevelopment = process.env.NODE_ENV === 'development';
const host = isDevelopment ? 'http://127.0.0.1:4943' : 'https://icp-api.io';

const BACKEND_CANISTER_ID = import.meta.env.VITE_BACKEND_CANISTER_ID;
const AUTH_CANISTER_ID = import.meta.env.VITE_AUTH_CANISTER_ID;
const NFT_CANISTER_ID = import.meta.env.VITE_NFT_CANISTER_ID;
const INTERNET_IDENTITY_URL = isDevelopment 
  ? `http://127.0.0.1:4943/?canister=rdmx6-jaaaa-aaaah-qdrqq-cai`
  : 'https://identity.ic0.app';

class ICPService {
  private agent: HttpAgent | null = null;
  private authClient: AuthClient | null = null;
  private identity: Identity | null = null;

  // Actor instances
  public backend: BackendService | null = null;
  public auth: AuthService | null = null;
  public nft: NftService | null = null;

  async initialize() {
    // Initialize auth client
    this.authClient = await AuthClient.create({
      idleOptions: {
        disableIdle: isDevelopment,
        idleTimeout: 1000 * 60 * 30, // 30 minutes
      },
    });

    // Create agent
    this.agent = new HttpAgent({ 
      host,
      identity: this.authClient.getIdentity(),
    });

    // Fetch root key for local development
    if (isDevelopment) {
      await this.agent.fetchRootKey().catch(console.error);
    }

    // Initialize actors
    this.createActors();

    return this.isAuthenticated();
  }

  private createActors() {
    if (!this.agent) return;

    this.backend = Actor.createActor<BackendService>(backendIdlFactory, {
      agent: this.agent,
      canisterId: BACKEND_CANISTER_ID,
    });

    this.auth = Actor.createActor<AuthService>(authIdlFactory, {
      agent: this.agent,
      canisterId: AUTH_CANISTER_ID,
    });

    this.nft = Actor.createActor<NftService>(nftIdlFactory, {
      agent: this.agent,
      canisterId: NFT_CANISTER_ID,
    });
  }

  async login(): Promise<boolean> {
    if (!this.authClient) {
      throw new Error('AuthClient not initialized');
    }

    return new Promise((resolve) => {
      this.authClient!.login({
        identityProvider: INTERNET_IDENTITY_URL,
        onSuccess: async () => {
          this.identity = this.authClient!.getIdentity();
          
          // Update agent with new identity
          this.agent = new HttpAgent({
            host,
            identity: this.identity,
          });

          if (isDevelopment) {
            await this.agent.fetchRootKey().catch(console.error);
          }

          // Recreate actors with authenticated identity
          this.createActors();

          // Create session in auth canister
          try {
            await this.auth!.create_session([]);
          } catch (error) {
            console.error('Failed to create session:', error);
          }

          resolve(true);
        },
        onError: (error) => {
          console.error('Login failed:', error);
          resolve(false);
        },
      });
    });
  }

  async logout(): Promise<void> {
    if (!this.authClient) return;

    await this.authClient.logout();
    this.identity = null;
    
    // Reset agent to anonymous
    this.agent = new HttpAgent({ host });
    
    if (isDevelopment) {
      await this.agent.fetchRootKey().catch(console.error);
    }

    // Recreate actors with anonymous identity
    this.createActors();
  }

  isAuthenticated(): boolean {
    return this.authClient?.isAuthenticated() ?? false;
  }

  getPrincipal(): Principal | null {
    return this.identity?.getPrincipal() ?? null;
  }

  async createProfile(githubUsername: string) {
    if (!this.backend) throw new Error('Backend actor not available');
    return await this.backend.create_profile(githubUsername);
  }

  async getProfile(user?: Principal) {
    if (!this.backend) throw new Error('Backend actor not available');
    return await this.backend.get_profile(user ? [user] : []);
  }

  async triggerAnalysis() {
    if (!this.backend) throw new Error('Backend actor not available');
    return await this.backend.trigger_analysis();
  }

  async getBadges(user?: Principal) {
    if (!this.backend) throw new Error('Backend actor not available');
    return await this.backend.get_badges(user ? [user] : []);
  }

  async getLeaderboard(limit?: number) {
    if (!this.backend) throw new Error('Backend actor not available');
    return await this.backend.get_leaderboard(limit ? [limit] : []);
  }

  async getUserNFTs(user: Principal) {
    if (!this.nft) throw new Error('NFT actor not available');
    return await this.nft.get_user_badges(user);
  }
}

export const icpService = new ICPService();
export default icpService;
```

#### `src/hooks/useICP.ts`
```typescript
import { useEffect, useState } from 'react';
import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query';
import { Principal } from '@dfinity/principal';
import icpService from '../services/icp';
import type { UserProfile, Badge } from '../types';

export const useICP = () => {
  const [isInitialized, setIsInitialized] = useState(false);
  const [isAuthenticated, setIsAuthenticated] = useState(false);
  const [principal, setPrincipal] = useState<Principal | null>(null);

  useEffect(() => {
    const initializeICP = async () => {
      try {
        const authenticated = await icpService.initialize();
        setIsAuthenticated(authenticated);
        setPrincipal(icpService.getPrincipal());
        setIsInitialized(true);
      } catch (error) {
        console.error('Failed to initialize ICP:', error);
        setIsInitialized(true);
      }
    };

    initializeICP();
  }, []);

  const login = async () => {
    try {
      const success = await icpService.login();
      if (success) {
        setIsAuthenticated(true);
        setPrincipal(icpService.getPrincipal());
      }
      return success;
    } catch (error) {
      console.error('Login failed:', error);
      return false;
    }
  };

  const logout = async () => {
    try {
      await icpService.logout();
      setIsAuthenticated(false);
      setPrincipal(null);
    } catch (error) {
      console.error('Logout failed:', error);
    }
  };

  return {
    isInitialized,
    isAuthenticated,
    principal,
    login,
    logout,
  };
};

export const useProfile = (user?: Principal) => {
  return useQuery({
    queryKey: ['profile', user?.toString()],
    queryFn: () => icpService.getProfile(user),
    enabled: !!icpService.backend,
  });
};

export const useCreateProfile = () => {
  const queryClient = useQueryClient();
  
  return useMutation({
    mutationFn: (githubUsername: string) => icpService.createProfile(githubUsername),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['profile'] });
    },
  });
};

export const useTriggerAnalysis = () => {
  const queryClient = useQueryClient();
  
  return useMutation({
    mutationFn: () => icpService.triggerAnalysis(),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['profile'] });
      queryClient.invalidateQueries({ queryKey: ['badges'] });
    },
  });
};

export const useBadges = (user?: Principal) => {
  return useQuery({
    queryKey: ['badges', user?.toString()],
    queryFn: () => icpService.getBadges(user),
    enabled: !!icpService.backend,
  });
};

export const useLeaderboard = (limit?: number) => {
  return useQuery({
    queryKey: ['leaderboard', limit],
    queryFn: () => icpService.getLeaderboard(limit),
    enabled: !!icpService.backend,
    staleTime: 5 * 60 * 1000, // 5 minutes
  });
};

export const useUserNFTs = (user: Principal) => {
  return useQuery({
    queryKey: ['nfts', user.toString()],
    queryFn: () => icpService.getUserNFTs(user),
    enabled: !!icpService.nft && !!user,
  });
};
```

### 4.3 State Management

#### `src/store/authStore.ts`
```typescript
import { create } from 'zustand';
import { Principal } from '@dfinity/principal';
import { persist } from 'zustand/middleware';

interface AuthState {
  isAuthenticated: boolean;
  principal: Principal | null;
  githubUsername: string | null;
  hasProfile: boolean;
  
  setAuthenticated: (authenticated: boolean) => void;
  setPrincipal: (principal: Principal | null) => void;
  setGithubUsername: (username: string | null) => void;
  setHasProfile: (hasProfile: boolean) => void;
  reset: () => void;
}

export const useAuthStore = create<AuthState>()(
  persist(
    (set) => ({
      isAuthenticated: false,
      principal: null,
      githubUsername: null,
      hasProfile: false,

      setAuthenticated: (authenticated) => set({ isAuthenticated: authenticated }),
      setPrincipal: (principal) => set({ principal }),
      setGithubUsername: (username) => set({ githubUsername: username }),
      setHasProfile: (hasProfile) => set({ hasProfile }),
      
      reset: () => set({
        isAuthenticated: false,
        principal: null,
        githubUsername: null,
        hasProfile: false,
      }),
    }),
    {
      name: 'auth-storage',
      partialize: (state) => ({
        githubUsername: state.githubUsername,
      }),
    }
  )
);
```

#### `src/store/uiStore.ts`
```typescript
import { create } from 'zustand';

interface UIState {
  theme: 'light' | 'dark';
  sidebarOpen: boolean;
  loading: boolean;
  error: string | null;
  
  setTheme: (theme: 'light' | 'dark') => void;
  setSidebarOpen: (open: boolean) => void;
  setLoading: (loading: boolean) => void;
  setError: (error: string | null) => void;
}

export const useUIStore = create<UIState>((set) => ({
  theme: 'dark',
  sidebarOpen: false,
  loading: false,
  error: null,

  setTheme: (theme) => set({ theme }),
  setSidebarOpen: (open) => set({ sidebarOpen: open }),
  setLoading: (loading) => set({ loading }),
  setError: (error) => set({ error }),
}));
```

### 4.4 Component Implementation

#### `src/components/common/LoadingSpinner.tsx`
```typescript
import React from 'react';
import { clsx } from 'clsx';

interface LoadingSpinnerProps {
  size?: 'sm' | 'md' | 'lg';
  className?: string;
}

const LoadingSpinner: React.FC<LoadingSpinnerProps> = ({ 
  size = 'md', 
  className 
}) => {
  const sizeClasses = {
    sm: 'w-4 h-4',
    md: 'w-8 h-8',
    lg: 'w-12 h-12',
  };

  return (
    <div
      className={clsx(
        'animate-spin rounded-full border-2 border-gray-300 border-t-primary-500',
        sizeClasses[size],
        className
      )}
    />
  );
};

export default LoadingSpinner;
```

#### `src/components/auth/LoginButton.tsx`
```typescript
import React, { useState } from 'react';
import { useICP } from '../../hooks/useICP';
import LoadingSpinner from '../common/LoadingSpinner';

const LoginButton: React.FC = () => {
  const { isAuthenticated, login, logout } = useICP();
  const [loading, setLoading] = useState(false);

  const handleAuth = async () => {
    setLoading(true);
    try {
      if (isAuthenticated) {
        await logout();
      } else {
        await login();
      }
    } catch (error) {
      console.error('Authentication failed:', error);
    } finally {
      setLoading(false);
    }
  };

  return (
    <button
      onClick={handleAuth}
      disabled={loading}
      className="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-primary-600 hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500 disabled:opacity-50 disabled:cursor-not-allowed"
    >
      {loading && <LoadingSpinner size="sm" className="mr-2" />}
      {isAuthenticated ? 'Logout' : 'Login with Internet Identity'}
    </button>
  );
};

export default LoginButton;
```

#### `src/components/badges/BadgeCard.tsx`
```typescript
import React from 'react';
import { clsx } from 'clsx';
import type { Badge, BadgeTier } from '../../types';

interface BadgeCardProps {
  badge: Badge;
  size?: 'sm' | 'md' | 'lg';
  showDetails?: boolean;
  animated?: boolean;
}

const BadgeCard: React.FC<BadgeCardProps> = ({
  badge,
  size = 'md',
  showDetails = true,
  animated = true,
}) => {
  const sizeClasses = {
    sm: 'w-16 h-16',
    md: 'w-24 h-24',
    lg: 'w-32 h-32',
  };

  const tierColors = {
    Bronze: 'from-yellow-600 to-yellow-800',
    Silver: 'from-gray-300 to-gray-500',
    Gold: 'from-yellow-300 to-yellow-500',
    Platinum: 'from-purple-300 to-purple-500',
  };

  const tierGlow = {
    Bronze: 'shadow-yellow-500/50',
    Silver: 'shadow-gray-400/50',
    Gold: 'shadow-yellow-400/50',
    Platinum: 'shadow-purple-400/50',
  };

  return (
    <div
      className={clsx(
        'relative group cursor-pointer transition-all duration-300',
        animated && 'hover:scale-105 hover:-translate-y-1',
        showDetails ? 'p-4' : 'p-2'
      )}
    >
      {/* Badge Container */}
      <div
        className={clsx(
          'relative mx-auto rounded-full bg-gradient-to-br border-2 border-white/20',
          sizeClasses[size],
          tierColors[badge.tier as keyof typeof tierColors],
          animated && 'group-hover:shadow-lg transition-shadow duration-300',
          tierGlow[badge.tier as keyof typeof tierGlow]
        )}
      >
        {/* Badge Icon/Image */}
        <div className="absolute inset-2 flex items-center justify-center">
          {badge.metadata.image_url ? (
            <img
              src={badge.metadata.image_url}
              alt={badge.name}
              className="w-full h-full object-contain"
            />
          ) : (
            <div className="w-full h-full bg-white/20 rounded-full flex items-center justify-center">
              <span className="text-white font-bold text-xs">
                {badge.name.charAt(0)}
              </span>
            </div>
          )}
        </div>

        {/* Tier Indicator */}
        <div className="absolute -bottom-1 -right-1 bg-black/80 text-white text-xs px-1.5 py-0.5 rounded-full font-semibold">
          {badge.tier}
        </div>

        {/* Glow Effect */}
        {animated && (
          <div className="absolute inset-0 rounded-full bg-gradient-to-br opacity-0 group-hover:opacity-30 transition-opacity duration-300 from-white to-transparent" />
        )}
      </div>

      {/* Badge Details */}
      {showDetails && (
        <div className="mt-3 text-center">
          <h3 className="font-semibold text-sm text-gray-900 dark:text-white truncate">
            {badge.name}
          </h3>
          <p className="text-xs text-gray-600 dark:text-gray-400 mt-1 truncate">
            {badge.description}
          </p>
          <div className="text-xs text-gray-500 dark:text-gray-500 mt-1">
            {new Date(Number(badge.earned_at) / 1000000).toLocaleDateString()}
          </div>
        </div>
      )}

      {/* Tooltip */}
      {!showDetails && (
        <div className="absolute bottom-full left-1/2 transform -translate-x-1/2 mb-2 px-3 py-2 bg-black text-white text-sm rounded-lg opacity-0 group-hover:opacity-100 transition-opacity duration-200 pointer-events-none whitespace-nowrap z-10">
          <div className="font-semibold">{badge.name}</div>
          <div className="text-xs">{badge.description}</div>
          <div className="absolute top-full left-1/2 transform -translate-x-1/2 w-0 h-0 border-l-4 border-r-4 border-t-4 border-transparent border-t-black"></div>
        </div>
      )}
    </div>
  );
};

export default BadgeCard;
```

#### `src/components/profile/ProfileSetup.tsx`
```typescript
import React, { useState } from 'react';
import { useCreateProfile } from '../../hooks/useICP';
import LoadingSpinner from '../common/LoadingSpinner';

const ProfileSetup: React.FC = () => {
  const [githubUsername, setGithubUsername] = useState('');
  const [error, setError] = useState<string | null>(null);
  const createProfile = useCreateProfile();

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError(null);

    if (!githubUsername.trim()) {
      setError('GitHub username is required');
      return;
    }

    // Basic GitHub username validation
    const githubUsernameRegex = /^[a-z\d](?:[a-z\d]|-(?=[a-z\d])){0,38}$/i;
    if (!githubUsernameRegex.test(githubUsername)) {
      setError('Invalid GitHub username format');
      return;
    }

    try {
      await createProfile.mutateAsync(githubUsername);
    } catch (err: any) {
      setError(err.message || 'Failed to create profile');
    }
  };

  return (
    <div className="min-h-screen bg-gradient-to-br from-primary-50 to-indigo-100 dark:from-gray-900 dark:to-gray-800 flex items-center justify-center py-12 px-4 sm:px-6 lg:px-8">
      <div className="max-w-md w-full space-y-8">
        <div>
          <div className="mx-auto h-12 w-12 flex items-center justify-center rounded-full bg-primary-100 dark:bg-primary-900">
            <svg className="h-6 w-6 text-primary-600 dark:text-primary-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" />
            </svg>
          </div>
          <h2 className="mt-6 text-center text-3xl font-extrabold text-gray-900 dark:text-white">
            Create Your Profile
          </h2>
          <p className="mt-2 text-center text-sm text-gray-600 dark:text-gray-400">
            Connect your GitHub account to start earning badges
          </p>
        </div>

        <form className="mt-8 space-y-6" onSubmit={handleSubmit}>
          <div>
            <label htmlFor="github-username" className="sr-only">
              GitHub Username
            </label>
            <div className="relative">
              <div className="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
                <svg className="h-5 w-5 text-gray-400" fill="currentColor" viewBox="0 0 24 24">
                  <path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/>
                </svg>
              </div>
              <input
                id="github-username"
                name="github-username"
                type="text"
                required
                value={githubUsername}
                onChange={(e) => setGithubUsername(e.target.value)}
                className="appearance-none rounded-md relative block w-full px-3 py-2 pl-10 border border-gray-300 dark:border-gray-600 placeholder-gray-500 dark:placeholder-gray-400 text-gray-900 dark:text-white bg-white dark:bg-gray-700 focus:outline-none focus:ring-primary-500 focus:border-primary-500 focus:z-10 sm:text-sm"
                placeholder="GitHub username"
              />
            </div>
          </div>

          {error && (
            <div className="text-red-600 dark:text-red-400 text-sm text-center">
              {error}
            </div>
          )}

          <div>
            <button
              type="submit"
              disabled={createProfile.isPending}
              className="group relative w-full flex justify-center py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white bg-primary-600 hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500 disabled:opacity-50 disabled:cursor-not-allowed"
            >
              {createProfile.isPending && (
                <LoadingSpinner size="sm" className="mr-2" />
              )}
              Create Profile
            </button>
          </div>
        </form>
      </div>
    </div>
  );
};

export default ProfileSetup;
```

### 4.5 Page Components

#### `src/pages/LandingPage.tsx`
```typescript
import React from 'react';
import { Link } from 'react-router-dom';
import { useICP } from '../hooks/useICP';
import LoginButton from '../components/auth/LoginButton';
import BadgeCard from '../components/badges/BadgeCard';

// Mock badge for demo
const mockBadge = {
  id: 'demo-badge',
  name: 'Rust Expert',
  description: 'Mastery in Rust programming',
  category: { Language: 'Rust' },
  tier: 'Gold' as const,
  earned_at: BigInt(Date.now() * 1000000),
  metadata: {
    image_url: '/assets/icons/rust_icon.svg',
    animation_url: null,
    attributes: [],
  },
};

const LandingPage: React.FC = () => {
  const { isAuthenticated } = useICP();

  return (
    <div className="min-h-screen bg-gradient-to-br from-primary-50 via-white to-indigo-50 dark:from-gray-900 dark:via-gray-800 dark:to-gray-900">
      {/* Hero Section */}
      <div className="relative overflow-hidden">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-24">
          <div className="text-center">
            <h1 className="text-4xl md:text-6xl font-extrabold text-gray-900 dark:text-white mb-6">
              <span className="bg-gradient-to-r from-primary-600 to-indigo-600 bg-clip-text text-transparent">
                VeriFlair
              </span>
            </h1>
            <p className="text-xl md:text-2xl text-gray-600 dark:text-gray-300 mb-8 max-w-3xl mx-auto">
              Gamified, on-chain reputation system that transforms your GitHub contributions 
              into verifiable, collectible achievement badges.
            </p>
            
            <div className="flex flex-col sm:flex-row gap-4 justify-center items-center mb-16">
              {isAuthenticated ? (
                <Link
                  to="/profile"
                  className="px-8 py-3 bg-primary-600 text-white font-semibold rounded-lg hover:bg-primary-700 transition-colors duration-200"
                >
                  Go to Profile
                </Link>
              ) : (
                <LoginButton />
              )}
              
              <Link
                to="/leaderboard"
                className="px-8 py-3 border border-primary-600 text-primary-600 font-semibold rounded-lg hover:bg-primary-50 dark:hover:bg-primary-900/20 transition-colors duration-200"
              >
                View Leaderboard
              </Link>
            </div>

            {/* Demo Badge */}
            <div className="flex justify-center">
              <div className="relative">
                <BadgeCard badge={mockBadge} size="lg" showDetails={false} />
                <div className="absolute -inset-4 bg-gradient-to-r from-primary-400 to-indigo-400 rounded-full opacity-20 animate-pulse"></div>
              </div>
            </div>
          </div>
        </div>
      </div>

      {/* Features Section */}
      <div className="py-24 bg-white dark:bg-gray-800">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="text-center mb-16">
            <h2 className="text-3xl md:text-4xl font-bold text-gray-900 dark:text-white mb-4">
              Why VeriFlair?
            </h2>
            <p className="text-lg text-gray-600 dark:text-gray-300 max-w-2xl mx-auto">
              Transform your coding journey into an epic adventure with verifiable achievements
            </p>
          </div>

          <div className="grid md:grid-cols-3 gap-8">
            <div className="text-center p-6">
              <div className="w-16 h-16 mx-auto mb-4 bg-gradient-to-br from-primary-500 to-indigo-500 rounded-full flex items-center justify-center">
                <svg className="w-8 h-8 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
                </svg>
              </div>
              <h3 className="text-xl font-semibold text-gray-900 dark:text-white mb-2">
                Verifiable Achievements
              </h3>
              <p className="text-gray-600 dark:text-gray-300">
                Soulbound NFT badges that prove your coding expertise on the blockchain
              </p>
            </div>

            <div className="text-center p-6">
              <div className="w-16 h-16 mx-auto mb-4 bg-gradient-to-br from-green-500 to-emerald-500 rounded-full flex items-center justify-center">
                <svg className="w-8 h-8 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 10V3L4 14h7v7l9-11h-7z" />
                </svg>
              </div>
              <h3 className="text-xl font-semibold text-gray-900 dark:text-white mb-2">
                AI-Powered Analysis
              </h3>
              <p className="text-gray-600 dark:text-gray-300">
                Advanced AI evaluates your code quality, contribution patterns, and expertise
              </p>
            </div>

            <div className="text-center p-6">
              <div className="w-16 h-16 mx-auto mb-4 bg-gradient-to-br from-purple-500 to-pink-500 rounded-full flex items-center justify-center">
                <svg className="w-8 h-8 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M16 11V7a4 4 0 00-8 0v4M5 9h14l1 12H4L5 9z" />
                </svg>
              </div>
              <h3 className="text-xl font-semibold text-gray-900 dark:text-white mb-2">
                Gamified Experience
              </h3>
              <p className="text-gray-600 dark:text-gray-300">
                Collect badges across tiers and categories, compete on leaderboards
              </p>
            </div>
          </div>
        </div>
      </div>

      {/* CTA Section */}
      <div className="py-24 bg-gradient-to-r from-primary-600 to-indigo-600">
        <div className="max-w-4xl mx-auto text-center px-4 sm:px-6 lg:px-8">
          <h2 className="text-3xl md:text-4xl font-bold text-white mb-6">
            Ready to showcase your skills?
          </h2>
          <p className="text-xl text-primary-100 mb-8">
            Join VeriFlair today and turn your GitHub contributions into verifiable achievements
          </p>
          
          {!isAuthenticated && (
            <LoginButton />
          )}
        </div>
      </div>
    </div>
  );
};

export default LandingPage;
```

---

## 5. Security & Authentication

### 5.1 Authentication Flow

#### Internet Identity Integration
```typescript
// Authentication flow diagram:
// 1. User clicks "Login" → Frontend redirects to Internet Identity
// 2. User authenticates with II → Returns delegation
// 3. Frontend creates session with Auth Canister
// 4. All subsequent requests include authenticated identity
```

### 5.2 Security Best Practices

#### Canister Security
```rust
// Always validate caller identity
fn validate_caller() -> Result<Principal, String> {
    let caller = ic_cdk::caller();
    if caller == Principal::anonymous() {
        return Err("Anonymous users not allowed".to_string());
    }
    Ok(caller)
}

// Use guard functions for sensitive operations
#[update(guard = "is_authorized")]
fn sensitive_operation() -> Result<(), String> {
    // Only authorized users can call this
    Ok(())
}

fn is_authorized() -> Result<(), String> {
    let caller = ic_cdk::caller();
    // Check authorization logic
    if is_user_authorized(caller) {
        Ok(())
    } else {
        Err("Unauthorized".to_string())
    }
}
```

#### Input Validation
```rust
fn validate_github_username(username: &str) -> Result<(), String> {
    if username.is_empty() {
        return Err("Username cannot be empty".to_string());
    }
    
    if username.len() > 39 {
        return Err("Username too long".to_string());
    }
    
    let valid_chars = username.chars().all(|c| {
        c.is_ascii_alphanumeric() || c == '-'
    });
    
    if !valid_chars {
        return Err("Invalid characters in username".to_string());
    }
    
    Ok(())
}
```

### 5.3 Rate Limiting

#### Implementation Example
```rust
use std::collections::HashMap;
use ic_cdk::api::time;

thread_local! {
    static RATE_LIMITS: RefCell<HashMap<Principal, Vec<u64>>> = RefCell::new(HashMap::new());
}

fn check_rate_limit(caller: Principal, max_calls: usize, window_seconds: u64) -> Result<(), String> {
    let now = time();
    let window_nanos = window_seconds * 1_000_000_000;
    
    RATE_LIMITS.with(|limits| {
        let mut limits = limits.borrow_mut();
        let call_times = limits.entry(caller).or_insert_with(Vec::new);
        
        // Remove old calls outside the window
        call_times.retain(|&time| now - time < window_nanos);
        
        if call_times.len() >= max_calls {
            return Err("Rate limit exceeded".to_string());
        }
        
        call_times.push(now);
        Ok(())
    })
}

#[update]
async fn trigger_analysis() -> Result<String, String> {
    let caller = validate_caller()?;
    
    // Allow max 3 analysis requests per hour
    check_rate_limit(caller, 3, 3600)?;
    
    // Proceed with analysis...
    Ok("Analysis triggered".to_string())
}
```

---

## 6. Testing Strategies

### 6.1 Backend Testing

#### Unit Tests (`src/backend/src/lib.rs`)
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use candid::Principal;

    #[test]
    fn test_validate_github_username() {
        assert!(validate_github_username("valid-user").is_ok());
        assert!(validate_github_username("").is_err());
        assert!(validate_github_username("user_with_underscores").is_err());
        assert!(validate_github_username(&"a".repeat(40)).is_err());
    }

    #[test]
    fn test_badge_generation() {
        let analysis = GitHubAnalysis {
            username: "testuser".to_string(),
            total_commits: 1000,
            languages: [("Rust".to_string(), 50000)].iter().cloned().collect(),
            repositories: vec![],
            llm_insights: LLMAnalysis {
                code_quality_score: 8.5,
                contribution_type: "Expert".to_string(),
                expertise_areas: vec!["Rust".to_string()],
                recommended_badges: vec!["RustExpert".to_string()],
                analysis_summary: "Excellent Rust developer".to_string(),
            },
            analyzed_at: 0,
        };

        let badges = generate_badges_from_analysis(&analysis);
        assert!(!badges.is_empty());
        assert!(badges.iter().any(|b| matches!(b.category, BadgeCategory::Language(ref lang) if lang == "Rust")));
    }
}
```

#### Integration Tests (`tests/integration_tests.rs`)
```rust
use candid::Principal;
use pocket_ic::PocketIc;

#[test]
fn test_full_user_flow() {
    let pic = PocketIc::new();
    
    // Install canisters
    let auth_wasm = include_bytes!("../target/wasm32-unknown-unknown/release/auth.wasm");
    let backend_wasm = include_bytes!("../target/wasm32-unknown-unknown/release/backend.wasm");
    let nft_wasm = include_bytes!("../target/wasm32-unknown-unknown/release/nft.wasm");
    
    let auth_canister = pic.create_canister();
    let backend_canister = pic.create_canister();
    let nft_canister = pic.create_canister();
    
    pic.install_canister(auth_canister, auth_wasm.to_vec(), vec![]);
    pic.install_canister(backend_canister, backend_wasm.to_vec(), candid::encode_args((auth_canister, nft_canister)).unwrap());
    pic.install_canister(nft_canister, nft_wasm.to_vec(), candid::encode_args((backend_canister,)).unwrap());
    
    // Test user flow
    let user = Principal::from_text("rdmx6-jaaaa-aaaah-qdrqq-cai").unwrap();
    pic.set_sender(user);
    
    // Create session
    let result: Result<UserSession, String> = pic.query_call(
        auth_canister,
        "create_session",
        candid::encode_args((Some("testuser".to_string()),)).unwrap(),
    ).unwrap();
    assert!(result.is_ok());
    
    // Create profile
    let result: Result<UserProfile, String> = pic.update_call(
        backend_canister,
        "create_profile", 
        candid::encode_args(("testuser".to_string(),)).unwrap(),
    ).unwrap();
    assert!(result.is_ok());
    
    // Get profile
    let profile: Option<UserProfile> = pic.query_call(
        backend_canister,
        "get_profile",
        candid::encode_args((None::<Principal>,)).unwrap(),
    ).unwrap();
    assert!(profile.is_some());
    assert_eq!(profile.unwrap().github_username, "testuser");
}