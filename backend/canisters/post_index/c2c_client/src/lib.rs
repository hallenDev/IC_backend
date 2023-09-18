use canister_client::generate_c2c_call;
use post_index_canister::*;

// Queries
generate_c2c_call!(c2c_is_nobleblocks_post);

// Updates
generate_c2c_call!(c2c_notify_events);

