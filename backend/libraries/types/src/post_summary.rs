use crate::{PostId, TimestampMillis, NobleId, FileId, Category};
use candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
pub struct PostSummary {
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
    pub date_last_commented: TimestampMillis,
    pub like_state: bool,
    pub bookmark_state: bool,
    pub loading_like: bool,
    pub loading_bookmark: bool,
    pub loading_delete: bool,
}
