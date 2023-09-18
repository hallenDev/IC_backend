use candid::CandidType;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use types::{CanisterId, CyclesTopUp, Version, NobleId};

use crate::LOCAL_USER_LIMIT;

#[derive(CandidType, Serialize, Deserialize, Default)]
pub struct LocalUserIndexMap {
    index_map: HashMap<CanisterId, LocalUserIndex>,
    user_to_index: HashMap<NobleId, CanisterId>,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Default)]
pub struct LocalUserIndex {
    user_count: u32,
    full: bool,
    cycle_top_ups: Vec<CyclesTopUp>,
    wasm_version: Version,
}

impl LocalUserIndexMap {
    pub fn add_index(&mut self, index_id: CanisterId, wasm_version: Version) -> bool {
        let exists = self.index_map.contains_key(&index_id);
        if !exists {
            self.index_map.insert(
                index_id,
                LocalUserIndex {
                    user_count: 0,
                    full: false,
                    cycle_top_ups: Vec::default(),
                    wasm_version,
                },
            );
        }
        !exists
    }

    pub fn add_user(&mut self, index_id: CanisterId, noble_id: NobleId) -> bool {
        if let Some(index) = self.index_map.get_mut(&index_id) {
            if self.user_to_index.insert(noble_id, index_id).is_none() {
                index.user_count += 1;
                if index.user_count >= LOCAL_USER_LIMIT as u32 {
                    index.mark_full();
                }
                return true;
            }
        }

        false
    }

    pub fn remove_user(&mut self, index_id: CanisterId, noble_id: NobleId) -> bool {
        if let Some(index) = self.index_map.get_mut(&index_id) {
            if self.user_to_index.remove(&noble_id).is_some() {
                index.user_count -= 1;
                if index.full == true && index.user_count < LOCAL_USER_LIMIT as u32 {
                    index.mark_unfull();
                }
                return true;
            }
        }

        false
    }

    pub fn index_for_new_user(&self) -> Option<CanisterId> {
        self.index_map
            .iter()
            .filter(|(_, v)| !v.full)
            .min_by_key(|(_, v)| v.user_count)
            .map(|(k, _)| *k)
    }

    pub fn contains_key(&self, index_id: &CanisterId) -> bool {
        self.index_map.contains_key(index_id)
    }

    #[allow(dead_code)]
    pub fn canisters(&self) -> impl Iterator<Item = &CanisterId> {
        self.index_map.keys()
    }

    pub fn get_index_canister(&self, noble_id: &NobleId) -> Option<CanisterId> {
        self.user_to_index.get(noble_id).copied()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&CanisterId, &LocalUserIndex)> {
        self.index_map.iter()
    }

    pub fn get(&self, index_id: &CanisterId) -> Option<&LocalUserIndex> {
        self.index_map.get(index_id)
    }

    pub fn get_mut(&mut self, index_id: &CanisterId) -> Option<&mut LocalUserIndex> {
        self.index_map.get_mut(index_id)
    }
}

impl LocalUserIndex {
    pub fn mark_cycles_top_up(&mut self, top_up: CyclesTopUp) {
        self.cycle_top_ups.push(top_up);
    }

    pub fn set_wasm_version(&mut self, wasm_version: Version) {
        self.wasm_version = wasm_version;
    }

    pub fn mark_full(&mut self) {
        self.full = true;
    }

    pub fn mark_unfull(&mut self) {
        self.full = false;
    }

    pub fn wasm_version(&self) -> Version {
        self.wasm_version
    }
}
