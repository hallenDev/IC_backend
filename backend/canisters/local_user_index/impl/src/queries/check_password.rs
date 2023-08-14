use crate::{read_state, RuntimeState};
use crate::guards::caller_is_user_index_canister;
use ic_cdk_macros::query;
use local_user_index_canister::check_password::{Response::*, *};

#[query(guard = "caller_is_user_index_canister")]
fn check_password(args: Args) -> Response {
    read_state(|state| check_password_impl(args, state))
}

fn check_password_impl(args: Args, state: &RuntimeState) -> Response {
    match state.data.users.get(args.noble_id) {
        Some(user) => {
            match user.verify_password(&args.password) {
                true => Success,
                false => Error,
            }
        },
        None => UserNotFound,
    }
}
