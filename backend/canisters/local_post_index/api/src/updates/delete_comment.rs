use candid::CandidType;
use serde::{Deserialize, Serialize};
use types::{PostId, CommentId};

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Args {
    pub jwt: String,
    pub post_id: PostId,
    pub comment_id: CommentId,
}

#[derive(CandidType, Deserialize, Debug, PartialEq, Eq)]
pub enum Response {
    Success(SuccessResult),
    PermissionDenied,
    PostNotFound,
    CommentNotFound,
}

#[derive(CandidType, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct SuccessResult {
    pub post_id: PostId,
    pub comment_id: CommentId,
}