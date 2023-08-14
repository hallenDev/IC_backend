use candid::{CandidType, Principal};
use serde::Deserialize;
use types::NobleId;

#[derive(CandidType, Deserialize, Debug)]
pub struct Args {
    pub caller: Principal,
    pub noble_id: NobleId,
    pub email: String,
    pub username: String,
    pub password_hash: String,
}

#[derive(CandidType, Deserialize, Debug, PartialEq, Eq)]
pub enum Response {
    Success,
    AlreadyRegistered,
    UserLimitReached,
}
