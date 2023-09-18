use crate::{mutate_state, RuntimeState};
use ic_cdk_macros::update;
use local_user_index_canister::unmute_user::{Response::*, *};
use types::check_jwt;

#[update]
fn unmute_user(args: Args) -> Response {
    mutate_state(|state| unmute_user_impl(args, state))
}

fn unmute_user_impl(args: Args, state: &mut RuntimeState) -> Response {
    if let Some(jwt) = check_jwt(&args.jwt, state.env.now()) {
        if let Some(user) = state.data.users.get_mut(jwt.noble_id) {
            if !user.is_muted(args.noble_id) {
                return UserNotFound;
            }
    
            user.unmute_user(args.noble_id);

            Success
        } else {
            UserNotFound
        }
    } else {
        PermissionDenied
    }
}
