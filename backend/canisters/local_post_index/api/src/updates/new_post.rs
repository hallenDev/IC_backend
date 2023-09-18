use std::collections::HashSet;

use candid::CandidType;
use serde::{Deserialize, Serialize};
use types::{NobleId, Category, PostPrivacy, FileId, PostId, TimestampMillis};

#[derive(CandidType, Serialize, Deserialize, Debug, Default)]
pub struct Args {
    pub post_id: PostId,
    pub noble_id: NobleId,
    pub title: String,
    pub description: String,
    pub category: Category,
    pub link_url: String,
    pub video_url: String,
    pub attached_file_id: FileId,
    pub post_privacy: PostPrivacy,
    pub invited_users: HashSet<NobleId>,
    pub date_created: TimestampMillis,
}

#[derive(CandidType, Deserialize, Debug, PartialEq, Eq)]
pub enum Response {
    Success,
    PostLimitReached,
}
