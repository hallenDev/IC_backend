use candid::{CandidType,Principal};
use serde::{Deserialize, Serialize};
use types::{
    PreferredPronouns, TimestampMillis, NobleId, AccountPrivacy, FollowingUser, Follower, UserDetail
};

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct User {
    pub principal: Principal,
    pub noble_id: NobleId,
    pub email: String,
    pub username: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub country: String,
    pub city: String,

    pub preferred_pronouns: Option<PreferredPronouns>,
    pub photo: Vec<u8>,
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
    pub block_me_users: Vec<NobleId>,

    pub date_created: TimestampMillis,
    pub date_updated: TimestampMillis,
}

impl User {
    pub fn new(
        principal: Principal,
        noble_id: NobleId,
        email: String,
        username: String,
        password: String,
        now: TimestampMillis,
    ) -> Self {
        User {
            principal,
            noble_id,
            username,
            email,
            password,
            date_created: now,
            date_updated: now,
            first_name: String::new(),
            last_name: String::new(),
            preferred_pronouns: None,
            photo: vec![],
            country: String::new(),
            city: String::new(),
            linkedin_handle: String::new(),
            twitter_handle: String::new(),
            mastodon_handle: String::new(),
            github_handle: String::new(),
            facebook_handle: String::new(),
            personal_website: String::new(),
            bio: String::new(),
            search_by_email: false,
            account_privacy: AccountPrivacy::AnyBodyCanView,
            followers: vec![],
            following_list: vec![],
            block_users: vec![],
            block_me_users: vec![],
        }
    }

    pub fn add_follower(&mut self, noble_id: NobleId) -> bool {
        if self.is_follower(noble_id) {
            return false;
        }
        self.followers.push(Follower { noble_id, is_approved: false });
        true
    }

    pub fn remove_follower(&mut self, noble_id: NobleId) -> bool {
        let index = match self.followers.iter().position(|x| x.noble_id == noble_id) {
            Some(index) => index,
            None => return false,
        };
        self.followers.remove(index);
        true
    }

    pub fn add_following_user(&mut self, noble_id: NobleId) -> bool {
        if self.is_follower(noble_id) {
            return false;
        }
        self.following_list.push(FollowingUser { noble_id, is_muted: false });
        true
    }

    pub fn remove_following_user(&mut self, noble_id: NobleId) -> bool {
        let index = match self.following_list.iter().position(|x| x.noble_id == noble_id) {
            Some(index) => index,
            None => return false,
        };
        self.following_list.remove(index);
        true
    }

    pub fn is_follower(&self, noble_id: NobleId) -> bool {
        self.followers.iter().any(|item| item.noble_id == noble_id)
    }

    pub fn is_approved(&self, noble_id: NobleId) -> bool {
        self.followers.contains(&Follower { noble_id, is_approved: true })
    }

    pub fn is_following(&self, noble_id: NobleId) -> bool {
        self.following_list.iter().any(|item| item.noble_id == noble_id)
    }

    pub fn is_blocked(&self, noble_id: NobleId) -> bool {
        self.block_users.contains(&noble_id)
    }

    pub fn is_muted(&self, noble_id: NobleId) -> bool {
        self.following_list.contains(&FollowingUser { noble_id, is_muted: true })
    }

    pub fn add_block_user(&mut self, noble_id: NobleId) {
        self.block_users.push(noble_id)
    }

    pub fn add_block_me_user(&mut self, noble_id: NobleId) {
        self.block_me_users.push(noble_id)
    }

    pub fn remove_block_user(&mut self, noble_id: NobleId) {
        if let Some(index) = self.block_users.iter().position(|x| *x == noble_id) {
            self.block_users.remove(index);
        }
    }

    pub fn remove_block_me_user(&mut self, noble_id: NobleId) {
        if let Some(index) = self.block_me_users.iter().position(|x| *x == noble_id) {
            self.block_me_users.remove(index);
        }
    }

    pub fn mute_user(&mut self, noble_id: NobleId) {
        if let Some(item) = self.following_list.iter_mut().find(|x| x.noble_id == noble_id) {
            item.is_muted = true;
        }
    }

    pub fn unmute_user(&mut self, noble_id: NobleId) {
        if let Some(item) = self.following_list.iter_mut().find(|x| x.noble_id == noble_id) {
            item.is_muted = false;
        }
    }

    #[allow(dead_code)]
    pub fn set_approved(&mut self, noble_id: NobleId, approved: bool) {
        if let Some(item) = self.followers.iter_mut().find(|x| x.noble_id == noble_id) {
            item.is_approved = approved;
        }
    }

    pub fn to_detail(&self, noble_id: NobleId) -> UserDetail {
        if self.account_privacy == AccountPrivacy::AnyBodyCanView ||
            (self.account_privacy == AccountPrivacy::ApprovedFollowersCanView && self.is_approved(noble_id))
        {
            UserDetail {
                noble_id: self.noble_id,
                username: self.username.clone(),
                first_name: self.first_name.clone(),
                last_name: self.last_name.clone(),
                country: self.country.clone(),
                city: self.city.clone(),
            
                photo: self.photo.clone(),
                email: self.email.clone(),
                bio: self.bio.clone(),
            
                // social links
                linkedin_handle: self.linkedin_handle.clone(),
                twitter_handle: self.twitter_handle.clone(),
                mastodon_handle: self.mastodon_handle.clone(),
                github_handle: self.github_handle.clone(),
                facebook_handle: self.facebook_handle.clone(),
                personal_website: self.personal_website.clone(),

                followers: self.followers.clone(),
                following_list: self.following_list.clone(),
            
                date_created: self.date_created,
                date_updated: self.date_updated,
            }
        } else {
            UserDetail {
                noble_id: self.noble_id,
                username: self.username.clone(),
                first_name: self.first_name.clone(),
                last_name: self.last_name.clone(),
                photo: self.photo.clone(),
                date_created: self.date_created,
                date_updated: self.date_updated,
                ..Default::default()
            }
        }
    }

    pub fn verify_password(&self, password: &str) -> bool {
        match argon2::verify_encoded(&self.password, password.as_bytes()) {
            Ok(result) => result,
            Err(_) => false,
        }
    }
}

#[cfg(test)]
impl Default for User {
    fn default() -> Self {
        User {
            principal: Principal::anonymous(),
            noble_id: 1234567890,
            username: String::new(),
            password: String::new(),
            date_created: 0,
            date_updated: 0,

            first_name: String::new(),
            last_name: String::new(),
            preferred_pronouns: None,
            photo: vec![],

            email: String::new(),
            country: String::new(),
            city: String::new(),
            linkedin_handle: String::new(),
            twitter_handle: String::new(),
            mastodon_handle: String::new(),
            github_handle: String::new(),
            facebook_handle: String::new(),
            personal_website: String::new(),
            bio: String::new(),
            search_by_email: false,
            account_privacy: AccountPrivacy::AnyBodyCanView,
            followers: vec![],
            following_list: vec![],
            block_users: vec![],
            block_me_users: vec![],
        }
    }
}
