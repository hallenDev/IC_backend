use candid::{CandidType,Principal};
use serde::{Deserialize, Serialize};
use types::{
    PreferredPronouns, TimestampMillis, NobleId, AccountPrivacy, FollowingUser, Follower, UserDetail, Gender, AcademicDegree, Country, CanisterId, PostId, CommentId, AvatarId
};

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct User {
    pub principal: Principal,
    pub noble_id: NobleId,
    pub canister_id: CanisterId,
    pub email: String,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub gender: Option<Gender>,
    pub degree: Option<AcademicDegree>,
    pub country: Option<Country>,
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

    #[serde(default)]
    pub bookmarks: Vec<PostId>,
    #[serde(default)]
    pub liked_posts: Vec<(PostId, CommentId)>,
    #[serde(default)]
    pub avatar_id: AvatarId,

    pub date_created: TimestampMillis,
    pub date_updated: TimestampMillis,
}

impl User {
    pub fn new(
        principal: Principal,
        noble_id: NobleId,
        canister_id: CanisterId,
        email: String,
        username: String,
        now: TimestampMillis,
    ) -> Self {
        User {
            principal,
            noble_id,
            canister_id,
            username,
            email,
            date_created: now,
            date_updated: now,
            first_name: String::new(),
            last_name: String::new(),
            gender: None,
            preferred_pronouns: None,
            photo: vec![],
            country: None,
            degree: None,
            city: String::new(),
            linkedin_handle: String::new(),
            twitter_handle: String::new(),
            mastodon_handle: String::new(),
            github_handle: String::new(),
            facebook_handle: String::new(),
            personal_website: String::new(),
            bio: String::new(),
            search_by_email: false,
            account_privacy: AccountPrivacy::Everyone,
            followers: vec![],
            following_list: vec![],
            block_users: vec![],
            block_me_users: vec![],
            bookmarks: vec![],
            liked_posts: vec![],
            avatar_id: 0,
        }
    }

    pub fn is_follower(&self, noble_id: NobleId) -> bool {
        self.followers.iter().any(|item| item.noble_id == noble_id)
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

    pub fn is_following(&self, noble_id: NobleId) -> bool {
        self.following_list.iter().any(|item| item.noble_id == noble_id)
    }
    pub fn add_following_user(&mut self, noble_id: NobleId) -> bool {
        if self.is_following(noble_id) {
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

    pub fn is_blocked(&self, noble_id: NobleId) -> bool {
        self.block_users.contains(&noble_id)
    }
    pub fn add_block_user(&mut self, noble_id: NobleId) {
        self.block_users.push(noble_id)
    }
    pub fn remove_block_user(&mut self, noble_id: NobleId) {
        if let Some(index) = self.block_users.iter().position(|x| *x == noble_id) {
            self.block_users.remove(index);
        }
    }

    pub fn add_block_me_user(&mut self, noble_id: NobleId) {
        self.block_me_users.push(noble_id)
    }
    pub fn remove_block_me_user(&mut self, noble_id: NobleId) {
        if let Some(index) = self.block_me_users.iter().position(|x| *x == noble_id) {
            self.block_me_users.remove(index);
        }
    }

    pub fn like_post(&mut self, post_id: PostId, comment_id: CommentId) {
        self.liked_posts.push((post_id, comment_id));
    }
    pub fn unlike_post(&mut self, post_id: PostId, comment_id: CommentId) {
        if let Some(index) = self.liked_posts.iter().position(|(x, y)| *x == post_id && *y == comment_id) {
            self.liked_posts.remove(index);
        }
    }

    pub fn is_muted(&self, noble_id: NobleId) -> bool {
        self.following_list.contains(&FollowingUser { noble_id, is_muted: true })
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

    pub fn is_bookmarked(&self, post_id: PostId) -> bool {
        self.bookmarks.contains(&post_id)
    }
    pub fn add_bookmark(&mut self, post_id: PostId) {
        self.bookmarks.push(post_id);
    }
    pub fn remove_bookmark(&mut self, post_id: PostId) -> bool {
        if let Some(index) = self.bookmarks.iter().position(|x| *x == post_id) {
            self.bookmarks.remove(index);
            true
        } else {
            false
        }
    }

    pub fn is_approved(&self, noble_id: NobleId) -> bool {
        self.followers.contains(&Follower { noble_id, is_approved: true })
    }

    #[allow(dead_code)]
    pub fn set_approved(&mut self, noble_id: NobleId, approved: bool) {
        if let Some(item) = self.followers.iter_mut().find(|x| x.noble_id == noble_id) {
            item.is_approved = approved;
        }
    }

    pub fn to_detail(&self, noble_id: NobleId) -> UserDetail {
        if self.account_privacy == AccountPrivacy::Everyone ||
            (self.account_privacy == AccountPrivacy::ApprovedFollowers && self.is_approved(noble_id))
        {
            UserDetail {
                noble_id: self.noble_id,
                local_user_canister_id: self.canister_id,
                avatar_id: self.avatar_id,
                username: self.username.clone(),
                first_name: self.first_name.clone(),
                last_name: self.last_name.clone(),
                gender: self.gender,
                preferred_pronouns: self.preferred_pronouns,
                degree: self.degree,
                country: self.country.clone(),
                city: self.city.clone(),

                email: self.email.clone(),
                bio: self.bio.clone(),
            
                // social links
                linkedin_handle: self.linkedin_handle.clone(),
                twitter_handle: self.twitter_handle.clone(),
                mastodon_handle: self.mastodon_handle.clone(),
                github_handle: self.github_handle.clone(),
                facebook_handle: self.facebook_handle.clone(),
                personal_website: self.personal_website.clone(),

                followers_count: self.followers.len() as u32,
                following_count: self.following_list.len() as u32,
                follow_state: self.is_follower(noble_id),

                date_created: self.date_created,
                date_updated: self.date_updated,
            }
        } else {
            UserDetail {
                noble_id: self.noble_id,
                username: self.username.clone(),
                first_name: self.first_name.clone(),
                last_name: self.last_name.clone(),
                date_created: self.date_created,
                date_updated: self.date_updated,
                ..Default::default()
            }
        }
    }
}

#[cfg(test)]
impl Default for User {
    fn default() -> Self {
        User {
            principal: Principal::anonymous(),
            noble_id: NobleId::default(),
            canister_id: Principal::anonymous(),
            email: String::default(),
            username: String::default(),
            first_name: String::default(),
            last_name: String::default(),
            gender: None,
            degree: None,
            country: None,
            city: String::default(),
        
            preferred_pronouns: None,
            photo: vec![],
            search_by_email: bool::default(),
            bio: String::default(),
        
            account_privacy: AccountPrivacy::default(),
        
            linkedin_handle: String::default(),
            twitter_handle: String::default(),
            mastodon_handle: String::default(),
            github_handle: String::default(),
            facebook_handle: String::default(),
            personal_website: String::default(),
        
            followers: vec![],
            following_list: vec![],
            block_users: vec![],
            block_me_users: vec![],
            bookmarks: vec![],
            liked_posts: vec![],
            avatar_id: 0,
        
            date_created: TimestampMillis::default(),
            date_updated: TimestampMillis::default(),
        }
    }
}
