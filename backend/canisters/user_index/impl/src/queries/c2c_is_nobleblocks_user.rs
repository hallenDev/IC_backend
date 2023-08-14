use crate::{read_state, RuntimeState};
use canister_api_macros::query_msgpack;
use user_index_canister::c2c_is_nobleblocks_user::{Response::*, *};

#[query_msgpack]
fn c2c_is_nobleblocks_user(args: Args) -> Response {
    read_state(|state| c2c_is_nobleblocks_user_impl(args, state))
}

fn c2c_is_nobleblocks_user_impl(args: Args, state: &RuntimeState) -> Response {
    if state.data.users.get(args.noble_id).is_some() {
        return Yes;
    }
    No
}

