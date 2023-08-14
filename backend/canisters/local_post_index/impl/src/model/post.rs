use candid::CandidType;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use types::{
    TimestampMillis, PostId, Category, NobleId, PostPrivacy, FileId
};

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Post {
    pub post_id: PostId,
    pub owner: NobleId,
    pub title: String,
    pub description: String,
    pub category: Category,
    pub link_url: String,
    pub video_url: String,
    pub attached_file_id: FileId,

    pub liked_users: HashSet<NobleId>,
    pub contributed_users: HashSet<NobleId>,
    pub post_privacy: PostPrivacy,
    pub invited_users: HashSet<NobleId>,

    pub date_created: TimestampMillis,
    pub date_updated: TimestampMillis,
    pub date_last_commented: TimestampMillis,
}

impl Post {
    pub fn new(
        post_id: PostId,
        owner: NobleId,
        title: String,
        description: String,
        category: Category,
        link_url: String,
        video_url: String,
        attached_file_id: FileId,
        post_privacy: PostPrivacy,
        invited_users: HashSet<NobleId>,
        now: TimestampMillis,
    ) -> Self {
        Post {
            post_id,
            owner,
            title,
            description,
            category,

            link_url,
            video_url,
            attached_file_id,

            post_privacy,
            invited_users,
            liked_users: HashSet::default(),
            contributed_users: HashSet::default(),

            date_created: now,
            date_updated: now,
            date_last_commented: now,
        }
    }
}

#[cfg(test)]
impl Default for Post {
    fn default() -> Self {
        Post {
            post_id: 0,
            owner: 0,
            title: String::new(),
            description: String::new(),
            category: Category::GeneralDiscussion,

            link_url: String::new(),
            video_url: String::new(),
            attached_file_id: 0,

            post_privacy: PostPrivacy::AnyBody,
            invited_users: HashSet::default(),
            liked_users: HashSet::default(),
            contributed_users: HashSet::default(),

            date_created: 0,
            date_updated: 0,
            date_last_commented: 0,
        }
    }
}
