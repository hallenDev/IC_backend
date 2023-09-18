use candid::CandidType;
use serde::{Deserialize, Serialize};
use types::{TimestampMillis, TempId};

use user_index_canister::register_user::Args as RegisterUserArgs;

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Temp {
    pub temp_id: TempId,
    pub is_used: bool,
    pub expired_time: TimestampMillis,
    pub passkey: String,
    pub email: String,
    pub temp_data: TempData,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub enum TempData {
    RegisterUser(RegisterUserArgs),
    ResetPassword(ResetPassword),
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct ResetPassword {
    pub name: String,
    pub password: String,
}

pub enum TempDataType {
    RegisterUser,
    ResetPassword,
}
