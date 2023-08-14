NETWORK=${1:-local}
IDENTITY=${2:-default}

INTERNET_IDENTITY_CANISTER_ID=qhbym-qaaaa-aaaaa-aaafq-cai

# Create the OpenChat canisters
dfx --identity $IDENTITY canister create --no-wallet --with-cycles 100000000000000 user_index
dfx --identity $IDENTITY canister create --no-wallet --with-cycles 100000000000000 local_user_index
dfx --identity $IDENTITY canister create --no-wallet --with-cycles 100000000000000 post_index
dfx --identity $IDENTITY canister create --no-wallet --with-cycles 100000000000000 local_post_index

USER_INDEX_CANISTER_ID=$(dfx canister --network $NETWORK id user_index)
LOCAL_USER_INDEX_CANISTER_ID=$(dfx canister --network $NETWORK id local_user_index)
POST_INDEX_CANISTER_ID=$(dfx canister --network $NETWORK id post_index)
LOCAL_POST_INDEX_CANISTER_ID=$(dfx canister --network $NETWORK id local_post_index)

dfx deploy --network $NETWORK user_index --argument '(record {
    internet_identity_canister_id=principal "'${INTERNET_IDENTITY_CANISTER_ID}'";
    post_index_canister_id=principal "'${POST_INDEX_CANISTER_ID}'";
    local_user_index_canister_ids = vec { principal "'${LOCAL_USER_INDEX_CANISTER_ID}'" }
})'

dfx deploy --network $NETWORK local_user_index --argument '(record {
    user_index_canister_id=principal "'${USER_INDEX_CANISTER_ID}'";
    post_index_canister_id=principal "'${POST_INDEX_CANISTER_ID}'";
})'

dfx deploy --network $NETWORK post_index --argument '(record {
    user_index_canister_id=principal "'${USER_INDEX_CANISTER_ID}'";
    local_post_index_canister_ids = vec { principal "'${LOCAL_POST_INDEX_CANISTER_ID}'" }
})'

dfx deploy --network $NETWORK local_post_index --argument '(record {
    user_index_canister_id=principal "'${USER_INDEX_CANISTER_ID}'";
    post_index_canister_id=principal "'${POST_INDEX_CANISTER_ID}'";
})'
