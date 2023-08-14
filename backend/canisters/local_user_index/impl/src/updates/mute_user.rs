use crate::model::user_map::UpdateUserResult;
use crate::{mutate_state, RuntimeState};
use ic_cdk_macros::update;
use local_user_index_canister::mute_user::{Response::*, *};
use types::check_jwt;

#[update]
fn mute_user(args: Args) -> Response {
    mutate_state(|state| mute_user_impl(args, state))
}

fn mute_user_impl(args: Args, state: &mut RuntimeState) -> Response {
    if let Some(jwt) = check_jwt(&args.jwt, state.env.now()) {
        if let Some(user) = state.data.users.get(jwt.noble_id) {
            if state.data.users.get(args.noble_id).is_none() {
                return UserNotFound;
            }
    
            if user.is_muted(args.noble_id) {
                return AlreadyMuted;
            }
    
            if !user.is_following(args.noble_id) {
                return NotFollowingUser;
            }
    
            let mut user_to_update = user.clone();
    
            user_to_update.mute_user(args.noble_id);
            let now = state.env.now();
    
            match state.data.users.update(user_to_update, now) {
                UpdateUserResult::Success => Success,
                UpdateUserResult::UserNotFound => UserNotFound,
            }
        } else {
            UserNotFound
        }
    } else {
        PermissionDenied
    }
}
