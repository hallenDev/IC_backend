use candid::CandidType;
use serde::{Deserialize, Serialize};
use types::{TempId, SuccessLogin};

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Args {
    pub id: TempId,
    pub passkey: String,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum Response {
    Success(SuccessLogin),
    TempNotExist,
    InvalidPasskey,
    InternalError(String),
}
