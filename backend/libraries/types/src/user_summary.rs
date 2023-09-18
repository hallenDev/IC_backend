use crate::{NobleId, TimestampMillis, CanisterId, AcademicDegree, Country, AvatarId};
use candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct UserSummary {
    pub noble_id: NobleId,
    pub local_user_canister_id: CanisterId,
    pub avatar_id: AvatarId,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub date_created: TimestampMillis,
    pub degree: Option<AcademicDegree>,
    pub bio: String,
    pub follow_state: bool,
    pub country: Option<Country>,
    pub city: String,
    pub is_online: bool,
    pub loading_state: bool,
}
