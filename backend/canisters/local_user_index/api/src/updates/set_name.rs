use candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Args {
    pub jwt: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(CandidType, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Response {
    Success,
    PermissionDenied,
    InputIsInvalid,
    UserNotFound,
}
