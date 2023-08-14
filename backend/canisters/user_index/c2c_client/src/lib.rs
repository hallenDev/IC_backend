use canister_client::{generate_c2c_call, generate_candid_c2c_call};
use user_index_canister::*;

// Queries
generate_c2c_call!(c2c_is_nobleblocks_user);
generate_c2c_call!(c2c_get_local_user_index_canister_id);
generate_candid_c2c_call!(check_email);
generate_candid_c2c_call!(check_username);

// Updates
generate_c2c_call!(c2c_notify_events);

