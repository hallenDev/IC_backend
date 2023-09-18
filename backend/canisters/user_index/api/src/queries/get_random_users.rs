use candid::CandidType;
use serde::{Serialize, Deserialize};
use types::UserInfo;

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Args {
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum Response {
    Success(Vec<UserInfo>),
}
