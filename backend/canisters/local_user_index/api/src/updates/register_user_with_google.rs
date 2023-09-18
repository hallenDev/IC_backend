use candid::{CandidType, Principal};
use serde::Deserialize;
use types::{NobleId, CanisterId};

#[derive(CandidType, Deserialize, Debug)]
pub struct Args {
    pub caller: Principal,
    pub noble_id: NobleId,
    pub canister_id: CanisterId,
    pub email: String,
    pub username: String,
}

#[derive(CandidType, Deserialize, Debug, PartialEq, Eq)]
pub enum Response {
    Success,
    UserLimitReached,
}
