use candid::CandidType;
use serde::Deserialize;
use types::NobleId;

#[derive(CandidType, Deserialize, Debug)]
pub struct Args {
    pub jwt: String,
    pub noble_id: Option<NobleId>,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum Response {
    Success(Vec<NobleId>),
    PermissionDenied,
    UserNotFound,
}
