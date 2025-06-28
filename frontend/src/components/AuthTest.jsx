import React, { useState } from 'react';
import { useAuth } from '../hooks/useAuth';

const AuthTest = () => {
  const {
    isInitialized,
    isAuthenticated,
    user,
    principal,
    login,
    logout,
    setGitHubUsername,
    renewSession,
    error,
    isLoading
  } = useAuth();

  const [githubInput, setGithubInput] = useState('');

  const handleAuthAction = async () => {
    try {
      if (isAuthenticated) {
        await logout();
      } else {
        await login();
      }
    } catch (error) {
      console.error('Auth action failed:', error);
    }
  };

  const handleSetGitHub = async () => {
    if (githubInput.trim()) {
      const success = await setGitHubUsername(githubInput.trim());
      if (success) {
        setGithubInput('');
      }
    }
  };

  const handleRenewSession = async () => {
    await renewSession();
  };

  if (!isInitialized) {
    return (
      <div className="p-6 bg-gray-100 rounded-lg">
        <p>Initializing authentication...</p>
      </div>
    );
  }

  return (
    <div className="max-w-2xl mx-auto p-6 bg-white rounded-lg shadow-lg">
      <h2 className="text-2xl font-bold mb-6 text-center text-gray-900">
        VeriFlair - Internet Identity Test
      </h2>

      {/* Status */}
      <div className="mb-6 p-4 bg-gray-50 rounded">
        <h3 className="font-semibold mb-2 text-gray-900">Status:</h3>
        <p className="text-gray-700">
          Initialized: <span className="font-mono">{isInitialized ? '✅' : '❌'}</span>
        </p>
        <p className="text-gray-700">
          Authenticated: <span className="font-mono">{isAuthenticated ? '✅' : '❌'}</span>
        </p>
        <p className="text-gray-700">
          Loading: <span className="font-mono">{isLoading ? '⏳' : '✅'}</span>
        </p>
      </div>

      {/* Error Display */}
      {error && (
        <div className="mb-6 p-4 bg-red-100 border border-red-300 rounded text-red-700">
          <strong>Error:</strong> {error}
        </div>
      )}

      {/* Login/Logout Button */}
      <div className="mb-6 text-center">
        <button
          onClick={handleAuthAction}
          disabled={isLoading}
          className={`px-6 py-3 rounded font-medium transition-colors ${
            isAuthenticated
              ? 'bg-red-600 hover:bg-red-700 text-white'
              : 'bg-blue-600 hover:bg-blue-700 text-white'
          } disabled:opacity-50 disabled:cursor-not-allowed`}
        >
          {isLoading ? 'Loading...' : (isAuthenticated ? 'Logout' : 'Login with Internet Identity')}
        </button>
      </div>

      {/* User Information */}
      {isAuthenticated && user && (
        <div className="mb-6 p-4 bg-green-50 border border-green-200 rounded">
          <h3 className="font-semibold mb-3 text-green-800">User Information:</h3>

          <div className="space-y-2 text-sm text-gray-700">
            <div>
              <strong>Principal:</strong>
              <p className="font-mono text-xs break-all bg-white p-2 rounded mt-1">
                {user.user_principal.toString()}
              </p>
            </div>

            <div>
              <strong>GitHub Username:</strong>
              <p className="font-mono">{user.github_username || 'Not set'}</p>
            </div>

            <div>
              <strong>Role:</strong>
              <p className="font-mono">{Object.keys(user.role)[0]}</p>
            </div>

            <div>
              <strong>Session Expires:</strong>
              <p className="font-mono text-xs">
                {new Date(Number(user.expires_at) / 1_000_000).toLocaleString()}
              </p>
            </div>

            <div>
              <strong>Verified:</strong>
              <p className="font-mono">{user.is_verified ? '✅ Yes' : '❌ No'}</p>
            </div>
          </div>
        </div>
      )}

      {/* GitHub Username Input */}
      {isAuthenticated && (
        <div className="mb-6 p-4 bg-blue-50 border border-blue-200 rounded">
          <h3 className="font-semibold mb-3 text-blue-800">Set GitHub Username:</h3>
          <div className="flex gap-2">
            <input
              type="text"
              value={githubInput}
              onChange={(e) => setGithubInput(e.target.value)}
              placeholder="Enter GitHub username"
              className="flex-1 px-3 py-2 border border-gray-300 rounded focus:outline-none focus:border-blue-500 text-gray-900"
            />
            <button
              onClick={handleSetGitHub}
              disabled={!githubInput.trim() || isLoading}
              className="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed"
            >
              Set
            </button>
          </div>
        </div>
      )}

      {/* Session Management */}
      {isAuthenticated && (
        <div className="text-center">
          <button
            onClick={handleRenewSession}
            disabled={isLoading}
            className="px-4 py-2 bg-yellow-600 text-white rounded hover:bg-yellow-700 disabled:opacity-50 disabled:cursor-not-allowed"
          >
            Renew Session
          </button>
        </div>
      )}

      {/* Debug Info */}
      <div className="mt-6 p-4 bg-gray-100 rounded text-xs text-gray-700">
        <h4 className="font-semibold mb-2">Debug Info:</h4>
        <p><strong>Environment:</strong> {process.env.REACT_APP_DFX_NETWORK || 'local'}</p>
        <p><strong>Auth Canister:</strong> {process.env.REACT_APP_AUTH_CANISTER_ID || 'Not set'}</p>
        <p><strong>IC Host:</strong> {process.env.REACT_APP_IC_HOST || 'Default local'}</p>
      </div>
    </div>
  );
};

export default AuthTest;