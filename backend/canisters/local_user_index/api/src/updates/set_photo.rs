use candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Args {
    pub jwt: String,
    pub photo: Vec<u8>,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum Response {
    Success,
    PermissionDenied,
    UserNotFound,
    PhotoTooBig(usize),
}
