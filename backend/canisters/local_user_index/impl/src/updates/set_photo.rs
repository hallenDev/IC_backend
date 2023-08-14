use crate::model::user_map::UpdateUserResult;
use crate::{mutate_state, RuntimeState};
use ic_cdk_macros::update;
use local_user_index_canister::set_photo::{Response::*, *};
use types::check_jwt;

pub const MAX_PHOTO_SIZE: usize = 1_024 * 1_024; // 1MB

#[update]
fn set_photo(args: Args) -> Response {
    mutate_state(|state| set_photo_impl(args, state))
}

fn set_photo_impl(args: Args, state: &mut RuntimeState) -> Response {
    if let Some(jwt) = check_jwt(&args.jwt, state.env.now()) {
        if args.photo.len() > MAX_PHOTO_SIZE {
            return PhotoTooBig(MAX_PHOTO_SIZE);
        }

        if let Some(user) = state.data.users.get(jwt.noble_id) {
            let mut user_to_update = user.clone();
            user_to_update.photo = args.photo.clone();
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
