use candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Args {
    pub jwt: String,
    pub password: String,
    pub new_password: String,
    pub password_confirm: String,
}

#[derive(CandidType, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Response {
    Success,
    PermissionDenied,
    UserNotFound,
    Error(ErrorResult),
}

#[derive(CandidType, Serialize, Deserialize, Debug, PartialEq, Eq, Default)]
pub struct ErrorResult {
    pub password: String,
    pub new_password: String,
    pub password_confirm: String,
}

impl ErrorResult {
    pub fn new() -> Self {
        Self {
            password: String::new(),
            new_password: String::new(),
            password_confirm: String::new(),
        }
    }

    pub fn is_error(&self) -> bool {
        !(self.password.is_empty() &&
        self.new_password.is_empty() &&
        self.password_confirm.is_empty())
    }
}
