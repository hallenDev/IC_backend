use crate::{NobleId, TimestampMillis, Country, AcademicDegree, Gender, PreferredPronouns, CanisterId, AvatarId};
use candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct UserDetail {
    pub noble_id: NobleId,
    pub local_user_canister_id: CanisterId,
    pub avatar_id: AvatarId,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub gender: Option<Gender>,
    pub preferred_pronouns: Option<PreferredPronouns>,
    pub country: Option<Country>,
    pub city: String,
    pub degree: Option<AcademicDegree>,

    pub email: String,
    pub bio: String,

    // social links
    pub linkedin_handle: String,
    pub twitter_handle: String,
    pub mastodon_handle: String,
    pub github_handle: String,
    pub facebook_handle: String,
    pub personal_website: String,

    pub followers_count: u32,
    pub following_count: u32,
    pub follow_state: bool,

    pub date_created: TimestampMillis,
    pub date_updated: TimestampMillis,
}

impl Default for UserDetail {
    fn default() -> Self {
        UserDetail {
            noble_id: NobleId::default(),
            local_user_canister_id: CanisterId::anonymous(),
            avatar_id: AvatarId::default(),
            username: String::default(),
            first_name: String::default(),
            last_name: String::default(),
            gender: None,
            preferred_pronouns: None,
            country: None,
            city: String::default(),
            degree: None,
        
            email: String::default(),
            bio: String::default(),
        
            // social links
            linkedin_handle: String::default(),
            twitter_handle: String::default(),
            mastodon_handle: String::default(),
            github_handle: String::default(),
            facebook_handle: String::default(),
            personal_website: String::default(),
        
            followers_count: u32::default(),
            following_count: u32::default(),
            follow_state: false,
        
            date_created: TimestampMillis::default(),
            date_updated: TimestampMillis::default(),
        }
    }
}