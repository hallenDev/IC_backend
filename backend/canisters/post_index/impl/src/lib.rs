use canister_state_macros::canister_state;
use model::{post_map::PostMap, local_post_index_map::LocalPostIndexMap};
use serde::{Deserialize, Serialize};
use types::CanisterId;
use utils::env::Environment;

mod jobs;
mod guards;
mod lifecycle;
mod model;
mod queries;
mod updates;
mod memory;

pub const POST_LIMIT: usize = 100_000;
pub const MAX_POST_DATA_SIZE: usize = 10_240;
pub const MAX_TITLE_LENGTH: usize = 50;
pub const MAX_DESCRIPTION_LENGTH: usize = 1_000;

canister_state!(RuntimeState);

struct RuntimeState {
    pub env: Box<dyn Environment>,
    pub data: Data,
}

impl RuntimeState {
    pub fn new(env: Box<dyn Environment>, data: Data) -> RuntimeState {
        RuntimeState { env, data }
    }
}

#[derive(Serialize, Deserialize)]
struct Data {
    pub posts: PostMap,
    pub local_index_map: LocalPostIndexMap,
    pub user_index_canister_id: CanisterId,
}

impl Data {
    pub fn new(
        user_index_canister_id: CanisterId,
    ) -> Self {
        Data {
            posts: PostMap::default(),
            local_index_map: LocalPostIndexMap::default(),
            user_index_canister_id,
        }
    }
}

#[cfg(test)]
use candid::Principal;

#[cfg(test)]
impl Default for Data {
    fn default() -> Data {
        Data {
            posts: PostMap::default(),
            user_index_canister_id: Principal::anonymous(),
            local_index_map: LocalPostIndexMap::default(),
        }
    }
}
