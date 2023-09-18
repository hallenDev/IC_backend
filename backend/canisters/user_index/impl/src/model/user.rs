use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};
use types::{NobleId, CanisterId, TimestampMillis, UserSummary, Country, AcademicDegree, UserInfo, SuccessLogin, JWT, AvatarId};

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
    pub degree: Option<AcademicDegree>,
    pub country: Option<Country>,
    pub city: String,
    pub bio: String,
    #[serde(default)]
    pub password: String,
    #[serde(default)]
    pub avatar_id: AvatarId,
}

impl User {
    pub fn new(
        principal: Principal,
        noble_id: NobleId,
        email: String,
        username: String,
        password: String,
        canister_id: CanisterId,
        now: TimestampMillis,
    ) -> Self {
        User {
            principal,
            noble_id,
            email,
            username,
            password,
            canister_id,
            search_by_email: false,
            first_name: String::new(),
            last_name: String::new(),
            date_created: now,
            bio: String::new(),
            country: None,
            degree: None,
            city: String::new(),
            avatar_id: 0,
        }
    }


    pub fn to_summary(&self, follow_state: bool) -> UserSummary {
        UserSummary {
            noble_id: self.noble_id,
            local_user_canister_id: self.canister_id,
            avatar_id: self.avatar_id,
            username: self.username.clone(),
            first_name: self.first_name.clone(),
            last_name: self.last_name.clone(),
            date_created: self.date_created,
            degree: self.degree,
            bio: self.bio.clone(),
            follow_state,
            country: self.country,
            city: self.city.clone(),
            is_online: true,
            loading_state: false,
        }
    }

    pub fn get_user_info(&self) -> UserInfo {
        UserInfo {
            noble_id: self.noble_id,
            canister_id: self.canister_id,
            username: self.username.clone(),
            first_name: self.first_name.clone(),
            last_name: self.last_name.clone(),
            avatar_id: self.avatar_id,
        }
    }
    
    pub fn verify_password(&self, password: &str) -> bool {
        match argon2::verify_encoded(&self.password, password.as_bytes()) {
            Ok(result) => result,
            Err(_) => false,
        }
    }

    pub fn get_login_info(&self, now: TimestampMillis) -> Result<SuccessLogin, String> {
        let jwt = JWT::new(self.noble_id, self.canister_id, self.email.clone(), self.username.clone(), now);

        let jwt = match jwt.to_string() {
            Some(j) => j,
            None => return Err(format!("JWT parsing error")),
        };

        Ok(SuccessLogin{
            jwt,
            noble_id: self.noble_id,
            username: self.username.clone(),
            first_name: self.first_name.clone(),
            last_name: self.last_name.clone(),
            canister_id: self.canister_id,
            avatar_id: self.avatar_id,
        })
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
            password: String::new(),
            email: String::new(),
            search_by_email: false,
            first_name: String::new(),
            last_name: String::new(),
            date_created: 0,
            country: None,
            degree: None,
            city: String::new(),
            bio: String::new(), 
            avatar_id: 0,
        }
    }
}