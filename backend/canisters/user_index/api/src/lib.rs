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
    UsernameChanged(Box<UsernameChanged>),
    AccountDeleted(Box<AccountDeleted>),
    UserFollowed(Box<FollowUser>),
    UserUnfollowed(Box<FollowUser>),
    UserBlocked(Box<BlockUser>),
    UserUnblocked(Box<BlockUser>),
    FollowRequest(Box<FollowRequest>),
    EmailChanged(Box<EmailChanged>),
    SearchByEmailChanged(Box<SearchByEmailChanged>),
    NameChanged(Box<NameChanged>),
    LocationChanged(Box<LocationChanged>),
    BioChanged(Box<BioChanged>),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BioChanged {
    pub noble_id: NobleId,
    pub bio: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LocationChanged {
    pub noble_id: NobleId,
    pub country: String,
    pub city: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NameChanged {
    pub noble_id: NobleId,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SearchByEmailChanged {
    pub noble_id: NobleId,
    pub search_by_email: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UsernameChanged {
    pub noble_id: NobleId,
    pub username: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AccountDeleted {
    pub noble_id: NobleId,
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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FollowRequest {
    pub sender_id: NobleId,
    pub receiver_id: NobleId,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EmailChanged {
    pub noble_id: NobleId,
    pub email: String,
}
