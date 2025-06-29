#!/bin/bash

echo "🛠️ VeriFlair Troubleshooting & Recovery"
echo "======================================="

echo ""
echo "1. 🔍 Checking current setup..."

# Check if dfx is running
if ! pgrep -f "dfx start" > /dev/null; then
    echo "❌ DFX is not running"
    echo "🔧 Starting DFX..."
    dfx start --background --clean
else
    echo "✅ DFX is running"
fi

# Check canister status
echo ""
echo "2. 📊 Checking canister status..."

for canister in auth backend nft; do
    echo "Checking $canister canister..."
    if dfx canister status $canister 2>/dev/null; then
        echo "✅ $canister canister exists"
    else
        echo "❌ $canister canister not found"
    fi
done

echo ""
echo "3. 🧹 Clean rebuild option..."
echo "Choose an option:"
echo "1) Clean rebuild all canisters"
echo "2) Just restart DFX"
echo "3) Check logs"
echo "4) Manual canister calls"
echo "5) Reset everything and redeploy"

read -p "Enter your choice (1-5): " choice

case $choice in
    1)
        echo "🔄 Clean rebuilding all canisters..."
        dfx build --clean
        ./deploy-azure.sh
        ;;
    2)
        echo "🔄 Restarting DFX..."
        dfx stop
        dfx start --background --clean
        ;;
    3)
        echo "📋 Checking logs..."
        echo "Auth canister logs:"
        dfx canister logs auth | tail -20
        echo ""
        echo "Backend canister logs:"
        dfx canister logs backend | tail -20
        echo ""
        echo "NFT canister logs:"
        dfx canister logs nft | tail -20
        ;;
    4)
        echo "🔧 Manual canister testing..."
        echo "Testing basic canister calls..."

        echo "Auth health check:"
        dfx canister call auth health_check || echo "❌ Auth call failed"

        echo "Backend health check:"
        dfx canister call backend health_check || echo "❌ Backend call failed"

        echo "NFT health check:"
        dfx canister call nft health_check || echo "❌ NFT call failed"
        ;;
    5)
        echo "🗑️ Complete reset and redeploy..."
        echo "This will destroy all canisters and start fresh."
        read -p "Are you sure? (yes/no): " confirm

        if [ "$confirm" = "yes" ]; then
            echo "Stopping DFX..."
            dfx stop

            echo "Cleaning build artifacts..."
            rm -rf .dfx/

            echo "Rebuilding..."
            dfx start --background --clean
            dfx build

            echo "Redeploying everything..."
            ./deploy-azure.sh
        else
            echo "Reset cancelled."
        fi
        ;;
    *)
        echo "Invalid choice"
        ;;
esac

echo ""
echo "4. 🎯 Common Issues & Solutions:"
echo ""
echo "❌ Problem: 'Wasm module not found'"
echo "✅ Solution: Run option 1 (Clean rebuild)"
echo ""
echo "❌ Problem: 'Authentication required'"
echo "✅ Solution: Run 'dfx canister call auth create_session (null)' first"
echo ""
echo "❌ Problem: Circular dependency errors"
echo "✅ Solution: This is fixed in the new deployment script"
echo ""
echo "❌ Problem: GitHub/Azure API errors"
echo "✅ Solution: Check your API keys and network connection"
echo ""
echo "❌ Problem: Candid interface warnings"
echo "✅ Solution: This is fixed in the new dfx.json"

echo ""
echo "🔗 Helpful commands:"
echo "• Check canister IDs: dfx canister id <canister_name>"
echo "• View real-time logs: dfx canister logs <canister_name> --follow"
echo "• Check canister status: dfx canister status <canister_name>"
echo "• Open Candid UI: dfx canister call --candid <canister_name>"
echo "• Reset identity: dfx identity new test_identity && dfx identity use test_identity"

echo ""
echo "🎉 If problems persist:"
echo "1. Check the DEPLOY_INSTRUCTIONS.md file"
echo "2. Ensure all files are correctly placed"
echo "3. Try the complete reset option (5)"
echo "4. Check that your GitHub OAuth app callback URL is correct"