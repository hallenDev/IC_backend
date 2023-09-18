use serde::{Serialize, Deserialize};
mod lifecycle;
mod queries;
mod updates;

pub use lifecycle::*;
pub use queries::*;
use types::{NobleId, PostId, CommentId, CanisterId};
pub use updates::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Event {
    UserFollowed(Box<FollowUser>),
    UserUnfollowed(Box<FollowUser>),
    UserBlocked(Box<BlockUser>),
    UserUnblocked(Box<BlockUser>),
    UsernameChanged(Box<UsernameChanged>),
    CommentLiked(Box<CommentLiked>),
    CommentUnliked(Box<CommentUnliked>),
    LocalPostIndexCanisterAdded(Box<LocalPostIndexCanisterAdded>),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LocalPostIndexCanisterAdded {
    pub canister_id: CanisterId,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CommentUnliked {
    pub noble_id: NobleId,
    pub post_id: PostId,
    pub comment_id: CommentId,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CommentLiked {
    pub noble_id: NobleId,
    pub post_id: PostId,
    pub comment_id: CommentId,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UsernameChanged {
    pub noble_id: NobleId,
    pub username: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FollowUser {
    pub sender_id: NobleId,
    pub receiver_id: NobleId,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BlockUser {
    pub sender_id: NobleId,
    pub receiver_id: NobleId,
}