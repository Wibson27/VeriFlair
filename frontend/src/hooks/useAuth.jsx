import React, { createContext, useContext, useEffect, useState } from 'react';
import { authService } from '../services/authService';

/**
 * @typedef {Object} UserSession
 * @property {import('@dfinity/principal').Principal} user_principal
 * @property {string|null} github_username
 * @property {bigint} created_at
 * @property {bigint} last_active
 * @property {bigint} expires_at
 * @property {Object} role
 * @property {boolean} is_verified
 */

/**
 * @typedef {Object} AuthContextType
 * @property {boolean} isInitialized
 * @property {boolean} isAuthenticated
 * @property {UserSession|null} user
 * @property {import('@dfinity/principal').Principal|null} principal
 * @property {() => Promise<void>} login
 * @property {() => Promise<void>} logout
 * @property {(username: string) => Promise<boolean>} setGitHubUsername
 * @property {() => Promise<boolean>} renewSession
 * @property {string|null} error
 * @property {boolean} isLoading
 */

const AuthContext = createContext(null);

/**
 * Auth Provider Component
 * @param {Object} props
 * @param {React.ReactNode} props.children
 */
export const AuthProvider = ({ children }) => {
  const [isInitialized, setIsInitialized] = useState(false);
  const [isAuthenticated, setIsAuthenticated] = useState(false);
  const [user, setUser] = useState(null);
  const [principal, setPrincipal] = useState(null);
  const [error, setError] = useState(null);
  const [isLoading, setIsLoading] = useState(true);

  // Initialize on mount
  useEffect(() => {
    initializeAuth();
  }, []);

  const initializeAuth = async () => {
    try {
      setIsLoading(true);
      console.log('Initializing VeriFlair authentication...');

      const authenticated = await authService.initialize();

      if (authenticated) {
        console.log('User is authenticated');
        const session = await authService.getCurrentSession();
        const userPrincipal = authService.getPrincipal();

        setIsAuthenticated(true);
        setUser(session);
        setPrincipal(userPrincipal);

        console.log('Session loaded:', session);
      } else {
        console.log('User is not authenticated');
      }

      setIsInitialized(true);
    } catch (err) {
      console.error('Auth initialization failed:', err);
      setError(err?.message || 'Authentication initialization failed');
    } finally {
      setIsLoading(false);
    }
  };

  const login = async () => {
    try {
      setIsLoading(true);
      setError(null);
      console.log('Starting VeriFlair login process...');

      const success = await authService.login();

      if (success) {
        console.log('Login successful');
        const session = await authService.getCurrentSession();
        const userPrincipal = authService.getPrincipal();

        setIsAuthenticated(true);
        setUser(session);
        setPrincipal(userPrincipal);

        console.log('Login complete, session:', session);
      }
    } catch (err) {
      console.error('Login failed:', err);
      const errorMessage = err?.message || 'Login failed';
      setError(errorMessage);
      throw err;
    } finally {
      setIsLoading(false);
    }
  };

  const logout = async () => {
    try {
      setIsLoading(true);
      console.log('Starting logout process...');

      await authService.logout();

      setIsAuthenticated(false);
      setUser(null);
      setPrincipal(null);
      setError(null);

      console.log('Logout complete');
    } catch (err) {
      console.error('Logout failed:', err);
      setError(err?.message || 'Logout failed');
    } finally {
      setIsLoading(false);
    }
  };

  const setGitHubUsername = async (username) => {
    try {
      console.log('Setting GitHub username:', username);
      const success = await authService.setGitHubUsername(username);

      if (success) {
        // Refresh user session
        const session = await authService.getCurrentSession();
        setUser(session);
        console.log('GitHub username updated successfully');
      }

      return success;
    } catch (err) {
      console.error('GitHub username update failed:', err);
      setError(err?.message || 'Failed to update GitHub username');
      return false;
    }
  };

  const renewSession = async () => {
    try {
      console.log('Renewing session...');
      const success = await authService.renewSession();

      if (success) {
        // Refresh user session
        const session = await authService.getCurrentSession();
        setUser(session);
        console.log('Session renewed successfully');
      }

      return success;
    } catch (err) {
      console.error('Session renewal failed:', err);
      setError(err?.message || 'Failed to renew session');
      return false;
    }
  };

  return (
    <AuthContext.Provider value={{
      isInitialized,
      isAuthenticated,
      user,
      principal,
      login,
      logout,
      setGitHubUsername,
      renewSession,
      error,
      isLoading,
    }}>
      {children}
    </AuthContext.Provider>
  );
};

/**
 * useAuth Hook
 * @returns {AuthContextType}
 */
export const useAuth = () => {
  const context = useContext(AuthContext);
  if (!context) {
    throw new Error('useAuth must be used within an AuthProvider');
  }
  return context;
};