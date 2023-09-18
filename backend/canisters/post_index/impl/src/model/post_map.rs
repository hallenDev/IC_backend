use crate::model::post::Post;
use serde::{Serialize, Deserialize};
use std::collections::{HashMap, HashSet};
use types::{TimestampMillis, PostId, Category, NobleId, PostPrivacy, FileId, CanisterId};

#[derive(Serialize, Deserialize, Default)]
pub struct PostMap {
    posts: HashMap<PostId, Post>,
}

impl PostMap {
    pub fn add_post(
        &mut self,
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
    ) {
        let post = Post::new(post_id, canister_id, noble_id, title, description, category, link_url, video_url, attached_file_id, post_privacy, invited_users, now);
        self.posts.insert(post_id, post);
    }

    pub fn remove_post(
        &mut self,
        post_id: PostId,
    ) {
        self.posts.remove(&post_id);
    }

    pub fn get(&self, post_id: PostId) -> Option<&Post> {
        self.posts.get(&post_id)
    }

    pub fn get_mut(&mut self, post_id: PostId) -> Option<&mut Post> {
        self.posts.get_mut(&post_id)
    }

    pub fn len(&self) -> usize {
        self.posts.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Post> {
        self.posts.values()
    }

    #[cfg(test)]
    pub fn add_test_post(&mut self, post: Post) {
        self.posts.insert(post.post_id, post);
    }
}

