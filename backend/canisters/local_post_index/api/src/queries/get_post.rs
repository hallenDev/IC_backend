use candid::CandidType;
use serde::Deserialize;
use types::{PostDetail, PostId, CommentDetail, NobleId};

#[derive(CandidType, Deserialize, Debug)]
pub struct Args {
    pub jwt: String,
    pub post_id: PostId,
    pub limit: u32,
    pub following_list: Vec<NobleId>,
    pub block_me_users: Vec<NobleId>,
    pub bookmarks: Vec<PostId>,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum Response {
    Success(SuccessResult),
    PermissionDenied,
    PostNotFound,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct SuccessResult {
    pub post: PostDetail,
    pub comments: Vec<CommentDetail>,
    pub more_exist: bool,
}