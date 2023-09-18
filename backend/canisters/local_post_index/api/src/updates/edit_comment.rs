use candid::CandidType;
use serde::{Deserialize, Serialize};
use types::{PostId, CommentId};

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Args {
    pub jwt: String,
    pub post_id: PostId,
    pub comment_id: CommentId,
    pub description: String,
}

#[derive(CandidType, Deserialize, Debug, PartialEq, Eq)]
pub enum Response {
    Success,
    Error(ErrorResult),
    PermissionDenied,
    PostNotFound,
    CommentNotFound,
}

#[derive(CandidType, Deserialize, Debug, PartialEq, Eq)]
pub struct ErrorResult{
    pub description: String,
}

impl ErrorResult {
    pub fn new() -> Self {
        ErrorResult {
            description: String::new(),
        }
    }

    pub fn is_error(&self) -> bool {
        !self.description.is_empty()
    }
}