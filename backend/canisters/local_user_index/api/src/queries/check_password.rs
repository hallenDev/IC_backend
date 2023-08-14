use candid::CandidType;
use serde::{Deserialize, Serialize};
use types::NobleId;

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Args {
    pub noble_id: NobleId,
    pub password: String,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum Response {
    Success,
    UserNotFound,
    Error,
}
