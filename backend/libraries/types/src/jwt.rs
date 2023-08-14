use serde::{Deserialize, Serialize};
// use jwt_simple::prelude::*;

use crate::{NobleId, TimestampMillis};

pub const EXP_TIME: u64 = 60 * 60 * 1_000; // 1 hour

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct JWT {
    pub noble_id: NobleId,
    pub email: String,
    pub username: String,
    pub iat: u64,
    pub exp: u64,
}

impl JWT {
    pub fn new(noble_id: NobleId, email: String, username: String, iat: u64) -> Self {
        Self {
            noble_id,
            email,
            username,
            iat,
            exp: iat + EXP_TIME,
        }
    }

    // pub fn get_key_pair() -> Option<Ed25519KeyPair> {
    //     let key: [u8; 64] = [73, 19, 226, 209, 207, 251, 30, 214, 192, 187, 171, 4, 246, 152, 187, 146, 48, 153, 244, 235, 243, 43, 9, 10, 47, 197, 136, 108, 47, 236, 97, 103, 172, 7, 158, 232, 111, 153, 186, 100, 
    //     145, 241, 219, 121, 235, 214, 154, 234, 102, 14, 182, 197, 214, 212, 16, 122, 15, 37, 144, 7, 156, 80, 34, 42];
    //     match Ed25519KeyPair::from_bytes(&key) {
    //         Ok(ok) => Some(ok),
    //         Err(_) => return None,
    //     }
    // }

    pub fn to_string(&self) -> Option<String> {
        Some(serde_json::to_string(self).unwrap())
        // let key_pair = JWT::get_key_pair()?;

        // // create claims valid for 1 hour
        // let claims = Claims::with_custom_claims(self.clone(), Duration::from_secs(EXP_TIME));

        // match key_pair.sign(claims) {
        //     Ok(ok) => return Some(ok),
        //     Err(_) => return None,
        // }
    }

    pub fn from_string(token: &str) -> Option<Self> {
        match serde_json::from_str(token) {
            Ok(ok) => Some(ok),
            Err(_) => None,
        }
        // let key_pair = JWT::get_key_pair()?;

        // // a public key can be extracted from a key pair:
        // let public_key = key_pair.public_key();

        // match public_key.verify_token::<Self>(&token, None) {
        //     Ok(ok) => Some(ok.custom),
        //     Err(_) => return None,
        // }
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
