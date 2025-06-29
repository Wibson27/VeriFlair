import React, { createContext, useContext, useEffect, useState } from 'react';
import { authService } from '../services/authService';

const AuthContext = createContext(null);

/**
 * Auth Provider Component - Enhanced for better local development support
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
      setError(null);
      console.log('🚀 Initializing VeriFlair authentication...');

      const authenticated = await authService.initialize();

      if (authenticated) {
        console.log('✅ User is authenticated');
        const userPrincipal = authService.getPrincipal();

        // Try to get session, but don't fail if it doesn't work
        let session = null;
        try {
          session = await authService.getCurrentSession();
          console.log('📋 Session loaded:', session);
        } catch (sessionError) {
          console.warn('⚠️ Could not load session (continuing anyway):', sessionError);
        }

        setIsAuthenticated(true);
        setUser(session);
        setPrincipal(userPrincipal);
      } else {
        console.log('❌ User is not authenticated');
      }

      setIsInitialized(true);
    } catch (err) {
      console.error('❌ Auth initialization failed:', err);
      setError(err?.message || 'Authentication initialization failed');
      setIsInitialized(true); // Still mark as initialized so app can continue
    } finally {
      setIsLoading(false);
    }
  };

  const login = async () => {
    try {
      setIsLoading(true);
      setError(null);
      console.log('🔄 Starting VeriFlair login process...');

      const success = await authService.login();

      if (success) {
        console.log('✅ Login successful');
        const userPrincipal = authService.getPrincipal();

        // Try to get session, but don't fail the login if it doesn't work
        let session = null;
        try {
          session = await authService.getCurrentSession();
          console.log('📋 Session after login:', session);
        } catch (sessionError) {
          console.warn('⚠️ Could not load session after login (continuing anyway):', sessionError);
        }

        setIsAuthenticated(true);
        setUser(session);
        setPrincipal(userPrincipal);

        console.log('🎉 Login complete - ready to navigate');
        return true;
      } else {
        throw new Error('Login was not successful');
      }
    } catch (err) {
      console.error('❌ Login failed:', err);
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
      console.log('🔄 Starting logout process...');

      await authService.logout();

      setIsAuthenticated(false);
      setUser(null);
      setPrincipal(null);
      setError(null);

      console.log('✅ Logout complete');
    } catch (err) {
      console.error('❌ Logout failed:', err);
      setError(err?.message || 'Logout failed');

      // Even if logout fails, reset local state
      setIsAuthenticated(false);
      setUser(null);
      setPrincipal(null);
    } finally {
      setIsLoading(false);
    }
  };

  const setGitHubUsername = async (username) => {
    try {
      setError(null);
      console.log('🔄 Setting GitHub username:', username);
      const success = await authService.setGitHubUsername(username);

      if (success) {
        // Try to refresh user session
        try {
          const session = await authService.getCurrentSession();
          setUser(session);
          console.log('✅ GitHub username updated successfully');
        } catch (sessionError) {
          console.warn('⚠️ Could not refresh session after GitHub update:', sessionError);
        }
      }

      return success;
    } catch (err) {
      console.error('❌ GitHub username update failed:', err);
      setError(err?.message || 'Failed to update GitHub username');
      return false;
    }
  };

  const renewSession = async () => {
    try {
      console.log('🔄 Renewing session...');
      const success = await authService.renewSession();

      if (success) {
        // Try to refresh user session
        try {
          const session = await authService.getCurrentSession();
          setUser(session);
          console.log('✅ Session renewed successfully');
        } catch (sessionError) {
          console.warn('⚠️ Could not refresh session after renewal:', sessionError);
        }
      }

      return success;
    } catch (err) {
      console.error('❌ Session renewal failed:', err);
      setError(err?.message || 'Failed to renew session');
      return false;
    }
  };

  const clearError = () => {
    setError(null);
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
      clearError,
    }}>
      {children}
    </AuthContext.Provider>
  );
};

/**
 * useAuth Hook
 */
export const useAuth = () => {
  const context = useContext(AuthContext);
  if (!context) {
    throw new Error('useAuth must be used within an AuthProvider');
  }
  return context;
};