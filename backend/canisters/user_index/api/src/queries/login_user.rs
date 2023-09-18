use candid::CandidType;
use serde::{Serialize, Deserialize};
use types::SuccessLogin;

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Args {
    pub email: String,
    pub password: String,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum Response {
    Success(SuccessLogin),
    UnregisteredUser,
    EmailOrPasswordIncorrect,
    InternalError(String),
}
