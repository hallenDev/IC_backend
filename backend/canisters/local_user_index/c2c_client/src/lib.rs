use canister_client::{generate_candid_c2c_call, generate_c2c_call};
pub use local_user_index_canister::*;

// Updates
generate_candid_c2c_call!(register_user);
generate_candid_c2c_call!(register_user_with_internet_identity);
generate_candid_c2c_call!(register_user_with_google);
generate_c2c_call!(c2c_notify_events);
