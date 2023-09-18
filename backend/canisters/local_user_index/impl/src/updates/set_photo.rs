use crate::{mutate_state, RuntimeState, MAX_PHOTO_SIZE};
use ic_cdk_macros::update;
use local_user_index_canister::set_photo::{Response::*, *};
use user_index_canister::{Event as UserIndexEvent, PhotoChanged};
use types::check_jwt;

#[update]
fn set_photo(args: Args) -> Response {
    mutate_state(|state| set_photo_impl(args, state))
}

fn set_photo_impl(args: Args, state: &mut RuntimeState) -> Response {
    if let Some(jwt) = check_jwt(&args.jwt, state.env.now()) {
        match prepare(&args) {
            Ok(()) => {},
            Err(error) => return error,
        }

        state.data.users.update_avatar_id(jwt.noble_id, state.env.rng());

        if let Some(user) = state.data.users.get_mut(jwt.noble_id) {
            user.photo = args.photo;
            user.date_updated = state.env.now();
            let avatar_id = user.avatar_id;

            state.push_event_to_user_index(UserIndexEvent::PhotoChanged(Box::new(
                PhotoChanged {
                    noble_id: jwt.noble_id,
                    avatar_id,
                }
            )));

            Success(avatar_id)
        } else {
            UserNotFound
        }
    } else {
        PermissionDenied
    }
}

fn prepare(args: &Args) -> Result<(), Response> {
    let mut error = ErrorResult::new();

    if args.photo.is_empty() {
        error.photo = format!("Photo is required.");
    }

    if args.photo.len() > MAX_PHOTO_SIZE {
        error.photo = format!("Image size should be less than 1MB");
    }

    if error.is_error() {
        Err(Error(error))
    } else {
        Ok(())
    }
}
