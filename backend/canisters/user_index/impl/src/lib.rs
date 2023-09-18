use std::{collections::HashSet, cell::RefCell};

use crate::model::{
    user_map::UserMap,
    follow_request_map::FollowRequestMap,
};
use canister_state_macros::canister_state;
use candid::{Principal, CandidType};
use local_user_index_canister::Event as LocalUserIndexEvent;
use post_index_canister::Event as PostIndexEvent;
use model::{local_user_index_map::{LocalUserIndexMap, LocalUserIndex}, temp_map::TempMap};
use serde::{Deserialize, Serialize};
use tracing::info;
use types::{CanisterId, NobleId, TimestampMillis, Cycles, CanisterWasm, Timestamped, Version};
use user_index_canister::EmailEvent;
use utils::{env::Environment, canister_event_sync_queue::CanisterEventSyncQueue, email_event_sync_queue::EmailEventSyncQueue, canister::{CanistersRequiringUpgrade, FailedUpgradeCount}, consts::{CYCLES_REQUIRED_FOR_UPGRADE, DEV_TEAM_PRINCIPAL}};

mod jobs;
mod guards;
mod lifecycle;
mod model;
mod queries;
mod updates;
mod memory;

const LOCAL_USER_INDEX_CANISTER_INITIAL_CYCLES_BALANCE: Cycles = CYCLES_REQUIRED_FOR_UPGRADE + LOCAL_USER_INDEX_CANISTER_TOP_UP_AMOUNT; // 3.08T cycles
const LOCAL_USER_INDEX_CANISTER_TOP_UP_AMOUNT: Cycles = 3_000_000_000_000; // 3T cycles

pub const USER_LIMIT: usize = 1_000_000;
pub const LOCAL_USER_LIMIT: usize = 200;
pub const MAX_USER_DATA_SIZE: usize = 512; // 0.5 KB
pub const INFO_EMAIL: &'static str = "info@nobleblocks.com";
pub const FEEDBACK_LIMIT: usize = 2_500;

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

    pub fn caller_is_local_user_index_canister(&self) -> bool {
        let caller = self.env.caller();
        self.data.local_index_map.contains_key(&caller)
    }
    
    pub fn caller_is_known_canister(&self) -> bool {
        let caller = self.env.caller();
        self.data.local_index_map.contains_key(&caller) ||
        self.data.local_post_index_canister_ids.contains(&caller) ||
        self.data.post_index_canister_id == caller
    }

    pub fn caller_is_governance_principal(&self) -> bool {
        let caller = self.env.caller();
        DEV_TEAM_PRINCIPAL == caller ||
        self.data.super_admin == caller
    }

    pub fn push_event_to_local_user_index(&mut self, noble_id: NobleId, event: LocalUserIndexEvent) {
        if let Some(canister_id) = self.data.local_index_map.get_index_canister(&noble_id) {
            self.data.user_index_event_sync_queue.push(canister_id, event);
            #[cfg(not(test))]
            jobs::sync_events_to_local_user_index_canisters::start_job_if_required(self);
        }
    }

    pub fn push_event_to_post_index(&mut self, event: PostIndexEvent) {
        self.data
        .post_index_event_sync_queue
        .push(self.data.post_index_canister_id, event);
        #[cfg(not(test))]
        jobs::sync_events_to_post_index_canister::start_job_if_required(self);
    }

    pub fn push_event_to_all_local_user_index(&mut self, event: LocalUserIndexEvent) {
        self.data.local_index_map.iter().for_each(|(canisster_id, ..)| {
            self.data.user_index_event_sync_queue.push(*canisster_id, event.clone());
        });
        #[cfg(not(test))]
        jobs::sync_events_to_local_user_index_canisters::start_job_if_required(self);
    }

    pub fn push_event_to_send_email(&mut self, email: &str, event: EmailEvent) {
        info!("from: {email} {:?}", event);
        ic_cdk::println!("from: {email} {:?}", event);
        self.data.email_event_sync_queue.push(String::from(email), event);
        #[cfg(not(test))]
        jobs::sync_events_to_send_email::start_job_if_required(self);
    }

    pub fn metrics(&self) -> Metrics {
        let canister_upgrades_metrics = self.data.canisters_requiring_upgrade.metrics();
        Metrics {
            now: self.env.now(),
            memory_used: utils::memory::used(),
            cycles_balance: self.env.cycles_balance(),
            total_users: self.data.users.len(),
            wasm_version: WASM_VERSION.with(|v| **v.borrow()),
            canister_upgrades_completed: canister_upgrades_metrics.completed,
            canister_upgrades_failed: canister_upgrades_metrics.failed,
            canister_upgrades_pending: canister_upgrades_metrics.pending as u64,
            canister_upgrades_in_progress: canister_upgrades_metrics.in_progress as u64,
            governance_principals: self.data.governance_principals.iter().copied().collect(),
            local_user_index_wasm_version: self.data.local_user_index_canister_wasm_for_new_canisters.version,
            platform_moderators: self.data.platform_moderators.len() as u8,
            platform_operators: self.data.platform_operators.len() as u8,
            user_index_events_queue_length: self.data.user_index_event_sync_queue.len(),
            local_user_indexes: self.data.local_index_map.iter().map(|(c, i)| (*c, i.clone())).collect(),
            total_cycles_spent_on_canisters: self.data.total_cycles_spent_on_canisters,
            canister_ids: CanisterIds {
                post_index_canister_id: self.data.post_index_canister_id,
                local_post_index_canister_ids: self.data.local_post_index_canister_ids.clone()
            },
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Data {
    pub users: UserMap,
    pub temps: TempMap,
    pub follow_requests: FollowRequestMap,
    pub local_index_map: LocalUserIndexMap,
    pub post_index_canister_id: CanisterId,
    pub local_post_index_canister_ids: HashSet<CanisterId>,
    pub super_admin: Principal,
    pub user_index_event_sync_queue: CanisterEventSyncQueue<LocalUserIndexEvent>,
    #[serde(default)]
    pub post_index_event_sync_queue: CanisterEventSyncQueue<PostIndexEvent>,
    pub email_event_sync_queue: EmailEventSyncQueue<EmailEvent>,
    #[serde(default)]
    pub local_user_index_canister_wasm_for_new_canisters: CanisterWasm,
    #[serde(default)]
    pub local_user_index_canister_wasm_for_upgrades: CanisterWasm,
    #[serde(default)]
    pub canisters_requiring_upgrade: CanistersRequiringUpgrade,
    #[serde(default)]
    pub governance_principals: HashSet<Principal>,
    #[serde(default)]
    pub platform_moderators: HashSet<NobleId>,
    #[serde(default)]
    pub platform_operators: HashSet<NobleId>,
    #[serde(default)]
    pub total_cycles_spent_on_canisters: Cycles,
}

impl Data {
    pub fn new(
        post_index_canister_id: CanisterId,
        local_post_index_canister_ids: HashSet<CanisterId>,
        super_admin: Principal,
    ) -> Self {
        Data {
            users: UserMap::default(),
            temps: TempMap::default(),
            follow_requests: FollowRequestMap::default(),
            local_index_map: LocalUserIndexMap::default(),
            post_index_canister_id,
            local_post_index_canister_ids,
            super_admin,
            user_index_event_sync_queue: CanisterEventSyncQueue::default(),
            post_index_event_sync_queue: CanisterEventSyncQueue::default(),
            email_event_sync_queue: EmailEventSyncQueue::default(),
            local_user_index_canister_wasm_for_new_canisters: CanisterWasm::default(),
            local_user_index_canister_wasm_for_upgrades: CanisterWasm::default(),
            canisters_requiring_upgrade: CanistersRequiringUpgrade::default(),
            governance_principals: HashSet::default(),
            platform_moderators: HashSet::default(),
            platform_operators: HashSet::default(),
            total_cycles_spent_on_canisters: Cycles::default(),
        }
    }

    pub fn get_anonymous_username(&self) -> String {
        let mut id = 1;
        loop {
            let temp  = format!("user{id:0>5}");
            if !self.users.does_username_exist(&temp) && !self.temps.does_username_exist(&temp) {
                return temp;
            }
            id += 1;
        }
    }
}

#[cfg(test)]
impl Default for Data {
    fn default() -> Data {
        Data {
            users: UserMap::default(),
            temps: TempMap::default(),
            follow_requests: FollowRequestMap::default(),
            local_index_map: LocalUserIndexMap::default(),
            post_index_canister_id: Principal::anonymous(),
            local_post_index_canister_ids: HashSet::default(),
            super_admin: Principal::anonymous(),
            user_index_event_sync_queue: CanisterEventSyncQueue::default(),
            post_index_event_sync_queue: CanisterEventSyncQueue::default(),
            email_event_sync_queue: EmailEventSyncQueue::default(),
            local_user_index_canister_wasm_for_new_canisters: CanisterWasm::default(),
            local_user_index_canister_wasm_for_upgrades: CanisterWasm::default(),
            canisters_requiring_upgrade: CanistersRequiringUpgrade::default(),
            governance_principals: HashSet::default(),
            platform_moderators: HashSet::default(),
            platform_operators: HashSet::default(),
            total_cycles_spent_on_canisters: Cycles::default(),
        }
    }
}

#[derive(CandidType, Serialize, Debug)]
pub struct Metrics {
    pub now: TimestampMillis,
    pub memory_used: u64,
    pub cycles_balance: Cycles,
    pub total_users: usize,
    pub canister_upgrades_completed: u64,
    pub canister_upgrades_failed: Vec<FailedUpgradeCount>,
    pub canister_upgrades_pending: u64,
    pub canister_upgrades_in_progress: u64,
    pub governance_principals: Vec<Principal>,
    pub wasm_version: Version,
    pub local_user_index_wasm_version: Version,
    pub platform_moderators: u8,
    pub platform_operators: u8,
    pub user_index_events_queue_length: usize,
    pub local_user_indexes: Vec<(CanisterId, LocalUserIndex)>,
    pub total_cycles_spent_on_canisters: Cycles,
    pub canister_ids: CanisterIds,
}

#[derive(CandidType, Serialize, Debug)]
pub struct CanisterIds {
    pub post_index_canister_id: CanisterId,
    pub local_post_index_canister_ids: HashSet<CanisterId>,
}
