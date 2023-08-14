use candid::CandidType;
use serde::{Serialize, Deserialize};
use types::NobleId;

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Args {
    pub noble_id: NobleId,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum Response {
    Success(Vec<NobleId>),
    UserNotFound,
}
