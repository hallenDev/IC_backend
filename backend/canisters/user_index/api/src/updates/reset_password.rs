use candid::CandidType;
use serde::{Deserialize, Serialize};
use types::TempId;

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Args {
    pub email: String,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum Response {
    Success(TempId),
    UserNotFound,
    EmailNotSet,
}
