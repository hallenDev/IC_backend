use canister_client::{generate_candid_c2c_call, generate_c2c_call};
pub use local_user_index_canister::*;

// Queries
generate_candid_c2c_call!(user);
generate_candid_c2c_call!(check_password);
generate_c2c_call!(c2c_get_block_me_users);
generate_c2c_call!(c2c_get_block_users);
generate_c2c_call!(c2c_get_following_list);

// Updates
generate_candid_c2c_call!(register_user);
generate_c2c_call!(c2c_notify_events);
