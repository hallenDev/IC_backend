use candid::CandidType;
use serde::Deserialize;
use types::{CommentDetail, PostId, CommentId, NobleId};

#[derive(CandidType, Deserialize, Debug)]
pub struct Args {
    pub jwt: String,
    pub post_id: PostId,
    pub comment_id: CommentId,
    pub from: u32,
    pub limit: u32,
    pub following_list: Vec<NobleId>,
    pub block_me_users: Vec<NobleId>,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum Response {
    Success(ScucessResult),
    PermissionDenied,
    PostNotFound,
    CommentNotFound,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct ScucessResult {
    pub comments: Vec<CommentDetail>,
    pub more_exist: bool,
}