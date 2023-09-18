use candid::CandidType;
use serde::{Deserialize, Serialize};
use types::SuccessLogin;

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Args {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum Response {
    Success(SuccessLogin),
    UsernameRequire(UsernameRequireResult),
    InternalError(String),
    UserLimitReached,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct UsernameRequireResult {
    pub jwt: String,
}
