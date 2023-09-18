use candid::CandidType;
use serde::Deserialize;
use types::{PostId, CommentId, NobleId};

#[derive(CandidType, Deserialize, Debug)]
pub struct Args {
    pub jwt: String,
    pub post_id: PostId,
    pub comment_id: CommentId,
    pub following_list: Vec<NobleId>,
    pub block_me_users: Vec<NobleId>,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum Response {
    Success(Vec<NobleId>),
    PermissionDenied,
    PostNotFound,
    CommentNotFound,
}
