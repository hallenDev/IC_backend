use std::collections::HashSet;

use crate::{PostId, TimestampMillis, NobleId, FileId, Category};
use candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct PostSummary {
    pub post_id: PostId,
    pub owner: NobleId,
    pub title: String,
    pub description: String,
    pub category: Category,
    pub link_url: String,
    pub video_url: String,
    pub attached_file_id: FileId,
    pub liked_user_count: u32,
    pub contributed_users: HashSet<NobleId>,
    pub date_created: TimestampMillis,
    pub date_last_commented: TimestampMillis,
}
