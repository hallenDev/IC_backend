use candid::CandidType;
use serde::{Deserialize, Serialize};
use types::AccountPrivacy;

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Args {
    pub jwt: String,
    pub account_privacy: AccountPrivacy,
}

#[derive(CandidType, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Response {
    Success,
    PermissionDenied,
    UserNotFound,
}
