use crate::{read_state, RuntimeState};
use crate::guards::caller_is_post_index_canister;
use canister_api_macros::query_msgpack;
use local_user_index_canister::c2c_get_block_users::{Response::*, *};

#[query_msgpack(guard = "caller_is_post_index_canister")]
fn c2c_get_block_users(args: Args) -> Response {
    read_state(|state| c2c_get_block_users_impl(args, state))
}

fn c2c_get_block_users_impl(args: Args, state: &RuntimeState) -> Response {
    if let Some(user) = state.data.users.get(args.noble_id) {
        return Success(user.block_users.clone());
    }
    UserNotFound
}

