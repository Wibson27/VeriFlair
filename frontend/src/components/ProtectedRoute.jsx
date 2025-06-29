import React, { useEffect } from 'react';
import { useNavigate } from 'react-router-dom';
import { useAuth } from '../hooks/useAuth';

/**
 * ProtectedRoute Component
 * Ensures only authenticated users can access protected pages
 * Follows VeriFlair security architecture from Section 5.1
 */
const ProtectedRoute = ({ children, redirectTo = '/' }) => {
  const { isInitialized, isAuthenticated, isLoading } = useAuth();
  const navigate = useNavigate();

  useEffect(() => {
    // Only redirect if auth is fully initialized and user is not authenticated
    if (isInitialized && !isAuthenticated && !isLoading) {
      console.log('User not authenticated, redirecting to:', redirectTo);
      navigate(redirectTo);
    }
  }, [isInitialized, isAuthenticated, isLoading, navigate, redirectTo]);

  // Show loading while initializing or authenticating
  if (!isInitialized || isLoading) {
    return (
      <div className="min-h-screen bg-brand-dark flex items-center justify-center">
        <div className="text-center text-white">
          <div className="inline-block animate-spin rounded-full h-8 w-8 border-b-2 border-white mb-4"></div>
          <p>Verifying authentication...</p>
        </div>
      </div>
    );
  }

  // Show loading if still checking authentication
  if (!isAuthenticated) {
    return (
      <div className="min-h-screen bg-brand-dark flex items-center justify-center">
        <div className="text-center text-white">
          <h2 className="text-2xl font-bold mb-4">Authentication Required</h2>
          <p className="text-gray-400 mb-6">Please authenticate with Internet Identity to access this page.</p>
          <button
            onClick={() => navigate('/')}
            className="px-6 py-3 bg-blue-600 text-white rounded-full hover:bg-blue-700 transition-colors"
          >
            Go to Home
          </button>
        </div>
      </div>
    );
  }

  // User is authenticated, render the protected content
  return children;
};

export default ProtectedRoute;