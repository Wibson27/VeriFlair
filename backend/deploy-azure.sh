#!/bin/bash

echo "🚀 Deploying VeriFlair with Azure OpenAI Integration"
echo "================================================="

# Your Azure OpenAI Configuration
AZURE_API_KEY="5MPwbn3V9D40xEzrCp6cP72jAtZxHk1kuYbXYJMSkdaA2mdRvGpBJQQJ99BFACHYHv6XJ3w3AAAAACOGBZeH"
AZURE_ENDPOINT="https://rifqi-mbufwy7f-eastus2.openai.azure.com"

# Your GitHub OAuth Configuration
GITHUB_CLIENT_ID="Ov23lilX1z6LtvGmM8x3"
GITHUB_CLIENT_SECRET="4e3613cadfa88427c1a09a6a715f125af2b20326"

# Stop any existing dfx and start fresh
dfx stop
dfx start --background --clean

echo "📦 Building all canisters..."
dfx build

echo "🔐 Deploying auth canister..."
dfx deploy auth

echo "🏭 Deploying NFT canister (without backend dependency)..."
dfx deploy nft

echo "⚙️ Getting canister IDs..."
AUTH_CANISTER_ID=$(dfx canister id auth)
NFT_CANISTER_ID=$(dfx canister id nft)

echo "Auth Canister ID: $AUTH_CANISTER_ID"
echo "NFT Canister ID: $NFT_CANISTER_ID"

echo "🤖 Deploying backend canister with all integrations..."
dfx deploy backend --argument "(
  principal \"$AUTH_CANISTER_ID\",
  principal \"$NFT_CANISTER_ID\",
  \"$GITHUB_CLIENT_ID\",
  \"$GITHUB_CLIENT_SECRET\",
  opt \"$AZURE_API_KEY\",
  opt \"$AZURE_ENDPOINT\"
)"

echo "🔗 Setting backend reference in NFT canister..."
BACKEND_CANISTER_ID=$(dfx canister id backend)
dfx canister call nft set_backend_canister "(principal \"$BACKEND_CANISTER_ID\")"

echo "✅ All canisters deployed with Azure OpenAI and GitHub OAuth!"
echo ""
echo "📋 Configuration Summary:"
echo "├── Auth Canister: $AUTH_CANISTER_ID"
echo "├── Backend Canister: $BACKEND_CANISTER_ID"
echo "├── NFT Canister: $NFT_CANISTER_ID"
echo "├── GitHub Client ID: $GITHUB_CLIENT_ID"
echo "├── Azure OpenAI Endpoint: $AZURE_ENDPOINT"
echo "└── Azure Deployment: gpt-35-turbo"
echo ""
echo "🌐 Candid UI URLs:"
echo "├── Auth: http://127.0.0.1:4943/?canisterId=$(dfx canister id __Candid_UI)&id=$AUTH_CANISTER_ID"
echo "├── Backend: http://127.0.0.1:4943/?canisterId=$(dfx canister id __Candid_UI)&id=$BACKEND_CANISTER_ID"
echo "└── NFT: http://127.0.0.1:4943/?canisterId=$(dfx canister id __Candid_UI)&id=$NFT_CANISTER_ID"

echo ""
echo "🧪 Testing deployments..."
echo "Testing auth canister..."
dfx canister call auth health_check

echo "Testing backend canister..."
dfx canister call backend health_check

echo "Testing NFT canister..."
dfx canister call nft health_check

echo ""
echo "🔧 Environment Setup Complete!"
echo "Your Azure OpenAI + GitHub OAuth integration is ready!"
echo ""
echo "🎯 Next Steps:"
echo "1. ✅ All canisters deployed successfully"
echo "2. ✅ GitHub OAuth configured"
echo "3. ✅ Azure OpenAI integration active"
echo "4. ✅ Ready for frontend integration"

# Create environment file for frontend
cat > ../frontend/.env.local << EOF
REACT_APP_AUTH_CANISTER_ID=$AUTH_CANISTER_ID
REACT_APP_BACKEND_CANISTER_ID=$BACKEND_CANISTER_ID
REACT_APP_NFT_CANISTER_ID=$NFT_CANISTER_ID
REACT_APP_DFX_NETWORK=local
REACT_APP_GITHUB_CLIENT_ID=$GITHUB_CLIENT_ID
EOF

echo ""
echo "✅ Frontend environment file created!"
echo "Your BadgeCard components are ready to display Azure AI-powered badges!"