#!/bin/sh

# Pass in network name, IC url, identity name, canister name, and version
# eg './upgrade-canister.sh local http://127.0.0.1:8080/ nobleblocks user_index 1.0.0'

NETWORK=$1
IC_URL=$2
IDENTITY=$3
CANISTER_NAME=$4
VERSION=$5

SCRIPT=$(readlink -f "$0")
SCRIPT_DIR=$(dirname "$SCRIPT")
cd $SCRIPT_DIR/..

USER_INDEX_CANISTER_ID=$(dfx --identity $IDENTITY canister --network $NETWORK id user_index)
POST_INDEX_CANISTER_ID=$(dfx --identity $IDENTITY canister --network $NETWORK id post_index)

cargo run --release \
  --manifest-path backend/canister_upgrader/Cargo.toml -- \
  --url $IC_URL \
  --controller $IDENTITY \
  --user-index $USER_INDEX_CANISTER_ID \
  --post-index $POST_INDEX_CANISTER_ID \
  --canister-to-upgrade $CANISTER_NAME \
  --version $VERSION