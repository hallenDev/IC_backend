use crate::{read_state, RuntimeState};
use ic_cdk_macros::query;
use local_user_index_canister::get_user::{Response::*, *};
use types::check_jwt;

#[query]
fn get_user(args: Args) -> Response {
    read_state(|state| get_user_impl(args, state))
}

fn get_user_impl(args: Args, state: &RuntimeState) -> Response {
    if let Some(jwt) = check_jwt(&args.jwt, state.env.now()) {
        if let Some(user) = state.data.users.get(args.noble_id) {
            Success(user.to_detail(jwt.noble_id))
        } else {
            UserNotFound
        }
    } else {
        // PermissionDenied
        if let Some(user) = state.data.users.get(args.noble_id) {
            Success(user.to_detail(0))
        } else {
            UserNotFound
        }
    }
}
