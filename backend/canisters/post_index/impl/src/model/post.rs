use candid::CandidType;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use types::{
    TimestampMillis, PostId, Category, NobleId, PostPrivacy, FileId, PostSummary
};

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Post {
    pub post_id: PostId,
    pub owner: NobleId,
    pub title: String,
    // up to 100 character.
    pub description: String,
    pub category: Category,
    pub link_url: String,
    pub video_url: String,
    pub attached_file_id: FileId,

    // up to five.
    pub liked_user_count: u32,
    pub contributed_users: HashSet<NobleId>,
    pub post_privacy: PostPrivacy,
    pub invited_users: HashSet<NobleId>,

    pub date_created: TimestampMillis,
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
            liked_user_count: 0,
            contributed_users: HashSet::default(),

            date_created: now,
            date_last_commented: now,
        }
    }

    pub fn to_summary(&self) -> PostSummary {
        PostSummary {
            post_id: self.post_id,
            owner: self.owner,
            title: self.title.clone(),
            description: self.description.clone(),
            category: self.category,
            link_url: self.link_url.clone(),
            video_url: self.video_url.clone(),
            attached_file_id: self.attached_file_id,
            liked_user_count: self.liked_user_count,
            contributed_users: self.contributed_users.clone(),
            date_created: self.date_created,
            date_last_commented: self.date_last_commented
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
            invited_users: HashSet::new(),
            liked_user_count: 0,
            contributed_users: HashSet::default(),

            date_created: 0,
            date_last_commented: 0,
        }
    }
}
