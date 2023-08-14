use crate::model::{
    user_map::UserMap,
    follow_request_map::FollowRequestMap,
};
use canister_state_macros::canister_state;
use local_user_index_canister::Event as LocalUserIndexEvent;
use model::local_user_index_map::LocalUserIndexMap;
use serde::{Deserialize, Serialize};
use types::{CanisterId, NobleId};
use utils::{env::Environment, canister_event_sync_queue::CanisterEventSyncQueue};

mod jobs;
mod guards;
mod lifecycle;
mod model;
mod queries;
mod updates;
mod memory;

pub const USER_LIMIT: usize = 1_000_000;
pub const MAX_USER_DATA_SIZE: usize = 512; // 0.5 KB

canister_state!(RuntimeState);

struct RuntimeState {
    pub env: Box<dyn Environment>,
    pub data: Data,
}

impl RuntimeState {
    pub fn new(env: Box<dyn Environment>, data: Data) -> RuntimeState {
        RuntimeState { env, data }
    }

    pub fn caller_is_local_user_index_canister(&self) -> bool {
        let caller = self.env.caller();
        self.data.local_index_map.contains_key(&caller)
    }

    pub fn caller_is_post_index_canister(&self) -> bool {
        let caller = self.env.caller();
        self.data.post_index_canister_id == caller
    }

    pub fn push_event_to_local_user_index(&mut self, noble_id: NobleId, event: LocalUserIndexEvent) {
        if let Some(canister_id) = self.data.local_index_map.get_index_canister(&noble_id) {
            self.data.user_index_event_sync_queue.push(canister_id, event);
            #[cfg(not(test))]
            jobs::sync_events_to_local_user_index_canisters::start_job_if_required(self);
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Data {
    pub users: UserMap,
    pub follow_requests: FollowRequestMap,
    pub local_index_map: LocalUserIndexMap,
    pub internet_identity_canister_id: CanisterId,
    pub post_index_canister_id: CanisterId,
    pub user_index_event_sync_queue: CanisterEventSyncQueue<LocalUserIndexEvent>,
}

impl Data {
    pub fn new(
        internet_identity_canister_id: CanisterId,
        post_index_canister_id: CanisterId,
    ) -> Self {
        Data {
            users: UserMap::default(),
            follow_requests: FollowRequestMap::default(),
            local_index_map: LocalUserIndexMap::default(),
            internet_identity_canister_id,
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
            follow_requests: FollowRequestMap::default(),
            local_index_map: LocalUserIndexMap::default(),
            internet_identity_canister_id: Principal::anonymous(),
            post_index_canister_id: Principal::anonymous(),
            user_index_event_sync_queue: CanisterEventSyncQueue::default(),
        }
    }
}
