use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};
use types::{NobleId, CanisterId, TimestampMillis, UserSummary};

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct User {
    pub principal: Principal,
    pub noble_id: NobleId,
    pub canister_id: CanisterId,
    pub username: String,
    pub email: String,
    pub search_by_email: bool,
    pub first_name: String,
    pub last_name: String,
    pub date_created: TimestampMillis,
    pub country: String,
    pub city: String,
    pub bio: String,        // <= 100 character
}

impl User {
    pub fn new(
        principal: Principal,
        noble_id: NobleId,
        email: String,
        username: String,
        canister_id: CanisterId,
        now: TimestampMillis,
    ) -> Self {
        User {
            principal,
            noble_id,
            email,
            username,
            canister_id,
            search_by_email: false,
            first_name: String::new(),
            last_name: String::new(),
            date_created: now,
            bio: String::new(),
            country: String::new(),
            city: String::new(),
        }
    }


    pub fn to_summary(&self) -> UserSummary {
        UserSummary {
            noble_id: self.noble_id,
            canister_id: self.canister_id,
            username: self.username.clone(),
            email: self.email.clone(),
            first_name: self.first_name.clone(),
            last_name: self.last_name.clone(),
            date_created: self.date_created,
            country: self.country.clone(),
            city: self.city.clone(),
            bio: self.bio.clone(),
        }
   }
}

#[cfg(test)]
impl Default for User {
    fn default() -> Self {
        Self {
            principal: Principal::anonymous(),
            noble_id: 0,
            canister_id: Principal::anonymous(),
            username: String::new(),
            email: String::new(),
            search_by_email: false,
            first_name: String::new(),
            last_name: String::new(),
            date_created: 0,
            country: String::new(),
            city: String::new(),
            bio: String::new(), 
        }
    }
}