use candid::CandidType;
use serde::{Deserialize, Serialize};
use types::{AcademicDegree, Country, Gender, PreferredPronouns};

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Args {
    pub jwt: String,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum Response {
    Success(SuccessResult),
    PermissionDenied,
    UserNotFound,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct SuccessResult {
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