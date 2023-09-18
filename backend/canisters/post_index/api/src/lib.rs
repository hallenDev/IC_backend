use std::collections::HashSet;

use serde::{Serialize, Deserialize};

mod lifecycle;
mod queries;
mod updates;

pub use lifecycle::*;
pub use queries::*;
use types::{TimestampMillis, NobleId, PostId, PostPrivacy, CanisterId};
pub use updates::*;


#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Event {
    NewComment(Box<NewComment>),
    CommentDeleted(Box<CommentDeleted>),
    PostLiked(Box<PostLiked>),
    PostUnliked(Box<PostUnliked>),
    PostEdited(Box<PostEdited>),
    PostDeleted(Box<PostDeleted>),
    LocalUserIndexAdded(Box<LocalUserIndexAdded>),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LocalUserIndexAdded {
    pub canister_id: CanisterId,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PostDeleted {
    pub post_id: PostId,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PostEdited {
    pub post_id: PostId,
    pub title: String,
    pub description: String,
    pub post_privacy: PostPrivacy,
    pub invited_users: HashSet<NobleId>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PostUnliked {
    pub noble_id: NobleId,
    pub post_id: PostId,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PostLiked {
    pub noble_id: NobleId,
    pub post_id: PostId,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CommentDeleted {
    pub post_id: PostId,
    pub comments_count: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NewComment {
    pub noble_id: NobleId,
    pub post_id: PostId,
    pub date_create: TimestampMillis,
}