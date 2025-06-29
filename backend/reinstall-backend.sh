#!/bin/bash

echo "🔄 Reinstalling Backend Canister with Correct Arguments"
echo "======================================================"

echo "1. 📋 Getting canister IDs..."
AUTH_CANISTER_ID=$(dfx canister id auth)
NFT_CANISTER_ID=$(dfx canister id nft)

echo "Auth Canister: $AUTH_CANISTER_ID"
echo "NFT Canister: $NFT_CANISTER_ID"

echo "2. 🔄 Reinstalling backend with all arguments..."
dfx canister install backend --mode reinstall --argument "(
  principal \"$AUTH_CANISTER_ID\",
  principal \"$NFT_CANISTER_ID\",
  \"Ov23lilX1z6LtvGmM8x3\",
  \"4e3613cadfa88427c1a09a6a715f125af2b20326\",
  opt \"5MPwbn3V9D40xEzrCp6cP72jAtZxHk1kuYbXYJMSkdaA2mdRvGpBJQQJ99BFACHYHv6XJ3w3AAAAACOGBZeH\",
  opt \"https://rifqi-mbufwy7f-eastus2.openai.azure.com\"
)"

echo "3. ✅ Testing backend..."
dfx canister call backend health_check

echo ""
echo "✅ Backend reinstalled successfully!"
echo "🔄 Try the GitHub OAuth flow again!"chmod +x reinstall-backend.sh
./reinstall-backend.sh