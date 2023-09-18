use candid::CandidType;
use serde::{Deserialize, Serialize};
use types::{AcademicDegree, Country, Gender, PreferredPronouns, AvatarId};

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Args {
    pub jwt: String,
    pub first_name: String,
    pub last_name: String,
    pub gender: Option<Gender>,
    pub degree: Option<AcademicDegree>,
    pub bio: String,
    pub personal_website: String,
    pub country: Option<Country>,
    pub city: String,
    pub preferred_pronouns: Option<PreferredPronouns>,
    pub linkedin_handle: String,
    pub twitter_handle: String,
    pub mastodon_handle: String,
    pub github_handle: String,
    pub facebook_handle: String,
}

#[derive(CandidType, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Response {
    Success(AvatarId),
    PermissionDenied,
    UserNotFound,
    Error(ErrorResult),
}

#[derive(CandidType, Serialize, Deserialize, Debug, PartialEq, Eq, Default)]
pub struct ErrorResult {
    pub first_name: String,
    pub last_name: String,
    pub bio: String,
    pub personal_website: String,
    pub city: String,
    pub linkedin_handle: String,
    pub twitter_handle: String,
    pub mastodon_handle: String,
    pub github_handle: String,
    pub facebook_handle: String,
    pub degree: String,
    pub country: String,
    pub preferred_pronouns: String,
}

impl ErrorResult {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_error(&self) -> bool {
        !(self.first_name.is_empty() &&
        self.last_name.is_empty() &&
        self.bio.is_empty() &&
        self.personal_website.is_empty() &&
        self.city.is_empty() &&
        self.linkedin_handle.is_empty() &&
        self.twitter_handle.is_empty() &&
        self.mastodon_handle.is_empty() &&
        self.github_handle.is_empty() &&
        self.facebook_handle.is_empty() &&
        self.degree.is_empty() &&
        self.country.is_empty() &&
        self.preferred_pronouns.is_empty())
    }
}
