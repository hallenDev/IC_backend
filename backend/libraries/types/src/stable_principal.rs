use candid::{CandidType, Principal};
use std::fmt::{Debug, Display, Formatter};
use serde::{Serialize, Deserialize};

#[derive(CandidType, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StablePrincipal(pub(crate) Principal);

impl From<Principal> for StablePrincipal {
    fn from(principal: Principal) -> Self {
        StablePrincipal(principal)
    }
}

impl From<StablePrincipal> for Principal {
    fn from(user_id: StablePrincipal) -> Self {
        user_id.0
    }
}

impl Debug for StablePrincipal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

impl Display for StablePrincipal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}
