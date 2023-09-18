use candid::{Principal, CandidType};
use serde::{Serialize, Deserialize};
use std::fmt::{Debug, Display, Formatter};

mod canister_wasm;
mod comment_detail;
mod cycle;
mod http;
mod jwt;
mod post_detail;
mod post_summary;
mod referral_codes;
mod stable_principal;
mod timestamped;
mod user;
mod user_detail;
mod user_info;
mod user_summary;
mod version;

pub use canister_wasm::*;
pub use comment_detail::*;
pub use cycle::*;
pub use http::*;
pub use jwt::*;
pub use post_detail::*;
pub use post_summary::*;
pub use referral_codes::*;
pub use stable_principal::*;
pub use timestamped::*;
pub use user::*;
pub use user_detail::*;
pub use user_info::*;
pub use user_summary::*;
pub use version::*;

pub type CanisterId = Principal;
pub type Cycles = u128;
pub type Milliseconds = u64;
pub type Profile = String;
pub type TimestampMillis = u64;
pub type TimestampNanos = u64;

pub type TempId = u32;
pub type NobleId = u64;
pub type AvatarId = u64;
pub type PostId = u64;
pub type FileId = u128;
pub type CommentId = u32;

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
    Everyone,
    Followers,
    SpecificUsers,
}

impl Default for PostPrivacy {
    fn default() -> Self {
        Self::Everyone
    }
}

#[derive(CandidType, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum AccountPrivacy {
    Everyone,
    ApprovedFollowers,
    OnlyMe,
}

impl Default for AccountPrivacy {
    fn default() -> Self {
        Self::Everyone
    }
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

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Copy)]
pub enum PreferredPronouns {
    SheHer,
    TheyThem,
    HeHim,
    Other,
    Prefernottosay,
}

impl Default for PreferredPronouns {
    fn default() -> Self {
        Self::Other
    }
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Copy)]
pub enum AcademicDegree {
    AA, AS, BVetMed, BA, BEng, BFA, BS, Mphil, PhD, GED, HS, Lic, MA, MFA, MRes, MS, MDPhD, MD, Other,
}

impl Default for AcademicDegree {
    fn default() -> Self {
        Self::HS
    }
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Copy, PartialEq, Eq)]
pub enum Country {
    AD, AE, AF, AG, AI, AL, AM, AN, AO, AQ, AR, AS, AT, AU, AW, AZ, BA, BB, BD, BE, BF, BG, BH, BI, BJ, BM, BN, BO, BR, BS,
    BT, BV, BW, BY, BZ, CA, CC, CD, CF, CG, CH, CI, CK, CL, CM, CN, CO, CR, CU, CV, CX, CY, CZ, DE, DJ, DK, DM, DO, DZ, EC,
    EE, EG, EH, ER, ES, ET, FI, FJ, FK, FM, FO, FR, GA, GB, GD, GE, GF, GG, GH, GI, GL, GM, GN, GP, GQ, GR, GS, GT, GU, GW,
    GY, GZ, HK, HM, HN, HR, HT, HU, ID, IE, IL, IM, IN, IO, IQ, IR, IS, IT, JE, JM, JO, JP, KE, KG, KH, KI, KM, KN, KP, KR,
    KW, KY, KZ, LA, LB, LC, LI, LK, LR, LS, LT, LU, LV, LY, MA, MC, MD, ME, MG, MH, MK, ML, MM, MN, MO, MP, MQ, MR, MS, MT,
    MU, MV, MW, MX, MY, MZ, NA, NC, NE, NF, NG, NI, NL, NO, NP, NR, NU, NZ, OM, PA, PE, PF, PG, PH, PK, PL, PM, PN, PR, PS,
    PT, PW, PY, QA, RE, RO, RS, RU, RW, SA, SB, SC, SD, SE, SG, SH, SI, SJ, SK, SL, SM, SN, SO, SR, ST, SV, SY, SZ, TC, TD,
    TF, TG, TH, TJ, TK, TL, TM, TN, TO, TR, TT, TV, TW, TZ, UA, UG, UM, US, UY, UZ, VA, VC, VE, VG, VI, VN, VU, WF, WS, XK,
    YE, YT, ZA, ZM, ZW,
}

impl Default for Country {
    fn default() -> Self {
        Self::AD
    }
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Copy, PartialEq, Eq)]
pub enum Gender {
    Male, Female,
}

impl Default for Gender {
    fn default() -> Self {
        Self::Male
    }
}

#[derive(CandidType, Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum Category {
    GeneralDiscussion,
    Questions,
    IntroduceYourself,
    UserFeedback,
}

impl Default for Category {
    fn default() -> Self {
        Self::GeneralDiscussion
    }
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Default)]
pub struct Empty {}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct SuccessLogin {
    pub jwt: String,
    pub noble_id: NobleId,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub canister_id: CanisterId,
    pub avatar_id: AvatarId,
}
