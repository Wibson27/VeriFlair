#!/bin/bash

echo "🧪 Testing VeriFlair Badge System"
echo "================================="

# Test basic health
echo "1. Health check..."
dfx canister call backend health_check

# Create profile if doesn't exist
echo "2. Creating/getting profile..."
dfx canister call backend create_initial_profile || echo "Profile already exists"

# Test GitHub validation
echo "3. Testing GitHub username validation..."
dfx canister call backend validate_github_username '("torvalds")'

# Get current profile
echo "4. Getting current profile..."
dfx canister call backend get_profile '(null)'

# Get badges
echo "5. Getting current badges..."
dfx canister call backend get_badges '(null)'

# Test leaderboard
echo "6. Testing leaderboard..."
dfx canister call backend get_leaderboard '(opt 5)'

# Test stats
echo "7. Getting system stats..."
dfx canister call backend get_stats

# Test NFT functions
echo "8. Testing NFT canister..."
dfx canister call nft icrc7_total_supply

echo ""
echo "✅ Basic tests completed!"
echo ""
echo "🎯 To test GitHub integration:"
echo "You'll need to complete the OAuth flow through your frontend"
echo "or simulate it with:"
echo "dfx canister call backend connect_github_oauth '({code=\"test\"; state=\"test\"})'"