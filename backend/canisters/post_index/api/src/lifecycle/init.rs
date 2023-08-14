use candid::CandidType;
use serde::{Deserialize, Serialize};
use types::CanisterId;

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Args {
    pub user_index_canister_id: CanisterId,
    pub local_post_index_canister_ids: Vec<CanisterId>,
}