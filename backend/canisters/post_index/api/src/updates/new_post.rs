use std::collections::HashSet;

use candid::CandidType;
use serde::{Deserialize, Serialize};
use types::{NobleId, Category, PostPrivacy, FileId, CanisterId, PostId};

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Args {
    pub jwt: String,
    pub title: String,
    pub description: String,
    pub category: Category,
    pub link_url: String,
    pub video_url: String,
    pub attached_file_id: FileId,
    pub post_privacy: PostPrivacy,
    pub invited_users: HashSet<NobleId>,
}

#[derive(CandidType, Deserialize, Debug, PartialEq, Eq)]
pub enum Response {
    Success(CanisterId, PostId),
    PermissionDenied,
    PostLimitReached,
    InternalError(String),
    Error(ErrorResult),
}

#[derive(CandidType, Deserialize, Debug, PartialEq, Eq)]
pub struct ErrorResult{
    pub title: String,
    pub description: String,
}

impl ErrorResult {
    pub fn new() -> Self {
        ErrorResult {
            title: String::new(),
            description: String::new(),
        }
    }

    pub fn is_error(&self) -> bool {
        !(self.title.is_empty() &&
        self.description.is_empty())
    }
}

impl Default for ErrorResult {
    fn default() -> Self {
        ErrorResult {
            title: String::new(),
            description: String::new(),
        }
    }
}

impl Default for Args {
    fn default() -> Self {
        Args {
            jwt: "".to_string(),
            title: "title".to_string(),
            description: "description".to_string(),
            category: Category::IntroduceYourself,
            link_url: "".to_string(),
            video_url: "".to_string(),
            attached_file_id: 0,
            post_privacy: PostPrivacy::Everyone,
            invited_users: HashSet::new(),
        }
    }
}