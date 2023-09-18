use canister_client::{generate_query_call, generate_update_call};
use user_index_canister::*;

// Queries
generate_query_call!(check_username);
generate_query_call!(check_email);

// Updates
generate_update_call!(upgrade_local_user_index_canister_wasm);
