use crate::{mutate_state, RuntimeState, USER_LIMIT};
use crate::guards::caller_is_user_index_canister;
use ic_cdk::update;
use local_user_index_canister::register_user::{Response::*, *};

#[update(guard = "caller_is_user_index_canister")]
fn register_user(args: Args) -> Response {
    mutate_state(|state| register_user_impl(args, state))
}

fn register_user_impl(args: Args, state: &mut RuntimeState) -> Response {
    let now = state.env.now();

    if state.data.users.len() >= USER_LIMIT {
        return UserLimitReached;
    }

    state
        .data
        .users
        .register(
            args.caller,
            args.noble_id,
            args.canister_id,
            args.email,
            args.username,
            now,
        );

    Success
}