use candid::CandidType;
use serde::{Deserialize, Serialize};
use types::AccountPrivacy;

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Args {
    pub jwt: String,
    pub username: String,
    pub email: String,
    pub search_by_email: bool,
    pub account_privacy: AccountPrivacy,
}

#[derive(CandidType, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Response {
    Success,
    PermissionDenied,
    UserNotFound,
    Error(ErrorResult),
    InternalError(String),
}

#[derive(CandidType, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct ErrorResult {
    pub username: String,
    pub email: String,
}

impl ErrorResult {
    pub fn new() -> Self {
        Self {
            username: String::new(),
            email: String::new(),
        }
    }

    pub fn is_error(&self) -> bool {
        !(self.username.is_empty() &&
        self.email.is_empty())
    }
}

impl Default for ErrorResult {
    fn default() -> Self {
        Self {
            username: String::new(),
            email: String::new(),
        }
    }
}