use crate::read_state;

pub fn caller_is_governance_principal() -> Result<(), String> {
    if read_state(|state| state.caller_is_governance_principal()) {
        Ok(())
    } else {
        Err("Permission Denied".to_owned())
    }
}

pub fn caller_is_known_canister() -> Result<(), String> {
    if read_state(|state| state.caller_is_known_canister()) {
        Ok(())
    } else {
        Err("Permission Denied".to_owned())
    }
}
