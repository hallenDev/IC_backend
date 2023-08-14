use serde::{Serialize, Deserialize};
mod lifecycle;
mod queries;
mod updates;

pub use lifecycle::*;
pub use queries::*;
use types::NobleId;
pub use updates::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Event {
    UserFollowed(Box<FollowUser>),
    UserUnfollowed(Box<FollowUser>),
    UserBlocked(Box<BlockUser>),
    UserUnblocked(Box<BlockUser>),
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