NETWORK=$1
IDENTITY=$2

USER_INDEX_CANISTER_ID=$(dfx --identity $IDENTITY canister --network $NETWORK id user_index)
LOCAL_USER_INDEX_CANISTER_ID=$(dfx --identity $IDENTITY canister --network $NETWORK id local_user_index)
POST_INDEX_CANISTER_ID=$(dfx --identity $IDENTITY canister --network $NETWORK id post_index)
LOCAL_POST_INDEX_CANISTER_ID=$(dfx --identity $IDENTITY canister --network $NETWORK id local_post_index)

echo "Super_admin : $SUPER_ADMIN"

dfx --identity $IDENTITY deploy --network $NETWORK user_index --argument '(record {
    post_index_canister_id = principal "'${POST_INDEX_CANISTER_ID}'";
    local_user_index_canister_ids = vec { principal "'${LOCAL_USER_INDEX_CANISTER_ID}'" };
    local_post_index_canister_ids = vec { principal "'${LOCAL_POST_INDEX_CANISTER_ID}'" };
    super_admin = principal "'${SUPER_ADMIN}'";
    wasm_version = record { major = 1; minor = 0; patch = 0; };
})'

dfx --identity $IDENTITY deploy --network $NETWORK local_user_index --argument '(record {
    user_index_canister_id = principal "'${USER_INDEX_CANISTER_ID}'";
    post_index_canister_id = principal "'${POST_INDEX_CANISTER_ID}'";
    local_post_index_canister_ids = vec { principal "'${LOCAL_POST_INDEX_CANISTER_ID}'" };
    super_admin = principal "'${SUPER_ADMIN}'";
    wasm_version = record { major = 1; minor = 0; patch = 0; };
})'

dfx --identity $IDENTITY deploy --network $NETWORK post_index --argument '(record {
    user_index_canister_id = principal "'${USER_INDEX_CANISTER_ID}'";
    local_post_index_canister_ids = vec { principal "'${LOCAL_POST_INDEX_CANISTER_ID}'" };
    local_user_index_canister_ids = vec { principal "'${LOCAL_USER_INDEX_CANISTER_ID}'" };
    super_admin = principal "'${SUPER_ADMIN}'";
    wasm_version = record { major = 1; minor = 0; patch = 0; };
})'

dfx --identity $IDENTITY deploy --network $NETWORK local_post_index --argument '(record {
    user_index_canister_id = principal "'${USER_INDEX_CANISTER_ID}'";
    post_index_canister_id = principal "'${POST_INDEX_CANISTER_ID}'";
    local_user_index_canister_ids = vec { principal "'${LOCAL_USER_INDEX_CANISTER_ID}'" };
    super_admin = principal "'${SUPER_ADMIN}'";
    wasm_version = record { major = 1; minor = 0; patch = 0; };
})'

dfx --identity $IDENTITY canister --network $NETWORK update-settings $LOCAL_USER_INDEX_CANISTER_ID --add-controller $USER_INDEX_CANISTER_ID
dfx --identity $IDENTITY canister --network $NETWORK update-settings $LOCAL_POST_INDEX_CANISTER_ID --add-controller $POST_INDEX_CANISTER_ID
