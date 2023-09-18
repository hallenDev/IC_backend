use crate::{mutate_state, RuntimeState};
use ic_cdk_macros::update;
use local_user_index_canister::remove_bookmark::{Response::*, *};
use types::check_jwt;

#[update]
fn remove_bookmark(args: Args) -> Response {
    mutate_state(|state| remove_bookmark_impl( args, state))
}

fn remove_bookmark_impl(args: Args, state: &mut RuntimeState) -> Response {
    if let Some(jwt) = check_jwt(&args.jwt, state.env.now()) {
        if let Some(user) = state.data.users.get_mut(jwt.noble_id) {
            if user.remove_bookmark(args.post_id) {
                Success
            } else {
                BookmarkNotFound
            }
        } else {
            UserNotFound
        }
    } else {
        PermissionDenied
    }
}
