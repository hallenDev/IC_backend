use crate::read_state;

pub fn caller_is_user_index_canister() -> Result<(), String> {
    if read_state(|state| state.caller_is_user_index_canister()) {
        Ok(())
    } else {
        Err("Permission Denied".to_owned())
    }
}

#[allow(dead_code)]
pub fn caller_is_known_canister() -> Result<(), String> {
    if read_state(|state| state.caller_is_known_canister()) {
        Ok(())
    } else {
        Err("Permission Denied".to_owned())
    }
}
