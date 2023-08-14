use canister_state_macros::canister_state;
use model::post_map::PostMap;
use post_index_canister::Event as PostIndexEvent;
use user_index_canister::Event as UserIndexEvent;
use serde::{Deserialize, Serialize};
use types::CanisterId;
use utils::{env::Environment, canister_event_sync_queue::CanisterEventSyncQueue};

mod guards;
mod lifecycle;
mod model;
mod queries;
mod updates;
mod memory;
mod jobs;

// MAX_POST_DATA_SIZE * POST_LIMIT < 4 GB
pub const MAX_POST_DATA_SIZE: u32 = 5 * 1_024 * 1_024; // 5 MB
pub const POST_LIMIT: usize = 800;

canister_state!(RuntimeState);

struct RuntimeState {
    pub env: Box<dyn Environment>,
    pub data: Data,
}

impl RuntimeState {
    pub fn new(env: Box<dyn Environment>, data: Data) -> RuntimeState {
        RuntimeState { env, data }
    }

    pub fn caller_is_post_index_canister(&self) -> bool {
        let caller = self.env.caller();
        self.data.post_index_canister_id == caller
    }

    #[allow(dead_code)]
    pub fn push_event_to_post_index(&mut self, event: PostIndexEvent) {
        self.data
        .post_index_event_sync_queue
        .push(self.data.post_index_canister_id, event);
        #[cfg(not(test))]
        jobs::sync_events_to_post_index_canister::start_job_if_required(self);
    }

    #[allow(dead_code)]
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
    pub posts: PostMap,
    pub user_index_canister_id: CanisterId,
    pub post_index_canister_id: CanisterId,
    pub post_index_event_sync_queue: CanisterEventSyncQueue<PostIndexEvent>,
    pub user_index_event_sync_queue: CanisterEventSyncQueue<UserIndexEvent>,
}

impl Data {
    pub fn new(
        user_index_canister_id: CanisterId,
        post_index_canister_id: CanisterId,
    ) -> Self {
        Data {
            posts: PostMap::default(),
            user_index_canister_id,
            post_index_canister_id,
            post_index_event_sync_queue: CanisterEventSyncQueue::default(),
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
            posts: PostMap::default(),
            user_index_canister_id: Principal::anonymous(),
            post_index_canister_id: Principal::anonymous(),
            post_index_event_sync_queue: CanisterEventSyncQueue::default(),
            user_index_event_sync_queue: CanisterEventSyncQueue::default(),
        }
    }
}
