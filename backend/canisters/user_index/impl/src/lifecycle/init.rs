use crate::lifecycle::{init_env, init_state};
use crate::Data;
use ic_cdk_macros::init;
use tracing::info;
use types::Version;
use user_index_canister::init::Args;

#[init]
fn init(args: Args) {
    canister_logger::init(false);
    let env = init_env();

    let mut data = Data::new(
        args.post_index_canister_id,
        args.local_post_index_canister_ids.into_iter().collect(),
        args.super_admin,
    );
    for canister_id in args.local_user_index_canister_ids {
        data.local_index_map.add_index(canister_id, Version::default());
    }

    init_state(env, data, args.wasm_version);

    info!(version = %args.wasm_version, "Initialization complete");
}
