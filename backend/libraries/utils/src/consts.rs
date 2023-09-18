use candid::Principal;
use types::{CanisterId, Cycles};

// This only applies to the 'top level' canisters
pub const MIN_CYCLES_BALANCE: Cycles = 10_000_000_000_000; // 10T
pub const CREATE_CANISTER_CYCLES_FEE: Cycles = 100_000_000_000; // 0.1T cycles
pub const CYCLES_REQUIRED_FOR_UPGRADE: Cycles = 80_000_000_000; // 0.08T cycles

pub const GOVERNANCE_PRINCIPAL: CanisterId = Principal::from_slice(&[
    100, 43, 204, 148, 139, 11, 173, 96, 171, 247, 19, 200, 41, 195, 251, 13, 205, 187, 72, 99, 92, 229, 81, 217, 244, 79, 175, 233, 2
]);

pub const DEV_TEAM_PRINCIPAL: CanisterId = Principal::from_slice(&[
    115, 146, 94, 196, 182, 99, 207, 145, 16, 242, 22, 216, 76, 169, 9, 57, 100, 53, 112, 20, 251, 168, 138, 30, 16, 74, 207, 143, 2
]);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn governance_principal() {
        assert_eq!(
            GOVERNANCE_PRINCIPAL,
            Principal::from_text("oww3o-vtefp-gjjcy-lvvqk-x5ytz-au4h6-ynzw5-uqy24-4vi5t-5cpv7-uqe").unwrap()
        );
    }

    #[test]
    fn dev_team_principal() {
        assert_eq!(
            DEV_TEAM_PRINCIPAL,
            Principal::from_text("jyd6z-43tsj-pmjnt-dz6ir-b4qw3-bgksc-jzmq2-xafh3-vcfb4-eckz6-hqe").unwrap()
        );
    }
}
