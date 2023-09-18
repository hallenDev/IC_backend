use serde::{Serialize, Deserialize};

mod lifecycle;
mod queries;
mod updates;

pub use lifecycle::*;
pub use queries::*;
use types::{NobleId, Country, AcademicDegree, PostId, CommentId, AvatarId, CanisterId};
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
    ProfileChanged(Box<ProfileChanged>),
    AccountChanged(Box<AccountChanged>),
    PhotoChanged(Box<PhotoChanged>),
    CommentLiked(Box<CommentLiked>),
    CommentUnliked(Box<CommentUnliked>),
    LocalPostIndexAdded(Box<LocalPostIndexAdded>),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LocalPostIndexAdded {
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
pub struct ProfileChanged {
    pub noble_id: NobleId,
    pub first_name: String,
    pub last_name: String,
    pub degree: Option<AcademicDegree>,
    pub country: Option<Country>,
    pub city: String,
    pub bio: String,        // <= 100 character
    pub avatar_id: AvatarId,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AccountChanged {
    pub noble_id: NobleId,
    pub username: String,
    pub email: String,
    pub search_by_email: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PhotoChanged {
    pub noble_id: NobleId,
    pub avatar_id: AvatarId,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum EmailEvent {
    RegisterUser(Box<RegisterUser>),
    ResetPassword(Box<ResetPassword>),
    ResetPasswordVerify(Box<ResetPasswordVerify>),
    Feedback(Box<Feedback>),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Feedback {
    pub email: String,
    pub feedback: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RegisterUser {
    pub email: String,
    pub name: String,
    pub passkey: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ResetPasswordVerify {
    pub email: String,
    pub name: String,
    pub passkey: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ResetPassword {
    pub email: String,
    pub name: String,
    pub password: String,
}
