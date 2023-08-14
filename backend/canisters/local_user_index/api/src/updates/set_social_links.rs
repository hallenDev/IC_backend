use candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Args {
    pub jwt: String,
    pub linkedin_handle: String,
    pub twitter_handle: String,
    pub mastodon_handle: String,
    pub github_handle: String,
    pub facebook_handle: String,
    pub personal_website: String,
}

#[derive(CandidType, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Response {
    Success,
    PermissionDenied,
    UserNotFound,
    Error(ErrorResult),
}

#[derive(CandidType, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct ErrorResult{
    pub linkedin_handle: String,
    pub twitter_handle: String,
    pub mastodon_handle: String,
    pub github_handle: String,
    pub facebook_handle: String,
    pub personal_website: String,
}

impl ErrorResult {
    pub fn new() -> Self {
        ErrorResult {
            linkedin_handle: String::new(),
            twitter_handle: String::new(),
            mastodon_handle: String::new(),
            github_handle: String::new(),
            facebook_handle: String::new(),
            personal_website: String::new(),
        }
    }

    pub fn is_error(&self) -> bool {
        !(self.linkedin_handle.is_empty() &&
        self.twitter_handle.is_empty() &&
        self.mastodon_handle.is_empty() &&
        self.github_handle.is_empty() &&
        self.facebook_handle.is_empty() &&
        self.personal_website.is_empty())
    }
}

impl Default for ErrorResult {
    fn default() -> Self {
        ErrorResult {
            linkedin_handle: String::new(),
            twitter_handle: String::new(),
            mastodon_handle: String::new(),
            github_handle: String::new(),
            facebook_handle: String::new(),
            personal_website: String::new(),
        }
    }
}

impl Default for Args {
    fn default() -> Self {
        Args {
            jwt: String::new(),
            linkedin_handle: String::new(),
            twitter_handle: String::new(),
            mastodon_handle: String::new(),
            github_handle: String::new(),
            facebook_handle: String::new(),
            personal_website: String::new(),
        }
    }
}