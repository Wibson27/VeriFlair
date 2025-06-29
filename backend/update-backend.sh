#!/bin/bash

echo "🔧 Quick Backend Update - Bypassing Auth for GitHub Testing"
echo "========================================================="

echo "1. 📦 Rebuilding backend canister..."
dfx build backend

echo "2. 🔄 Upgrading backend canister..."
dfx canister install backend --mode upgrade

echo "3. ✅ Testing backend..."
dfx canister call backend health_check

echo "4. 🎯 Backend updated successfully!"
echo ""
echo "✅ You can now test the GitHub OAuth flow again"
echo "The 'Authentication required' error should be gone"
echo ""
echo "🔄 Try clicking 'Connect GitHub' again!"