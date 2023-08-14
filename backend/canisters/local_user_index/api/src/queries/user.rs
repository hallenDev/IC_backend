use candid::CandidType;
use serde::Deserialize;
use types::{UserDetail, NobleId};

#[derive(CandidType, Deserialize, Debug)]
pub struct Args {
    pub jwt: String,
    pub noble_id: NobleId,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum Response {
    Success(UserDetail),
    PermissionDenied,
    UserNotFound,
    InternalError(String),
}
