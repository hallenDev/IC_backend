use std::collections::HashSet;

use candid::CandidType;
use serde::{Deserialize, Serialize};
use types::{NobleId, Category, PostPrivacy, FileId, PostId, TimestampMillis};

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Args {
    pub post_id: PostId,
    pub owner: NobleId,
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

#[derive(CandidType, Deserialize, Debug, PartialEq, Eq)]
pub struct ErrorResult{
    pub title: String,
    pub description: String,
    pub category: String,
}

impl ErrorResult {
    pub fn new() -> Self {
        ErrorResult {
            title: String::new(),
            description: String::new(),
            category: String::new(),
        }
    }

    pub fn is_error(&self) -> bool {
        !(self.title.is_empty() &&
        self.description.is_empty() &&
        self.category.is_empty())
    }
}

impl Default for ErrorResult {
    fn default() -> Self {
        ErrorResult {
            title: String::new(),
            description: String::new(),
            category: String::new(),
        }
    }
}

impl Default for Args {
    fn default() -> Self {
        Args {
            post_id: 0,
            owner: 0,
            title: "title".to_string(),
            description: "description".to_string(),
            category: Category::IntroduceYourself,
            link_url: "".to_string(),
            video_url: "".to_string(),
            attached_file_id: 0,
            post_privacy: PostPrivacy::AnyBody,
            invited_users: HashSet::new(),
            date_created: 0,
        }
    }
}