use crate::{read_state, RuntimeState};
use ic_cdk_macros::query;
use types::check_jwt;
use user_index_canister::get_user_info_by_username::{Response::*, *};

#[query]
fn get_user_info_by_username(args: Args) -> Response {
    read_state(|state| get_user_info_by_username_impl(args, state))
}

fn get_user_info_by_username_impl(args: Args, state: &RuntimeState) -> Response {
    if check_jwt(&args.jwt, state.env.now()).is_some() {
        if let Some(user) = state.data.users.get_by_username(&args.username) {
            Success(user.get_user_info())
        } else {
            UserNotFound
        }
    } else {
        // PermissionDenied
        if let Some(user) = state.data.users.get_by_username(&args.username) {
            Success(user.get_user_info())
        } else {
            UserNotFound
        }
    }
}

