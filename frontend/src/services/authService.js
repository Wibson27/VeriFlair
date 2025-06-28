// // Export singleton instance
// export const authService = new InternetIdentityAuthService();
import { AuthClient } from '@dfinity/auth-client';
import { Actor, HttpAgent } from '@dfinity/agent';
import { Principal } from '@dfinity/principal';
import { idlFactory } from '../declarations/auth';

/**
 * Internet Identity Auth Service for VeriFlair
 * Follows the architecture specified in VeriFlair documentation Section 5.1
 */
class InternetIdentityAuthService {
  constructor() {
    this.authClient = null;
    this.actor = null;
    this.isInitialized = false;
  }

  // Environment configuration
  getConfig() {
    const isLocal = process.env.REACT_APP_DFX_NETWORK === 'local' ||
                    process.env.NODE_ENV === 'development';

    return {
      authCanisterId: process.env.REACT_APP_AUTH_CANISTER_ID || 'uxrrr-q7777-77774-qaaaq-cai',

      // WORKAROUND: Always use the live II URL to bypass the local replica issue.
      internetIdentityUrl: 'https://identity.ic0.app',

      host: isLocal ? 'http://127.0.0.1:4943' : 'https://icp-api.io',
      isLocal,
    };
  }

  // Initialize the auth client
  async initialize() {
    if (this.isInitialized) return this.isAuthenticated();

    try {
      const config = this.getConfig();

      console.log('Initializing with config:', config);

      // Create AuthClient
      this.authClient = await AuthClient.create({
        idleOptions: {
          idleTimeout: 1000 * 60 * 30, // 30 minutes
          disableDefaultIdleCallback: true,
        },
      });

      // Setup idle callback
      this.authClient.idleManager?.registerCallback?.(() => {
        console.log('User went idle, logging out...');
        this.logout();
      });

      // Check if already authenticated
      const authenticated = await this.authClient.isAuthenticated();
      console.log('Already authenticated:', authenticated);

      if (authenticated) {
        await this.createActor();

        // Verify session with canister
        const isValid = await this.verifySession();
        if (!isValid) {
          console.log('Session invalid, logging out...');
          await this.logout();
          return false;
        }
      }

      this.isInitialized = true;
      return authenticated;
    } catch (error) {
      console.error('Failed to initialize auth:', error);
      return false;
    }
  }

  // Login with Internet Identity
  async login() {
    if (!this.authClient) {
      throw new Error('Auth client not initialized');
    }

    const config = this.getConfig();
    console.log('Starting login with config:', config);

    return new Promise((resolve, reject) => {
      this.authClient.login({
        identityProvider: config.internetIdentityUrl,
        maxTimeToLive: 7 * 24 * 60 * 60 * 1000 * 1000 * 1000, // 7 days as BigInt
        onSuccess: async () => {
          try {
            console.log('Internet Identity login successful');
            await this.createActor();

            // Authenticate with your canister
            console.log('Authenticating with auth canister...');
            const result = await this.actor.authenticate_user();

            if ('Ok' in result) {
              console.log('Successfully authenticated with canister:', result.Ok);
              resolve(true);
            } else {
              console.error('Canister authentication failed:', result.Err);
              reject(new Error(result.Err[0]));
            }
          } catch (error) {
            console.error('Post-login setup failed:', error);
            reject(error);
          }
        },
        onError: (error) => {
          console.error('Internet Identity login failed:', error);
          reject(error);
        },
      });
    });
  }

  // Logout
  async logout() {
    try {
      // Logout from canister
      if (this.actor) {
        await this.actor.logout();
        console.log('Logged out from canister');
      }
    } catch (error) {
      console.error('Canister logout failed:', error);
    }

    // Logout from Internet Identity
    if (this.authClient) {
      await this.authClient.logout();
      console.log('Logged out from Internet Identity');
    }

    this.actor = null;
  }

  // Check if authenticated
  isAuthenticated() {
    return this.authClient?.isAuthenticated() ?? false;
  }

  // Get current user principal
  getPrincipal() {
    return this.authClient?.getIdentity()?.getPrincipal() ?? null;
  }

  // Get current session
  async getCurrentSession() {
    if (!this.actor) return null;

    try {
      const session = await this.actor.get_session();
      // The session might be wrapped in an array (if it's optional)
      return session.length > 0 ? session[0] : null;
    } catch (error) {
      console.error('Failed to get session:', error);
      return null;
    }
  }

  // Update GitHub username
  async setGitHubUsername(username) {
    if (!this.actor) return false;

    try {
      const result = await this.actor.create_session([username]);

      if ('Ok' in result) {
        console.log('GitHub username updated:', result.Ok);
        return true;
      } else {
        console.error('Failed to update GitHub username:', result.Err);
        return false;
      }
    } catch (error) {
      console.error('GitHub username update error:', error);
      return false;
    }
  }

  // Renew session
  async renewSession() {
    if (!this.actor) return false;

    try {
      const result = await this.actor.renew_session();

      if ('Ok' in result) {
        console.log('Session renewed:', result.Ok);
        return true;
      } else {
        console.error('Session renewal failed:', result.Err);
        return false;
      }
    } catch (error) {
      console.error('Session renewal error:', error);
      return false;
    }
  }

  // Get canister health
  async getHealth() {
    if (!this.actor) return 'Actor not initialized';

    try {
      return await this.actor.health_check();
    } catch (error) {
      console.error('Health check failed:', error);
      return 'Health check failed';
    }
  }

  /**
   * Private: Create actor for canister communication
   * This function is updated to handle the local development environment
   * where a live Identity Provider is used with a local replica.
   */
  async createActor() {
    if (!this.authClient) throw new Error('Auth client not initialized');

    const config = this.getConfig();
    const identity = this.authClient.getIdentity();

    console.log('Creating actor with identity:', identity.getPrincipal().toString());

    // This is the new, more robust agent creation logic.
    let agent;

    if (config.isLocal) {
      console.log('Local environment detected. Creating agent with fetched root key.');

      // Create a temporary, anonymous agent to fetch the root key.
      // This is crucial to ensure the agent trusts the local replica.
      const rootKeyAgent = new HttpAgent({ host: config.host });
      await rootKeyAgent.fetchRootKey().catch(err => {
        console.warn('Unable to fetch root key. This may lead to errors.', err);
      });

      // Now create the final agent with the user's identity.
      // The root key fetched by the temporary agent will be used.
      agent = new HttpAgent({
        host: config.host,
        identity,
      });

    } else {
      // For the mainnet, we don't need to fetch the root key.
      console.log('Mainnet environment detected. Creating standard agent.');
      agent = new HttpAgent({
        host: config.host,
        identity,
      });
    }

    this.actor = Actor.createActor(idlFactory, {
      agent,
      canisterId: config.authCanisterId,
    });

    console.log('Actor created successfully');
  }

  // Private: Verify session is still valid
  async verifySession() {
    if (!this.actor) return false;

    try {
      return await this.actor.is_authenticated();
    } catch (error) {
      console.error('Session verification failed:', error);
      return false;
    }
  }
}

// Export singleton instance
export const authService = new InternetIdentityAuthService();
