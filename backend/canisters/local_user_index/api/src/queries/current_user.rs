use candid::CandidType;
use serde::Deserialize;
use types::{PreferredPronouns, NobleId, AccountPrivacy, FollowingUser, TimestampMillis, Follower};

#[derive(CandidType, Deserialize, Debug)]
pub struct Args {
    pub jwt: String,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum Response {
    Success(SuccessResult),
    PermissionDenied,
    UserNotFound,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct SuccessResult {
    pub noble_id: NobleId,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub country: String,
    pub city: String,

    pub preferred_pronouns: Option<PreferredPronouns>,
    pub photo: Vec<u8>,
    pub email: String,
    pub search_by_email: bool,
    pub bio: String,

    pub account_privacy: AccountPrivacy,

    // social links
    pub linkedin_handle: String,
    pub twitter_handle: String,
    pub mastodon_handle: String,
    pub github_handle: String,
    pub facebook_handle: String,
    pub personal_website: String,

    pub followers: Vec<Follower>,
    pub following_list: Vec<FollowingUser>,
    pub block_users: Vec<NobleId>,

    pub date_created: TimestampMillis,
    pub date_updated: TimestampMillis,
}
