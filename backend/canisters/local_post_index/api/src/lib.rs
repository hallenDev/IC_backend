use serde::{Serialize, Deserialize};
mod lifecycle;
mod queries;
mod updates;

pub use lifecycle::*;
pub use queries::*;
use types::CanisterId;
pub use updates::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Event {
    LocalUserIndexCanisterAdded(Box<LocalUserIndexCanisterAdded>),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LocalUserIndexCanisterAdded {
    pub canister_id: CanisterId,
}