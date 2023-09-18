use candid::Principal;
use rand::{rngs::StdRng, Rng};
use types::{CanisterId, Cycles, TimestampMillis, TimestampNanos};

pub mod canister;
pub mod test;

pub trait Environment {
    fn now_nanos(&self) -> TimestampNanos;
    fn caller(&self) -> Principal;
    fn canister_id(&self) -> CanisterId;
    fn cycles_balance(&self) -> Cycles;
    fn rng(&mut self) -> &mut StdRng;

    fn now(&self) -> TimestampMillis {
        self.now_nanos() / 1_000_000
    }
}

pub fn get_random_id(rng: &mut StdRng) -> u64 {
    rng.gen_range(100_000_000_000_000_000u64..1_000_000_000_000_000_000u64)
}