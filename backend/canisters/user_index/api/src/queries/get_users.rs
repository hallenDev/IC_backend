use candid::CandidType;
use serde::{Serialize, Deserialize};
use types::{TimestampMillis, UserSummary, NobleId};

#[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
pub struct Args {
    pub jwt: String,
    pub page: u32,
    pub limit: u32,
    pub following_list: Vec<NobleId>,
    pub block_me_users: Vec<NobleId>,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum Response {
    Success(SuccessResult),
    PermissionDenied,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct SuccessResult {
    pub total_users_count: u32,
    pub users: Vec<UserSummary>,
    pub timestamp: TimestampMillis,
}
