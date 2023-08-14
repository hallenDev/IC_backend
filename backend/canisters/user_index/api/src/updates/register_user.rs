use candid::CandidType;
use serde::{Deserialize, Serialize};
use types::CanisterId;

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Args {
    pub username: String,
    pub email: String,
    pub password: String,
    pub password_confirm: String,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum Response {
    Success(SuccessResult),
    AlreadyRegistered,
    UserLimitReached,
    UsernameInvalid,
    UsernameTooShort(u16),
    UsernameTooLong(u16),
    UsernameAlreayExist,
    CyclesBalanceTooLow,
    InternalError(String),
    PublicKeyInvalid(String),
    ReferralCodeInvalid,
    ReferralCodeAlreadyClaimed,
    ReferralCodeExpired,
    PasswordIsRequired,
    PasswordLengthIsInvalid(u16, u16),
    PasswordIsnotMatch,
    PasswordHashError,
    EmailIsInvalid,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct SuccessResult {
    pub canister_id: CanisterId,
}
