use crate::{NobleId, CanisterId, AvatarId};
use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

#[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
pub struct UserInfo {
    pub noble_id: NobleId,
    pub canister_id: CanisterId,
    pub avatar_id: AvatarId,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
}

impl UserInfo {
    pub fn new() -> Self {
        UserInfo {
            noble_id: 0,
            canister_id: Principal::anonymous(),
            avatar_id: 0,
            username: String::new(),
            first_name: String::new(),
            last_name: String::new()
        }
    }
}