use candid::CandidType;
use serde::{Deserialize, Serialize};
use types::TempId;

#[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
pub struct Args {
    pub username: String,
    pub email: String,
    pub password: String,
    pub password_confirm: String,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum Response {
    Success(TempId),
    UserLimitReached,
    CyclesBalanceTooLow,
    Error(ErrorResult),
    InternalError(String),
}

#[derive(CandidType, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct ErrorResult{
    pub username: String,
    pub password: String,
    pub password_confirm: String,
    pub email: String,
}

impl ErrorResult {
    pub fn new() -> Self {
        ErrorResult {
            username: String::new(),
            password: String::new(),
            password_confirm: String::new(),
            email: String::new(),
        }
    }

    pub fn is_error(&self) -> bool {
        !(self.username.is_empty() &&
        self.password.is_empty() &&
        self.password_confirm.is_empty() &&
        self.email.is_empty())
    }
}

impl Default for ErrorResult {
    fn default() -> Self {
        ErrorResult {
            username: String::new(),
            password: String::new(),
            password_confirm: String::new(),
            email: String::new(),
        }
    }
}
