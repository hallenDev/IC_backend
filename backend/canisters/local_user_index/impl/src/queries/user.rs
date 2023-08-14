use crate::{read_state, RuntimeState};
use crate::guards::caller_is_user_index_canister;
use ic_cdk_macros::query;
use local_user_index_canister::user::{Response::*, *};
use types::JWT;

#[query(guard = "caller_is_user_index_canister")]
fn user(args: Args) -> Response {
    read_state(|state| user_impl(args, state))
}

fn user_impl(args: Args, runtime_state: &RuntimeState) -> Response {
    let jwt = JWT::from_string(&args.jwt).unwrap();

    if let Some(user) = runtime_state.data.users.get(args.noble_id) {
        Success(user.to_detail(jwt.noble_id))
    } else {
        UserNotFound
    }
}
