use candid::CandidType;
use serde::{Deserialize, Serialize};
use types::SuccessLogin;

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Args {
    pub jwt: String,
    pub username: String,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum Response {
    Success(SuccessLogin),
    PermissionDenied,
    InvalidInternetIdentity,
    UserNotFound,
    Error(ErrorResult),
    InternalError(String),
}

#[derive(CandidType, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct ErrorResult{
    pub username: String,
}

impl ErrorResult {
    pub fn new() -> Self {
        ErrorResult {
            username: String::new(),
        }
    }

    pub fn is_error(&self) -> bool {
        !self.username.is_empty()
    }
}
