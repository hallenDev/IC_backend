use candid::CandidType;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use types::{CanisterId, CyclesTopUp, Version, PostId};

#[derive(CandidType, Serialize, Deserialize, Default)]
pub struct LocalPostIndexMap {
    index_map: HashMap<CanisterId, LocalPostIndex>,
    post_to_index: HashMap<PostId, CanisterId>,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Default)]
pub struct LocalPostIndex {
    post_count: u32,
    full: bool,
    cycle_top_ups: Vec<CyclesTopUp>,
    wasm_version: Version,
}

impl LocalPostIndexMap {
    pub fn add_index(&mut self, index_id: CanisterId, wasm_version: Version) -> bool {
        let exists = self.index_map.contains_key(&index_id);
        if !exists {
            self.index_map.insert(
                index_id,
                LocalPostIndex {
                    post_count: 0,
                    full: false,
                    cycle_top_ups: Vec::default(),
                    wasm_version,
                },
            );
        }
        !exists
    }

    pub fn add_post(&mut self, index_id: CanisterId, post_id: PostId) -> bool {
        if let Some(index) = self.index_map.get_mut(&index_id) {
            if self.post_to_index.insert(post_id, index_id).is_none() {
                index.post_count += 1;
                return true;
            }
        }

        false
    }

    pub fn index_for_new_post(&self) -> Option<CanisterId> {
        self.index_map
            .iter()
            .filter(|(_, v)| !v.full)
            .min_by_key(|(_, v)| v.post_count)
            .map(|(k, _)| *k)
    }

    #[allow(dead_code)]
    pub fn contains_key(&self, index_id: &CanisterId) -> bool {
        self.index_map.contains_key(index_id)
    }

    #[allow(dead_code)]
    pub fn get(&self, index_id: &CanisterId) -> Option<&LocalPostIndex> {
        self.index_map.get(index_id)
    }

    #[allow(dead_code)]
    pub fn get_mut(&mut self, index_id: &CanisterId) -> Option<&mut LocalPostIndex> {
        self.index_map.get_mut(index_id)
    }

    #[allow(dead_code)]
    pub fn canisters(&self) -> impl Iterator<Item = &CanisterId> {
        self.index_map.keys()
    }

    #[allow(dead_code)]
    pub fn get_index_canister(&self, post_id: &PostId) -> Option<CanisterId> {
        self.post_to_index.get(post_id).copied()
    }

    #[allow(dead_code)]
    pub fn iter(&self) -> impl Iterator<Item = (&CanisterId, &LocalPostIndex)> {
        self.index_map.iter()
    }
}

impl LocalPostIndex {
    #[allow(dead_code)]
    pub fn mark_cycles_top_up(&mut self, top_up: CyclesTopUp) {
        self.cycle_top_ups.push(top_up);
    }

    #[allow(dead_code)]
    pub fn set_wasm_version(&mut self, wasm_version: Version) {
        self.wasm_version = wasm_version;
    }

    #[allow(dead_code)]
    pub fn mark_full(&mut self) {
        self.full = true;
    }

    #[allow(dead_code)]
    pub fn wasm_version(&self) -> Version {
        self.wasm_version
    }
}
