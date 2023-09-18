use candid::CandidType;
use serde::{Deserialize, Serialize};
use types::AccountPrivacy;

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Args {
    pub jwt: String,
}

#[derive(CandidType, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Response {
    Success(SuccessResult),
    PermissionDenied,
    UserNotFound,
}

#[derive(CandidType, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct SuccessResult {
    pub username: String,
    pub email: String,
    pub search_by_email: bool,
    pub account_privacy: AccountPrivacy,
}