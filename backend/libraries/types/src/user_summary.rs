use crate::{NobleId, TimestampMillis, CanisterId};
use candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct UserSummary {
    pub noble_id: NobleId,
    pub canister_id: CanisterId,
    pub username: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub date_created: TimestampMillis,
    pub country: String,
    pub city: String,
    pub bio: String,
}
