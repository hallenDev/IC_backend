use candid::CandidType;
use serde::{Serialize, Deserialize};
use types::{NobleId, UserInfo};

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Args {
    pub jwt: String,
    pub noble_id: NobleId
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum Response {
    Success(UserInfo),
    UserNotFound,
    PermissionDenied,
}
