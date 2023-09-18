use candid::CandidType;
use serde::Deserialize;
use types::{NobleId, PostId};

#[derive(CandidType, Deserialize, Debug)]
pub struct Args {
    pub jwt: String,
    pub noble_id: Option<NobleId>,
    pub mask: u32,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum Response {
    Success(SuccessResult),
    PermissionDenied,
    UserNotFound,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct SuccessResult {
    pub block_me_users: Vec<NobleId>,
    pub following_list: Vec<NobleId>,
    pub bookmarks: Vec<PostId>,
    pub liked_posts: Vec<PostId>,
}
