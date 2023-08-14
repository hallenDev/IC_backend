use candid::CandidType;
use serde::{Serialize, Deserialize};
use types::{TimestampMillis, UserSummary};

#[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
pub struct Args {
    pub jwt: String,
    pub search_term: String,
    pub max_results: u8,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum Response {
    Success(SuccessResult),
    PermissionDenied,
    InternalError(String),
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct SuccessResult {
    pub users: Vec<UserSummary>,
    pub timestamp: TimestampMillis,
}
