use canister_client::generate_candid_c2c_call;
pub use local_post_index_canister::*;

generate_candid_c2c_call!(new_post);
