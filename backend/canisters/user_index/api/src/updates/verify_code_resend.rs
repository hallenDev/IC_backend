use candid::CandidType;
use serde::{Deserialize, Serialize};
use types::TempId;

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Args {
    pub id: TempId,
    pub email: String,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum Response {
    Success,
    EmailNotCorrect,
    TempNotExist,
    AlreadySent,
}
