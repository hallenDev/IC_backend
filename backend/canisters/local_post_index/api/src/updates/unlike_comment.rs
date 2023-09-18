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
    Success,
    UserNotFound,
    PermissionDenied,
    PostNotFound,
    CommentNotFound,
}
