#!/bin/bash

echo "🔧 Quick Fix: Updating Auth Canister"
echo "==================================="

echo "1. 📦 Rebuilding auth canister..."
dfx build auth

echo "2. 🔄 Upgrading auth canister..."
dfx canister install auth --mode upgrade

echo "3. ✅ Testing auth canister..."
dfx canister call auth health_check

echo "4. 🔐 Testing new session creation..."
dfx canister call auth create_test_session

echo "5. ✅ Verifying authentication..."
dfx canister call auth is_authenticated

echo ""
echo "✅ Auth fix completed!"
echo ""
echo "🎯 Now run the updated test:"
echo "./test-azure.sh"