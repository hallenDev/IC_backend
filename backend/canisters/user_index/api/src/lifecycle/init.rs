use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};
use types::{CanisterId, Version};

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Args {
    pub local_user_index_canister_ids: Vec<CanisterId>,
    pub post_index_canister_id: CanisterId,
    pub local_post_index_canister_ids: Vec<CanisterId>,
    pub super_admin: Principal,
    pub wasm_version: Version,
}
