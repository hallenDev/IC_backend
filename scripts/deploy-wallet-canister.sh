IDENTITY=$1
AMOUNT=$2

export NETWORK=ic

# Creating a new canister with a specified amount using the current identity's principal
dfx --identity $IDENTITY ledger --network $NETWORK create-canister $(dfx --identity $IDENTITY identity get-principal) --amount $AMOUNT

# Deploying the wallet using the current identity's principal
dfx --identity $IDENTITY identity --network $NETWORK deploy-wallet $(dfx --identity $IDENTITY canister id lookup)
