NETWORK=${1:-local}
LOCAL_USER_INDEX_CANISTER_ID=$(dfx canister --network $NETWORK id local_user_index)

icx-proxy --fetch-root-key --address 127.0.0.1:8453 --dns-alias 127.0.0.1:${LOCAL_USER_INDEX_CANISTER_ID} --replica http://localhost:8080 -v
