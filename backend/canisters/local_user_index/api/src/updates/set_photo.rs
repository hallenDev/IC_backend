use candid::CandidType;
use serde::{Deserialize, Serialize};
use types::AvatarId;

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Args {
    pub jwt: String,
    pub photo: Vec<u8>,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum Response {
    Success(AvatarId),
    PermissionDenied,
    UserNotFound,
    Error(ErrorResult),
    InternalError(String),
}

#[derive(CandidType, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct ErrorResult {
    pub photo: String,
}

impl ErrorResult {
    pub fn new() -> Self {
        Self {
            photo: String::new(),
        }
    }

    pub fn is_error(&self) -> bool {
        !self.photo.is_empty()
    }
}

impl Default for ErrorResult {
    fn default() -> Self {
        Self {
            photo: String::new(),
        }
    }
}