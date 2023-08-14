use candid::CandidType;
use serde::{Serialize, Deserialize};
use types::CanisterId;

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Args {
    pub email: String,
    pub password: String,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum Response {
    Success(SuccessResult),
    UnregisteredUser,
    EmailOrPasswordIncorrect,
    InternalError(String),
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct SuccessResult {
    pub canister_id: CanisterId,
    pub jwt: String,
}