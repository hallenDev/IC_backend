use candid::CandidType;
use serde::{Deserialize, Serialize};
use types::SuccessLogin;

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Args {
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum Response {
    Success(SuccessLogin),
    UsernameRequire(UsernameRequireResult),
    InternalError(String),
    InvalidInternetIdentity,
    UserLimitReached,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct UsernameRequireResult {
    pub jwt: String,
}
