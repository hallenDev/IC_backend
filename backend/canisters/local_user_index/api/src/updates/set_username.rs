use candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Args {
    pub jwt: String,
    pub username: String,
}

#[derive(CandidType, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Response {
    Success,
    PermissionDenied,
    UsernameTaken,
    UserNotFound,
    UsernameInvalid,
    UsernameTooShort(u16),
    UsernameTooLong(u16),
    InternalError(String),
}
