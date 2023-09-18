use canister_client::generate_update_call;
use post_index_canister::*;

// Updates
generate_update_call!(upgrade_local_post_index_canister_wasm);
