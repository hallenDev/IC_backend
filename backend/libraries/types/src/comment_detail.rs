use crate::{NobleId, TimestampMillis, CommentId};
use candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct CommentDetail {
    pub noble_id: NobleId,
    pub comment_id: CommentId,
    pub parent_comment_id: CommentId,
    pub description: String,
    pub liked_users_count: u32,
    pub comments_count: u32,
    pub date_created: TimestampMillis,
    pub like_state: bool,
    pub loading_like: bool,
    pub loading_delete: bool,
}