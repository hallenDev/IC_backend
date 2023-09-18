use crate::lifecycle::{init_env, init_state};
use crate::Data;
use ic_cdk_macros::init;
use local_user_index_canister::init::Args;
use tracing::info;

#[init]
fn init(args: Args) {
    canister_logger::init(false);
    let env = init_env();

    let data = Data::new(
        args.user_index_canister_id,
        args.post_index_canister_id,
        args.local_post_index_canister_ids,
        args.super_admin,
    );

    init_state(env, data, args.wasm_version);

    info!(version = %args.wasm_version, "Initialization complete");
}
