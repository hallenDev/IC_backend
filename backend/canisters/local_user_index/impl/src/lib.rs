use crate::model::user_map::UserMap;
use canister_state_macros::canister_state;
use user_index_canister::Event as UserIndexEvent;
use serde::{Deserialize, Serialize};
use types::CanisterId;
use utils::env::Environment;
use utils::canister_event_sync_queue::CanisterEventSyncQueue;

mod guards;
mod lifecycle;
mod model;
mod queries;
mod updates;
mod memory;
mod jobs;

// MAX_USER_DATA_SIZE * USER_LIMIT < 4 GB
pub const MAX_USER_DATA_SIZE: u32 = 5 * 1_024 * 1_024; // 5 MB
pub const USER_LIMIT: usize = 800;
pub const MAX_BIO_LENGTH: usize = 250;

canister_state!(RuntimeState);

struct RuntimeState {
    pub env: Box<dyn Environment>,
    pub data: Data,
}

impl RuntimeState {
    pub fn new(env: Box<dyn Environment>, data: Data) -> RuntimeState {
        RuntimeState { env, data }
    }

    pub fn caller_is_user_index_canister(&self) -> bool {
        let caller = self.env.caller();
        self.data.user_index_canister_id == caller
    }

    pub fn caller_is_post_index_canister(&self) -> bool {
        let caller = self.env.caller();
        self.data.post_index_canister_id == caller
    }

    pub fn push_event_to_user_index(&mut self, event: UserIndexEvent) {
        self.data
        .user_index_event_sync_queue
        .push(self.data.user_index_canister_id, event);
        #[cfg(not(test))]
        jobs::sync_events_to_user_index_canister::start_job_if_required(self);
    }
}

#[derive(Serialize, Deserialize)]
struct Data {
    pub users: UserMap,
    pub user_index_canister_id: CanisterId,
    pub post_index_canister_id: CanisterId,
    pub user_index_event_sync_queue: CanisterEventSyncQueue<UserIndexEvent>,
}

impl Data {
    pub fn new(
        user_index_canister_id: CanisterId,
        post_index_canister_id: CanisterId,
    ) -> Self {
        Data {
            users: UserMap::default(),
            user_index_canister_id,
            post_index_canister_id,
            user_index_event_sync_queue: CanisterEventSyncQueue::default(),
        }
    }
}

#[cfg(test)]
use candid::Principal;

#[cfg(test)]
impl Default for Data {
    fn default() -> Data {
        Data {
            users: UserMap::default(),
            user_index_canister_id: Principal::anonymous(),
            post_index_canister_id: Principal::anonymous(),
            user_index_event_sync_queue: CanisterEventSyncQueue::default(),
        }
    }
}
