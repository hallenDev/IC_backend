use crate::{NobleId, TimestampMillis, PostId, Category, FileId};
use candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct PostDetail {
    pub post_id: PostId,
    pub noble_id: NobleId,
    pub title: String,
    pub description: String,
    pub category: Category,
    pub link_url: String,
    pub video_url: String,
    pub attached_file_id: FileId,
    pub liked_users_count: u32,
    pub comments_count: u32,

    pub date_created: TimestampMillis,
    pub date_updated: TimestampMillis,
    pub date_last_commented: TimestampMillis,
    pub like_state: bool,
    pub bookmark_state: bool,
}
