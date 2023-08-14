use candid::CandidType;
use serde::{Deserialize, Serialize};
use types::PreferredPronouns;

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Args {
    pub jwt: String,
    pub preferred_pronouns: Option<PreferredPronouns>,
}

#[derive(CandidType, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Response {
    Success,
    PermissionDenied,
    UserNotFound,
}
