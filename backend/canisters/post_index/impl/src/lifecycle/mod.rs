use crate::{mutate_state, Data, RuntimeState, WASM_VERSION};
use std::time::Duration;
use tracing::info;
use types::{Timestamped, Version};
use utils::{
    env::{Environment, canister::CanisterEnv},
    canister::get_random_seed,
};

mod init;
mod pre_upgrade;
mod post_upgrade;

const UPGRADE_BUFFER_SIZE: usize = 1024 * 1024; // 1MB

fn init_env() -> Box<CanisterEnv> {
    ic_cdk_timers::set_timer(Duration::ZERO, reseed_rng);
    Box::default()
}

fn init_state(env: Box<dyn Environment>, data: Data, wasm_version: Version) {
    let now = env.now();
    let state = RuntimeState::new(env, data);

    crate::jobs::start(&state);
    crate::init_state(state);
    WASM_VERSION.with(|v| *v.borrow_mut() = Timestamped::new(wasm_version, now));
}

fn reseed_rng() {
    ic_cdk::spawn(reseed_rng_inner());

    async fn reseed_rng_inner() {
        let seed = get_random_seed().await;
        mutate_state(|state| state.env = Box::new(CanisterEnv::new(seed)));
        info!("Successfully reseeded rng");
    }
}
