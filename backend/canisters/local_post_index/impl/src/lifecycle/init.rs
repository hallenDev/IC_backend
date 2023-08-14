use crate::lifecycle::{init_env, init_state};
use crate::Data;
use canister_logger::set_panic_hook;
use ic_cdk_macros::init;
use local_post_index_canister::init::Args;

#[init]
fn init(args: Args) {
    set_panic_hook();
    let env = init_env();

    let data = Data::new(args.user_index_canister_id, args.post_index_canister_id);

    init_state(env, data);

    ic_cdk::println!("Initialization complete");
}
