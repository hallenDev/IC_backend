use crate::model::post::Post;
use serde::{Serialize, Deserialize};
use std::collections::{HashMap, HashSet};
use types::{TimestampMillis, PostId, Category, NobleId, PostPrivacy, FileId};

#[derive(Serialize, Deserialize, Default)]
pub struct PostMap {
    posts: HashMap<PostId, Post>,
}

impl PostMap {
    pub fn get(&self, post_id: PostId) -> Option<&Post> {
        self.posts.get(&post_id)
    }

    pub fn get_mut(&mut self, post_id: PostId) -> Option<&mut Post> {
        self.posts.get_mut(&post_id)
    }

    pub fn add_post(
        &mut self,
        post_id: PostId,
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
        let post = Post::new(post_id, noble_id, title, description, category, link_url, video_url, attached_file_id, post_privacy, invited_users, now);
        self.posts.insert(post_id, post);
    }

    pub fn remove_post(
        &mut self,
        post_id: PostId,
    ) {
        self.posts.remove(&post_id);
    }
    
    pub fn len(&self) -> usize {
        self.posts.len()
    }

    #[allow(dead_code)]
    pub fn iter(&self) -> impl Iterator<Item = &Post> {
        self.posts.values()
    }
}
