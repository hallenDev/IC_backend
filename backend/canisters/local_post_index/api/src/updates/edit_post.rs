use std::collections::HashSet;

use candid::CandidType;
use serde::{Deserialize, Serialize};
use types::{NobleId, PostPrivacy, PostId};

#[derive(CandidType, Serialize, Deserialize, Debug, Default)]
pub struct Args {
    pub jwt: String,
    pub post_id: PostId,
    pub title: String,
    pub description: String,
    pub post_privacy: PostPrivacy,
    pub invited_users: HashSet<NobleId>,
}

#[derive(CandidType, Deserialize, Debug, PartialEq, Eq)]
pub enum Response {
    Success,
    Error(ErrorResult),
    PermissionDenied,
    PostNotFound,
}

#[derive(CandidType, Deserialize, Debug, PartialEq, Eq)]
pub struct ErrorResult{
    pub title: String,
    pub description: String,
}

impl ErrorResult {
    pub fn new() -> Self {
        ErrorResult {
            title: String::new(),
            description: String::new(),
        }
    }

    pub fn is_error(&self) -> bool {
        !(self.title.is_empty() &&
        self.description.is_empty())
    }
}

impl Default for ErrorResult {
    fn default() -> Self {
        ErrorResult {
            title: String::new(),
            description: String::new(),
        }
    }
}
