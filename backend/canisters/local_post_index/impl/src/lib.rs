use std::{cell::RefCell, collections::HashSet};

use canister_state_macros::canister_state;
use candid::{Principal, CandidType};
use model::post_map::PostMap;
use post_index_canister::Event as PostIndexEvent;
use user_index_canister::Event as UserIndexEvent;
use serde::{Deserialize, Serialize};
use types::{CanisterId, TimestampMillis, Cycles, Timestamped, Version};
use utils::{env::Environment, canister_event_sync_queue::CanisterEventSyncQueue};

mod guards;
mod lifecycle;
mod model;
mod queries;
mod updates;
mod memory;
mod jobs;

pub const MAX_POST_DATA_SIZE: u32 = 5 * 1_024 * 1_024; // 5 MB
pub const POST_LIMIT: usize = 200;
pub const MAX_COMMENT_LENGTH: usize = 200;
pub const MAX_TITLE_LENGTH: usize = 50;
pub const MAX_DESCRIPTION_LENGTH: usize = 1_000;

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

    pub fn caller_is_post_index_canister(&self) -> bool {
        let caller = self.env.caller();
        self.data.post_index_canister_id == caller
    }

    pub fn caller_is_super_admin(&self) -> bool {
        let caller = self.env.caller();
        self.data.super_admin == caller
    }

    pub fn push_event_to_post_index(&mut self, event: PostIndexEvent) {
        self.data
        .post_index_event_sync_queue
        .push(self.data.post_index_canister_id, event);
        #[cfg(not(test))]
        jobs::sync_events_to_post_index_canister::start_job_if_required(self);
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
            post_count: self.data.posts.len(),
            wasm_version: WASM_VERSION.with(|v| **v.borrow()),
            canister_ids: CanisterIds {
                user_index_canister_id: self.data.user_index_canister_id,
                post_index_canister_id: self.data.post_index_canister_id,
                local_user_index_canister_ids: self.data.local_user_index_canister_ids.clone(),
            },
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Data {
    pub posts: PostMap,
    pub user_index_canister_id: CanisterId,
    pub post_index_canister_id: CanisterId,
    pub post_index_event_sync_queue: CanisterEventSyncQueue<PostIndexEvent>,
    pub user_index_event_sync_queue: CanisterEventSyncQueue<UserIndexEvent>,
    pub super_admin: Principal,
    #[serde(default = "default_local_user_index_canister_ids")]
    pub local_user_index_canister_ids: HashSet<CanisterId>,
}

fn default_local_user_index_canister_ids() -> HashSet<CanisterId>{
    let mut result = HashSet::new();
    result.insert(CanisterId::from_text("ok64i-eiaaa-aaaap-abjba-cai").unwrap());
    result
}

impl Data {
    pub fn new(
        user_index_canister_id: CanisterId,
        post_index_canister_id: CanisterId,
        local_user_index_canister_ids: HashSet<CanisterId>,
        super_admin: Principal,
    ) -> Self {
        Data {
            posts: PostMap::default(),
            user_index_canister_id,
            post_index_canister_id,
            super_admin,
            post_index_event_sync_queue: CanisterEventSyncQueue::default(),
            user_index_event_sync_queue: CanisterEventSyncQueue::default(),
            local_user_index_canister_ids,
        }
    }
}

#[cfg(test)]
impl Default for Data {
    fn default() -> Data {
        Data {
            posts: PostMap::default(),
            user_index_canister_id: Principal::anonymous(),
            post_index_canister_id: Principal::anonymous(),
            super_admin: Principal::anonymous(),
            post_index_event_sync_queue: CanisterEventSyncQueue::default(),
            user_index_event_sync_queue: CanisterEventSyncQueue::default(),
            local_user_index_canister_ids: HashSet::default(),
        }
    }
}

#[derive(CandidType, Serialize, Debug)]
pub struct Metrics {
    pub now: TimestampMillis,
    pub memory_used: u64,
    pub cycles_balance: Cycles,
    pub post_count: usize,
    pub wasm_version: Version,
    pub canister_ids: CanisterIds,
}

#[derive(CandidType, Serialize, Debug)]
pub struct CanisterIds {
    pub user_index_canister_id: CanisterId,
    pub post_index_canister_id: CanisterId,
    pub local_user_index_canister_ids: HashSet<CanisterId>,
}
