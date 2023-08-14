use crate::model::post::Post;
use serde::{Serialize, Deserialize};
use std::collections::{HashMap, HashSet};
use types::{TimestampMillis, PostId, Category, NobleId, PostPrivacy, FileId};

#[derive(Serialize, Deserialize, Default)]
pub struct PostMap {
    posts: HashMap<PostId, Post>,
}

impl PostMap {
    pub fn add_post(
        &mut self,
        post_id: PostId,
        owner: NobleId,
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
        let msg = Post::new(post_id, owner, title, description, category, link_url, video_url, attached_file_id, post_privacy, invited_users, now);
        self.posts.insert(post_id, msg);
    }

    pub fn get(&self, post_id: PostId) -> Option<&Post> {
        self.posts.get(&post_id)
    }

    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.posts.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Post> {
        self.posts.values()
    }

    #[allow(dead_code)]
    #[cfg(test)]
    pub fn add_test_post(&mut self, msg: &Post) {
        self.posts.insert(msg.post_id, msg.clone());
    }
}

