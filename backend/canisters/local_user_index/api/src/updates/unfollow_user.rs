use candid::CandidType;
use serde::{Deserialize, Serialize};
use types::NobleId;

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Args {
    pub jwt: String,
    pub noble_id: NobleId,
}

#[derive(CandidType, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Response {
    Success,
    PermissionDenied,
    UserNotFound,
    InternalError(String),
}
