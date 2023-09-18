use candid::CandidType;
use human_readable::ToHumanReadable;
use serde::{Deserialize, Serialize};
use types::CanisterId;

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Args {
    pub canister_id: Option<CanisterId>,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum Response {
    Success,
    AlreadyAdded,
    CyclesBalanceTooLow,
    InternalError(String),
}

#[derive(Serialize)]
pub struct HumanReadableArgs {
}

impl ToHumanReadable for Args {
    type Target = HumanReadableArgs;

    fn to_human_readable(&self) -> Self::Target {
        HumanReadableArgs {
        }
    }
}
