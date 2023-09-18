use candid::CandidType;
use serde::{Serialize, Deserialize};
use types::PostId;

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Args {
    pub post_id: PostId,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum Response {
    Yes,
    No,
}
