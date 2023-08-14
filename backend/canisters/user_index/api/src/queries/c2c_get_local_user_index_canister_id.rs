use candid::CandidType;
use serde::{Serialize, Deserialize};
use types::{NobleId, CanisterId};

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Args {
    pub noble_id: NobleId,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum Response {
    Success(CanisterId),
    UserNotFound,
}
