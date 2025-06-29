import { Actor, HttpAgent } from '@dfinity/agent';
import { AuthClient } from '@dfinity/auth-client';

// Your deployed canister IDs (from the deployment)
const CANISTER_IDS = {
  auth: process.env.REACT_APP_AUTH_CANISTER_ID,
  backend: process.env.REACT_APP_BACKEND_CANISTER_ID,
  nft: process.env.REACT_APP_NFT_CANISTER_ID,
};

const HOST = 'http://127.0.0.1:4943'; // Local development

// Candid interfaces for your canisters
const backendIdl = ({ IDL }) => {
  const BadgeTier = IDL.Variant({
    'Bronze1': IDL.Null,
    'Bronze2': IDL.Null,
    'Bronze3': IDL.Null,
    'Silver1': IDL.Null,
    'Silver2': IDL.Null,
    'Silver3': IDL.Null,
    'Gold1': IDL.Null,
    'Gold2': IDL.Null,
    'Gold3': IDL.Null,
  });

  const BadgeCategory = IDL.Variant({
    'Language': IDL.Text,
    'Contribution': IDL.Text,
    'Achievement': IDL.Text,
    'Special': IDL.Text,
  });

  const BadgeAttribute = IDL.Record({
    'trait_type': IDL.Text,
    'value': IDL.Text,
    'display_type': IDL.Opt(IDL.Text),
  });

  const BadgeMetadata = IDL.Record({
    'image_url': IDL.Text,
    'animation_url': IDL.Opt(IDL.Text),
    'attributes': IDL.Vec(BadgeAttribute),
    'rarity_score': IDL.Nat32,
  });

  const Badge = IDL.Record({
    'id': IDL.Text,
    'name': IDL.Text,
    'description': IDL.Text,
    'category': BadgeCategory,
    'tier': BadgeTier,
    'earned_at': IDL.Nat64,
    'criteria_met': IDL.Vec(IDL.Text),
    'score_achieved': IDL.Nat32,
    'metadata': BadgeMetadata,
  });

  const GitHubData = IDL.Record({
    'login': IDL.Text,
    'name': IDL.Opt(IDL.Text),
    'avatar_url': IDL.Text,
    'bio': IDL.Opt(IDL.Text),
    'public_repos': IDL.Nat32,
    'followers': IDL.Nat32,
    'following': IDL.Nat32,
    'created_at': IDL.Text,
    'updated_at': IDL.Text,
  });

  const UserProfile = IDL.Record({
    'user_principal': IDL.Principal,
    'github_username': IDL.Text,
    'github_connected': IDL.Bool,
    'github_data': IDL.Opt(GitHubData),
    'created_at': IDL.Nat64,
    'updated_at': IDL.Nat64,
    'last_github_sync': IDL.Opt(IDL.Nat64),
    'reputation_score': IDL.Nat64,
    'badges': IDL.Vec(Badge),
    'total_badges': IDL.Nat32,
  });

  const GitHubOAuthRequest = IDL.Record({
    'code': IDL.Text,
    'state': IDL.Text,
  });

  return IDL.Service({
    'create_initial_profile': IDL.Func([], [IDL.Variant({ 'Ok': UserProfile, 'Err': IDL.Text })], []),
    'connect_github_oauth': IDL.Func([GitHubOAuthRequest], [IDL.Variant({ 'Ok': UserProfile, 'Err': IDL.Text })], []),
    'get_profile': IDL.Func([IDL.Opt(IDL.Principal)], [IDL.Opt(UserProfile)], ['query']),
    'get_badges': IDL.Func([IDL.Opt(IDL.Principal)], [IDL.Vec(Badge)], ['query']),
    'get_leaderboard': IDL.Func([IDL.Opt(IDL.Nat32)], [IDL.Vec(UserProfile)], ['query']),
    'sync_github_data': IDL.Func([], [IDL.Variant({ 'Ok': UserProfile, 'Err': IDL.Text })], []),
    'health_check': IDL.Func([], [IDL.Text], ['query']),
  });
};

class CanisterService {
  constructor() {
    this.agent = null;
    this.authClient = null;
    this.backendActor = null;
    this.isInitialized = false;
  }

  async initialize() {
    if (this.isInitialized) return;

    try {
      // Create auth client
      this.authClient = await AuthClient.create();

      // Create agent
      this.agent = new HttpAgent({ host: HOST });

      // Fetch root key for local development
      await this.agent.fetchRootKey();

      // Create backend actor
      this.backendActor = Actor.createActor(backendIdl, {
        agent: this.agent,
        canisterId: CANISTER_IDS.backend,
      });

      this.isInitialized = true;
      console.log('🚀 Canister service initialized successfully');
    } catch (error) {
      console.error('❌ Failed to initialize canister service:', error);
      throw error;
    }
  }

  async login() {
    if (!this.authClient) throw new Error('Auth client not initialized');

    return new Promise((resolve) => {
      this.authClient.login({
        identityProvider: 'http://127.0.0.1:4943?canisterId=rdmx6-jaaaa-aaaah-qdrqq-cai',
        onSuccess: () => {
          console.log('✅ Internet Identity login successful');
          resolve(true);
        },
        onError: (error) => {
          console.error('❌ Internet Identity login failed:', error);
          resolve(false);
        },
      });
    });
  }

  async logout() {
    if (this.authClient) {
      await this.authClient.logout();
    }
  }

  async isAuthenticated() {
    return this.authClient?.isAuthenticated() || false;
  }

  // GitHub OAuth Flow
  initiateGitHubOAuth() {
    const state = Math.random().toString(36).substring(7);
    localStorage.setItem('github_oauth_state', state);

    const githubClientId = process.env.REACT_APP_GITHUB_CLIENT_ID || 'Ov23lilX1z6LtvGmM8x3';
    const redirectUri = `${window.location.origin}/auth/github/callback`;

    const githubOAuthUrl = `https://github.com/login/oauth/authorize?client_id=${githubClientId}&scope=user:email,public_repo&state=${state}&redirect_uri=${redirectUri}`;

    console.log('🐙 Redirecting to GitHub OAuth:', githubOAuthUrl);
    window.location.href = githubOAuthUrl;
  }

  async handleGitHubCallback(code, state) {
    await this.initialize();

    // Verify state
    const savedState = localStorage.getItem('github_oauth_state');
    if (state !== savedState) {
      throw new Error('Invalid OAuth state');
    }

    try {
      console.log('🔄 Processing GitHub OAuth callback...');

      // Call your backend to exchange code for GitHub data + generate badges
      const result = await this.backendActor.connect_github_oauth({
        code,
        state
      });

      if ('Ok' in result) {
        console.log('✅ GitHub connected successfully!', result.Ok);
        localStorage.removeItem('github_oauth_state');
        return result.Ok; // Returns UserProfile with badges
      } else {
        throw new Error(result.Err);
      }
    } catch (error) {
      console.error('❌ GitHub OAuth failed:', error);
      throw error;
    }
  }

  // Profile & Badge Management
  async createProfile() {
    await this.initialize();

    try {
      const result = await this.backendActor.create_initial_profile();
      if ('Ok' in result) {
        return result.Ok;
      } else {
        throw new Error(result.Err);
      }
    } catch (error) {
      console.error('❌ Failed to create profile:', error);
      throw error;
    }
  }

  async getProfile(userPrincipal = null) {
    await this.initialize();

    try {
      const profile = await this.backendActor.get_profile(userPrincipal ? [userPrincipal] : []);
      return profile;
    } catch (error) {
      console.error('❌ Failed to get profile:', error);
      return null;
    }
  }

  async getBadges(userPrincipal = null) {
    await this.initialize();

    try {
      const badges = await this.backendActor.get_badges(userPrincipal ? [userPrincipal] : []);
      return badges || [];
    } catch (error) {
      console.error('❌ Failed to get badges:', error);
      return [];
    }
  }

  async getLeaderboard(limit = 10) {
    await this.initialize();

    try {
      const leaderboard = await this.backendActor.get_leaderboard([limit]);
      return leaderboard || [];
    } catch (error) {
      console.error('❌ Failed to get leaderboard:', error);
      return [];
    }
  }

  async syncGitHubData() {
    await this.initialize();

    try {
      const result = await this.backendActor.sync_github_data();
      if ('Ok' in result) {
        return result.Ok;
      } else {
        throw new Error(result.Err);
      }
    } catch (error) {
      console.error('❌ Failed to sync GitHub data:', error);
      throw error;
    }
  }
}

// Create singleton instance
export const canisterService = new CanisterService();

// Helper functions for badge display
export const getBadgeImage = (tier) => {
  const tierMap = {
    'Bronze1': '/src/assets/image/Badges/bronze1.png',
    'Bronze2': '/src/assets/image/Badges/bronze2.png',
    'Bronze3': '/src/assets/image/Badges/bronze3.png',
    'Silver1': '/src/assets/image/Badges/silver1.png',
    'Silver2': '/src/assets/image/Badges/silver2.png',
    'Silver3': '/src/assets/image/Badges/silver3.png',
    'Gold1': '/src/assets/image/Badges/gold1.png',
    'Gold2': '/src/assets/image/Badges/gold2.png',
    'Gold3': '/src/assets/image/Badges/gold3.png',
  };

  // Find the matching tier
  const tierKey = Object.keys(tier)[0];
  return tierMap[tierKey] || '/src/assets/image/Badges/bronze1.png';
};

export const getTierDisplayName = (tier) => {
  const tierKey = Object.keys(tier)[0];
  const displayMap = {
    'Bronze1': 'Bronze I',
    'Bronze2': 'Bronze II',
    'Bronze3': 'Bronze III',
    'Silver1': 'Silver I',
    'Silver2': 'Silver II',
    'Silver3': 'Silver III',
    'Gold1': 'Gold I',
    'Gold2': 'Gold II',
    'Gold3': 'Gold III',
  };
  return displayMap[tierKey] || 'Bronze I';
};

export const getCategoryDisplayName = (category) => {
  const categoryKey = Object.keys(category)[0];
  const value = category[categoryKey];

  switch (categoryKey) {
    case 'Language':
      return value; // e.g., "Rust", "Python"
    case 'Achievement':
      return value; // e.g., "Repository Creator"
    case 'Contribution':
      return value; // e.g., "Open Source"
    case 'Special':
      return value; // e.g., "Innovator"
    default:
      return 'Developer';
  }
};