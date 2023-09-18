use candid::Principal;
use serde::{Deserialize, Serialize};
use magic_crypt::{MagicCryptTrait, MagicCrypt256};

use crate::{NobleId, TimestampMillis, CanisterId};

pub const EXP_TIME: u64 = 3 * 24 * 60 * 60 * 1_000; // 3 days

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct JWT {
    pub iat: u64,
    pub exp: u64,
    pub noble_id: NobleId,
    pub local_canister_id: CanisterId,
    pub email: String,
    pub username: String,
}

impl JWT {
    pub fn new(noble_id: NobleId, local_canister_id: CanisterId, email: String, username: String, iat: u64) -> Self {
        Self {
            noble_id,
            local_canister_id,
            email,
            username,
            iat,
            exp: iat + EXP_TIME,
        }
    }

    pub fn new_for_test(noble_id: NobleId, now: TimestampMillis) -> Self {
        Self {
            noble_id,
            iat: now,
            exp: now + EXP_TIME,
            ..Default::default()
        }
    }

    pub fn get_key() -> &'static str {
        "magickey2"
    }

    pub fn to_string(&self) -> Option<String> {
        match serde_json::to_string(self) {
            Ok(ok) => {
                let mcrypt = MagicCrypt256::new(Self::get_key(), None::<String>);
                let encrypted_string = mcrypt.encrypt_str_to_base64(&ok);
                Some(encrypted_string.trim_end_matches("=").to_string())
            },
            Err(_) => None,
        }
    }

    pub fn from_string(token: &str) -> Option<Self> {
        let mcrypt = MagicCrypt256::new(Self::get_key(), None::<String>);
        match mcrypt.decrypt_base64_to_string(token) {
            Ok(ok) => {
                match serde_json::from_str(&ok) {
                    Ok(ok) => Some(ok),
                    Err(_) => None,
                }
            },
            Err(_) => None,
        }
    }

    pub fn is_expired(&self, now: TimestampMillis) -> bool {
        self.exp < now
    }
}

pub fn check_jwt(str: &str, now: TimestampMillis) -> Option<JWT> {
    if let Some(jwt) = JWT::from_string(str) {
        if jwt.is_expired(now) {
            return None;
        }
        return Some(jwt);
    } else {
        return None;
    }
}

impl Default for JWT {
    fn default() -> Self {
        JWT {
            iat: 0,
            exp: 0,
            noble_id: 0,
            local_canister_id: Principal::anonymous(),
            email: "".to_string(),
            username: "".to_string(),
        }
    }
}
