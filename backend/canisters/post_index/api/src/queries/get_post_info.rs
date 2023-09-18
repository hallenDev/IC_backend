use candid::CandidType;
use serde::{Serialize, Deserialize};
use types::{PostId, CanisterId};

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Args {
    pub jwt: String,
    pub post_id: PostId,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum Response {
    Success(SuccessResult),
    PermissionDenied,
    PostNotFound,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct SuccessResult {
    pub post_id: PostId,
    pub local_post_canister_id: CanisterId,
}
