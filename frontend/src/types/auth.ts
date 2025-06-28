import { Principal } from '@dfinity/principal';

export interface UserSession {
  user_principal: Principal;
  github_username: string | null;
  created_at: bigint;
  last_active: bigint;
  expires_at: bigint;
  role: { User: null } | { Admin: null } | { Moderator: null };
  is_verified: boolean;
}

export interface AuthService {
  authenticate_user: () => Promise<{ Ok: UserSession } | { Err: string }>;
  is_authenticated: () => Promise<boolean>;
  get_session: () => Promise<UserSession | null>;
  create_session: (githubUsername: string | null) => Promise<{ Ok: UserSession } | { Err: string }>;
  renew_session: () => Promise<{ Ok: UserSession } | { Err: string }>;
  logout: () => Promise<{ Ok: string } | { Err: string }>;
  health_check: () => Promise<string>;
}

export interface AuthContextType {
  isInitialized: boolean;
  isAuthenticated: boolean;
  user: UserSession | null;
  principal: Principal | null;
  login: () => Promise<void>;
  logout: () => Promise<void>;
  setGitHubUsername: (username: string) => Promise<boolean>;
  renewSession: () => Promise<boolean>;
  error: string | null;
  isLoading: boolean;
}