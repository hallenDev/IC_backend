IDENTITY=$1

export NETWORK=ic

# Create the NobleBlocks canisters
dfx --identity $IDENTITY canister --network $NETWORK create user_index
dfx --identity $IDENTITY canister --network $NETWORK create local_user_index
dfx --identity $IDENTITY canister --network $NETWORK create post_index
dfx --identity $IDENTITY canister --network $NETWORK create local_post_index

export SUPER_ADMIN="oww3o-vtefp-gjjcy-lvvqk-x5ytz-au4h6-ynzw5-uqy24-4vi5t-5cpv7-uqe"

./scripts/deploy.sh $NETWORK $IDENTITY
