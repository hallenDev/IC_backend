use candid::{Principal, CandidType};
use serde::{Serialize, Deserialize};
use std::fmt::{Debug, Display, Formatter};

mod cycle;
mod http;
mod jwt;
mod post_summary;
mod referral_codes;
mod stable_principal;
mod user;
mod user_detail;
mod user_summary;
mod version;

pub use cycle::*;
pub use http::*;
pub use jwt::*;
pub use post_summary::*;
pub use referral_codes::*;
pub use stable_principal::*;
pub use user::*;
pub use user_detail::*;
pub use user_summary::*;
pub use version::*;

pub type CanisterId = Principal;
pub type Cycles = u128;
pub type Milliseconds = u64;
pub type Profile = String;
pub type TimestampMillis = u64;
pub type TimestampNanos = u64;

pub type NobleId = u64;
pub type PostId = u64;
pub type FileId = u128;

#[derive(CandidType, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Follower {
    pub noble_id: NobleId,
    pub is_approved: bool,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct FollowingUser {
    pub noble_id: NobleId,
    pub is_muted: bool,
}


#[derive(CandidType, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum PostPrivacy {
    AnyBody,
    Followers,
    SpecificUsers,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum AccountPrivacy {
    AnyBodyCanView,
    ApprovedFollowersCanView,
    OnlyMe,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UserId(CanisterId);

impl UserId {
    pub const fn new(canister_id: CanisterId) -> UserId {
        UserId(canister_id)
    }
}

impl From<Principal> for UserId {
    fn from(principal: Principal) -> Self {
        UserId(principal)
    }
}

impl From<UserId> for CanisterId {
    fn from(user_id: UserId) -> Self {
        user_id.0
    }
}

impl Debug for UserId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

impl Display for UserId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum PreferredPronouns {
    SheHer,
    TheyThem,
    HeHim,
    Other,
    Prefernottosay,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub enum AcademicDegree {
    #[serde(rename = "AA")]
    AssociatesofArts,
    #[serde(rename = "AS")]
    AssociatesofScience,
    #[serde(rename = "BVM")]
    BachelorofVeterinaryMedicine,
    #[serde(rename = "BA")]
    BachelorsofArts,
    #[serde(rename = "BEng")]
    BachelorsofEngineering,
    #[serde(rename = "BFA")]
    BachelorsofFineArts,
    #[serde(rename = "BS")]
    BachelorsofScience,
    #[serde(rename = "Mphil")]
    CandidateofPhilosophy,
    #[serde(rename = "PhD")]
    DoctorofPhilosophy,
    #[serde(rename = "GED")]
    GeneralEducationDevelopment,
    #[serde(rename = "HS")]
    HighSchool,
    #[serde(rename = "LI")]
    Licenciada,
    #[serde(rename = "MA")]
    MastersofArts,
    #[serde(rename = "MFA")]
    MastersofFineArts,
    #[serde(rename = "MRes")]
    MastersofResearch,
    #[serde(rename = "MS")]
    MastersofScience,
    #[serde(rename = "MDPhD")]
    MDPhD,
    #[serde(rename = "MD")]
    MedicalDoctor,
    #[serde(rename = "Other")]
    Other,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum Category {
    GeneralDiscussion,
    Questions,
    IntroduceYourself,
    UserFeedback,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Default)]
pub struct Empty {}
