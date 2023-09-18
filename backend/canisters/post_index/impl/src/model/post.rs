use candid::CandidType;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use types::{
    TimestampMillis, PostId, Category, NobleId, PostPrivacy, FileId, PostSummary, CanisterId
};

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Post {
    pub post_id: PostId,
    pub canister_id: CanisterId,
    pub noble_id: NobleId,
    pub title: String,
    // up to 300 character.
    pub description: String,
    pub category: Category,
    pub link_url: String,
    pub video_url: String,
    pub attached_file_id: FileId,

    pub liked_users_count: u32,
    pub comments_count: u32,
    // up to five.(first commenter, second commenter, recent commenter X 3)
    pub contributed_users: Vec<NobleId>,
    pub post_privacy: PostPrivacy,
    pub invited_users: HashSet<NobleId>,

    pub date_created: TimestampMillis,
    pub date_last_commented: TimestampMillis,
}

impl Post {
    pub fn new(
        post_id: PostId,
        canister_id: CanisterId,
        noble_id: NobleId,
        title: String,
        description: String,
        category: Category,
        link_url: String,
        video_url: String,
        attached_file_id: FileId,
        post_privacy: PostPrivacy,
        invited_users: HashSet<NobleId>,
        now: TimestampMillis,
    ) -> Self {
        Post {
            post_id,
            canister_id,
            noble_id,
            title,
            description,
            category,

            link_url,
            video_url,
            attached_file_id,

            post_privacy,
            invited_users,
            liked_users_count: 0,
            comments_count: 0,
            contributed_users: Vec::with_capacity(5),

            date_created: now,
            date_last_commented: now,
        }
    }

    pub fn to_summary(&self, like_state: bool, bookmark_state: bool) -> PostSummary {
        PostSummary {
            post_id: self.post_id,
            noble_id: self.noble_id,
            title: self.title.clone(),
            description: self.description.clone(),
            category: self.category,
            link_url: self.link_url.clone(),
            video_url: self.video_url.clone(),
            attached_file_id: self.attached_file_id,
            liked_users_count: self.liked_users_count,
            comments_count: self.comments_count,
            date_created: self.date_created,
            date_last_commented: self.date_last_commented,
            like_state,
            bookmark_state,
            loading_like: false,
            loading_bookmark: false,
            loading_delete: false,
        }
    }

    pub fn can_show(
        &self,
        noble_id: NobleId,
        category: &Option<Category>,
        following_list: &Vec<NobleId>,
        block_me_users: &Vec<NobleId>,
    ) -> bool {
        if block_me_users.contains(&self.noble_id) {
            return false;
        }

        if self.category == Category::UserFeedback {
            return false;
        }

        if self.noble_id == noble_id ||
           self.post_privacy == PostPrivacy::Everyone ||
          (self.post_privacy == PostPrivacy::Followers && following_list.contains(&self.noble_id)) ||
          (self.post_privacy == PostPrivacy::SpecificUsers && self.invited_users.contains(&noble_id)) {
            if let Some(cate) = category {
                *cate == self.category
            } else {
                true
            }
        } else {
            false
        }
    }
}
#[cfg(test)]
use candid::Principal;

#[cfg(test)]
impl Default for Post {
    fn default() -> Self {
        Post {
            post_id: 0,
            canister_id: Principal::anonymous(),
            noble_id: 0,
            title: String::new(),
            description: String::new(),
            category: Category::GeneralDiscussion,

            link_url: String::new(),
            video_url: String::new(),
            attached_file_id: 0,

            post_privacy: PostPrivacy::Everyone,
            invited_users: HashSet::new(),
            liked_users_count: 0,
            comments_count: 0,
            contributed_users: Vec::with_capacity(5),

            date_created: 0,
            date_last_commented: 0,
        }
    }
}
