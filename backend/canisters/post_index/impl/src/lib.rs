use std::{cell::RefCell, collections::HashSet};

use canister_state_macros::canister_state;
use candid::{Principal, CandidType};
use local_post_index_canister::Event as LocalPostIndexEvent;
use model::{post_map::PostMap, local_post_index_map::{LocalPostIndexMap, LocalPostIndex}};
use serde::{Deserialize, Serialize};
use types::{CanisterId, TimestampMillis, Cycles, Timestamped, Version, CanisterWasm, NobleId};
use utils::{env::Environment, canister::{CanistersRequiringUpgrade, FailedUpgradeCount}, consts::{DEV_TEAM_PRINCIPAL, CYCLES_REQUIRED_FOR_UPGRADE}, canister_event_sync_queue::CanisterEventSyncQueue};
use user_index_canister::Event as UserIndexEvent;

mod jobs;
mod guards;
mod lifecycle;
mod model;
mod queries;
mod updates;
mod memory;

const LOCAL_POST_INDEX_CANISTER_INITIAL_CYCLES_BALANCE: Cycles = CYCLES_REQUIRED_FOR_UPGRADE + LOCAL_POST_INDEX_CANISTER_TOP_UP_AMOUNT; // 3.08T cycles
const LOCAL_POST_INDEX_CANISTER_TOP_UP_AMOUNT: Cycles = 3_000_000_000_000; // 3T cycles

pub const LOCAL_POST_LIMIT: usize = 200;
pub const POST_LIMIT: usize = 100_000;
pub const MAX_POST_DATA_SIZE: usize = 10_240;
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

    pub fn caller_is_governance_principal(&self) -> bool {
        let caller = ic_cdk::caller();
        DEV_TEAM_PRINCIPAL == caller ||
        self.data.super_admin == caller
    }

    pub fn caller_is_known_canister(&self) -> bool {
        let caller = ic_cdk::caller();
        self.data.local_index_map.contains_key(&caller) ||
        self.data.user_index_canister_id == caller ||
        self.data.local_user_index_canister_ids.contains(&caller)
    }

    pub fn push_event_to_all_local_post_index(&mut self, event: LocalPostIndexEvent) {
        self.data.local_index_map.iter().for_each(|(canisster_id, ..)| {
            self.data.post_index_event_sync_queue.push(*canisster_id, event.clone());
        });
        #[cfg(not(test))]
        jobs::sync_events_to_local_post_index_canisters::start_job_if_required(self);
    }

    pub fn push_event_to_user_index(&mut self, event: UserIndexEvent) {
        self.data
        .user_index_event_sync_queue
        .push(self.data.user_index_canister_id, event);
        #[cfg(not(test))]
        jobs::sync_events_to_user_index_canister::start_job_if_required(self);
    }

    pub fn metrics(&self) -> Metrics {
        let canister_upgrades_metrics = self.data.canisters_requiring_upgrade.metrics();
        Metrics {
            now: self.env.now(),
            memory_used: utils::memory::used(),
            cycles_balance: self.env.cycles_balance(),
            total_posts: self.data.posts.len(),
            wasm_version: WASM_VERSION.with(|v| **v.borrow()),
            canister_upgrades_completed: canister_upgrades_metrics.completed,
            canister_upgrades_failed: canister_upgrades_metrics.failed,
            canister_upgrades_pending: canister_upgrades_metrics.pending as u64,
            canister_upgrades_in_progress: canister_upgrades_metrics.in_progress as u64,
            local_post_index_wasm_version: self.data.local_post_index_canister_wasm_for_new_canisters.version,
            platform_moderators: self.data.platform_moderators.len() as u8,
            platform_operators: self.data.platform_operators.len() as u8,
            local_post_indexes: self.data.local_index_map.iter().map(|(c, i)| (*c, i.clone())).collect(),
            total_cycles_spent_on_canisters: self.data.total_cycles_spent_on_canisters,
            canister_ids: CanisterIds {
                user_index_canister_id: self.data.user_index_canister_id,
                local_user_index_canister_ids: self.data.local_user_index_canister_ids.clone()
            },
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Data {
    pub posts: PostMap,
    pub local_index_map: LocalPostIndexMap,
    pub user_index_canister_id: CanisterId,
    pub super_admin: Principal,
    #[serde(default)]
    pub local_post_index_canister_wasm_for_new_canisters: CanisterWasm,
    #[serde(default)]
    pub local_post_index_canister_wasm_for_upgrades: CanisterWasm,
    #[serde(default)]
    pub canisters_requiring_upgrade: CanistersRequiringUpgrade,
    #[serde(default)]
    pub platform_moderators: HashSet<NobleId>,
    #[serde(default)]
    pub platform_operators: HashSet<NobleId>,
    #[serde(default)]
    pub total_cycles_spent_on_canisters: Cycles,
    #[serde(default = "default_local_user_index_canister_ids")]
    pub local_user_index_canister_ids: HashSet<CanisterId>,
    #[serde(default)]
    pub post_index_event_sync_queue: CanisterEventSyncQueue<LocalPostIndexEvent>,
    #[serde(default)]
    pub user_index_event_sync_queue: CanisterEventSyncQueue<UserIndexEvent>,
}

fn default_local_user_index_canister_ids() -> HashSet<CanisterId>{
    let mut result = HashSet::new();
    result.insert(CanisterId::from_text("ok64i-eiaaa-aaaap-abjba-cai").unwrap());
    result
}

impl Data {
    pub fn new(
        user_index_canister_id: CanisterId,
        local_user_index_canister_ids: HashSet<CanisterId>,
        super_admin: Principal,
    ) -> Self {
        Data {
            posts: PostMap::default(),
            super_admin,
            local_index_map: LocalPostIndexMap::default(),
            user_index_canister_id,
            local_user_index_canister_ids,
            local_post_index_canister_wasm_for_new_canisters: CanisterWasm::default(),
            local_post_index_canister_wasm_for_upgrades: CanisterWasm::default(),
            canisters_requiring_upgrade: CanistersRequiringUpgrade::default(),
            platform_moderators: HashSet::default(),
            platform_operators: HashSet::default(),
            total_cycles_spent_on_canisters: Cycles::default(),
            post_index_event_sync_queue: CanisterEventSyncQueue::default(),
            user_index_event_sync_queue: CanisterEventSyncQueue::default(),
        }
    }
}

#[cfg(test)]
impl Default for Data {
    fn default() -> Data {
        Data {
            posts: PostMap::default(),
            user_index_canister_id: Principal::anonymous(),
            super_admin: Principal::anonymous(),
            local_index_map: LocalPostIndexMap::default(),
            local_user_index_canister_ids: HashSet::default(),
            local_post_index_canister_wasm_for_new_canisters: CanisterWasm::default(),
            local_post_index_canister_wasm_for_upgrades: CanisterWasm::default(),
            canisters_requiring_upgrade: CanistersRequiringUpgrade::default(),
            platform_moderators: HashSet::default(),
            platform_operators: HashSet::default(),
            total_cycles_spent_on_canisters: Cycles::default(),
            post_index_event_sync_queue: CanisterEventSyncQueue::default(),
            user_index_event_sync_queue: CanisterEventSyncQueue::default(),
        }
    }
}

#[derive(CandidType, Serialize, Debug)]
pub struct Metrics {
    pub now: TimestampMillis,
    pub memory_used: u64,
    pub cycles_balance: Cycles,
    pub total_posts: usize,
    pub canister_upgrades_completed: u64,
    pub canister_upgrades_failed: Vec<FailedUpgradeCount>,
    pub canister_upgrades_pending: u64,
    pub canister_upgrades_in_progress: u64,
    pub wasm_version: Version,
    pub local_post_index_wasm_version: Version,
    pub platform_moderators: u8,
    pub platform_operators: u8,
    pub local_post_indexes: Vec<(CanisterId, LocalPostIndex)>,
    pub total_cycles_spent_on_canisters: Cycles,
    pub canister_ids: CanisterIds,
}

#[derive(CandidType, Serialize, Debug)]
pub struct CanisterIds {
    pub user_index_canister_id: CanisterId,
    pub local_user_index_canister_ids: HashSet<CanisterId>,
}
