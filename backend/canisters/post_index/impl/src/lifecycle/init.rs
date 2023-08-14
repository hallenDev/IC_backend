use crate::lifecycle::{init_env, init_state};
use crate::Data;
use canister_logger::set_panic_hook;
use ic_cdk_macros::init;
use post_index_canister::init::Args;
use types::Version;

#[init]
fn init(args: Args) {
    set_panic_hook();
    let env = init_env();

    let mut data = Data::new(args.user_index_canister_id);
    for canister_id in args.local_post_index_canister_ids {
        data.local_index_map.add_index(canister_id, Version::default());
    }

    init_state(env, data);

    ic_cdk::println!("Initialization complete");
}
