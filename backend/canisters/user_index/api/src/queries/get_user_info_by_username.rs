use candid::CandidType;
use serde::{Serialize, Deserialize};
use types::UserInfo;

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Args {
    pub jwt: String,
    pub username: String,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum Response {
    Success(UserInfo),
    UserNotFound,
    PermissionDenied,
}
