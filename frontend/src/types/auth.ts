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

export type AuthError =
  | { NotAuthenticated: null }
  | { SessionExpired: null }
  | { InvalidPrincipal: null }
  | { InternalError: string };

export interface AuthService {
  // Main Internet Identity endpoints
  authenticate_user: () => Promise<{ Ok: UserSession } | { Err: string }>;
  renew_session: () => Promise<{ Ok: UserSession } | { Err: string }>;
  logout: () => Promise<{ Ok: string } | { Err: string }>;

  // Session management
  create_session: (githubUsername: string | null) => Promise<{ Ok: UserSession } | { Err: string }>;
  update_last_active: () => Promise<{ Ok: null } | { Err: string }>;
  get_session: () => Promise<UserSession | null>;
  is_authenticated: () => Promise<boolean>;

  // Cross-canister validation
  validate_session: (principal: Principal) => Promise<{ Ok: UserSession } | { Err: AuthError }>;

  // Admin functions
  set_user_role: (user: Principal, role: { User: null } | { Admin: null } | { Moderator: null }) => Promise<{ Ok: null } | { Err: string }>;

  // Monitoring
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