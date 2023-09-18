use candid::CandidType;
use serde::{Deserialize, Serialize};
use types::PostId;

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Args {
    pub jwt: String,
    pub post_id: PostId,
}

#[derive(CandidType, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Response {
    Success,
    PermissionDenied,
    BookmarkNotFound,
    UserNotFound,
}
