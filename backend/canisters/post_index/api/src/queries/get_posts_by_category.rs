use candid::CandidType;
use serde::{Serialize, Deserialize};
use types::{TimestampMillis, Category, PostSummary, NobleId, PostId};

#[derive(CandidType, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Sort {
    RecentActivity,
    NewestPost,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Args {
    pub jwt: String,
    pub from: u32,
    pub limit: u32,
    pub category: Option<Category>,
    pub sort: Sort,
    pub following_list: Vec<NobleId>,
    pub block_me_users: Vec<NobleId>,
    pub liked_posts: Vec<NobleId>,
    pub bookmarks: Vec<PostId>,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum Response {
    Success(SuccessResult),
    PermissionDenied,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct SuccessResult {
    pub total_posts_count: u32,
    pub posts: Vec<PostSummary>,
    pub timestamp: TimestampMillis,
}
