use crate::{NobleId, TimestampMillis, Follower, FollowingUser};
use candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(CandidType, Serialize, Deserialize, Debug, Default)]
pub struct UserDetail {
    pub noble_id: NobleId,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub country: String,
    pub city: String,

    pub photo: Vec<u8>,
    pub email: String,
    pub bio: String,

    // social links
    pub linkedin_handle: String,
    pub twitter_handle: String,
    pub mastodon_handle: String,
    pub github_handle: String,
    pub facebook_handle: String,
    pub personal_website: String,

    pub followers: Vec<Follower>,
    pub following_list: Vec<FollowingUser>,

    pub date_created: TimestampMillis,
    pub date_updated: TimestampMillis,
}
