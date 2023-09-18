use std::cell::RefCell;
use std::collections::HashSet;

use crate::model::user_map::UserMap;
use canister_state_macros::canister_state;
use candid::{Principal, CandidType};
use user_index_canister::Event as UserIndexEvent;
use serde::{Deserialize, Serialize};
use types::{CanisterId, TimestampMillis, Cycles, Version, Timestamped};
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
pub const USER_LIMIT: usize = 200;
pub const MAX_BIO_LENGTH: usize = 250;
pub const MAX_PHOTO_SIZE: usize = 1_024 * 1_024; // 1MB

thread_local! {
    static WASM_VERSION: RefCell<Timestamped<Version>> = RefCell::default();
}

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

    pub fn caller_is_known_canister(&self) -> bool {
        let caller = self.env.caller();
        self.data.post_index_canister_id == caller ||
        self.data.user_index_canister_id == caller ||
        self.data.local_post_index_canister_ids.contains(&caller)
    }

    pub fn push_event_to_user_index(&mut self, event: UserIndexEvent) {
        self.data
        .user_index_event_sync_queue
        .push(self.data.user_index_canister_id, event);
        #[cfg(not(test))]
        jobs::sync_events_to_user_index_canister::start_job_if_required(self);
    }

    pub fn metrics(&self) -> Metrics {
        Metrics {
            now: self.env.now(),
            memory_used: utils::memory::used(),
            cycles_balance: self.env.cycles_balance(),
            user_count: self.data.users.len(),
            wasm_version: WASM_VERSION.with(|v| **v.borrow()),
            canister_ids: CanisterIds {
                user_index_canister_id: self.data.user_index_canister_id,
                post_index_canister_id: self.data.post_index_canister_id,
                local_post_index_canister_ids: self.data.local_post_index_canister_ids.clone()
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Data {
    pub users: UserMap,
    pub user_index_canister_id: CanisterId,
    pub post_index_canister_id: CanisterId,
    pub super_admin: Principal,
    pub local_post_index_canister_ids: HashSet<CanisterId>,
    pub user_index_event_sync_queue: CanisterEventSyncQueue<UserIndexEvent>,
}

impl Data {
    pub fn new(
        user_index_canister_id: CanisterId,
        post_index_canister_id: CanisterId,
        local_post_index_canister_ids: HashSet<CanisterId>,
        super_admin: Principal,
    ) -> Self {
        Data {
            users: UserMap::default(),
            user_index_canister_id,
            post_index_canister_id,
            local_post_index_canister_ids,
            super_admin,
            user_index_event_sync_queue: CanisterEventSyncQueue::default(),
        }
    }
}

#[cfg(test)]
impl Default for Data {
    fn default() -> Data {
        Data {
            users: UserMap::default(),
            user_index_canister_id: Principal::anonymous(),
            post_index_canister_id: Principal::anonymous(),
            super_admin: Principal::anonymous(),
            local_post_index_canister_ids: HashSet::default(),
            user_index_event_sync_queue: CanisterEventSyncQueue::default(),
        }
    }
}

#[derive(CandidType, Serialize, Debug)]
pub struct Metrics {
    pub now: TimestampMillis,
    pub memory_used: u64,
    pub cycles_balance: Cycles,
    pub user_count: usize,
    pub wasm_version: Version,
    pub canister_ids: CanisterIds,
}

#[derive(CandidType, Serialize, Debug)]
pub struct CanisterIds {
    pub user_index_canister_id: CanisterId,
    pub post_index_canister_id: CanisterId,
    pub local_post_index_canister_ids: HashSet<CanisterId>,
}
