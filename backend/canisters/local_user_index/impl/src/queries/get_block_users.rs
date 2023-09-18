use crate::{read_state, RuntimeState};
use ic_cdk_macros::query;
use local_user_index_canister::get_block_users::{Response::*, *};
use types::check_jwt;

#[query]
fn get_block_users(args: Args) -> Response {
    read_state(|state| get_block_users_impl(&args, state))
}

fn get_block_users_impl(args: &Args, state: &RuntimeState) -> Response {
    let now = state.env.now();

    if let Some(jwt) = check_jwt(&args.jwt, now) {
        let noble_id = args.noble_id.unwrap_or(jwt.noble_id);

        if let Some(user) = state.data.users.get(noble_id) {
            Success(user.block_users.clone())
        } else {
            UserNotFound
        }
    } else {
        PermissionDenied
    }
}