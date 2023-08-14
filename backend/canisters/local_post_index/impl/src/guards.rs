use crate::read_state;

pub fn caller_is_post_index_canister() -> Result<(), String> {
    if read_state(|state| state.caller_is_post_index_canister()) {
        Ok(())
    } else {
        Err("Permission Denied".to_owned())
    }
}
