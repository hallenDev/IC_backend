use crate::model::user_map::UpdateUserResult;
use crate::{mutate_state, RuntimeState, read_state};
use candid::Principal;
use ic_cdk_macros::update;
use local_user_index_canister::set_username::{Response::*, *};
use types::{check_jwt, NobleId};
use utils::username_validation::{validate_username, UsernameValidationError};
use user_index_canister::{Event as UserIndexEvent, UsernameChanged};

#[update]
async fn set_username(args: Args) -> Response {
    if let Some(jwt) = check_jwt(&args.jwt, read_state(|state| state.env.now())) {
        let (username_case_insensitive_changed, user_index_canister_id) = match read_state(|state| prepare(jwt.noble_id, &args, state)) {
            Ok(ok) => ok,
            Err(error) => return error,
        };

        if username_case_insensitive_changed {
            match user_index_canister_c2c_client::check_username(
                user_index_canister_id,
                &user_index_canister::check_username::Args{username: args.username.clone()}
            ).await {
                Ok(response) => {
                    match response {
                        user_index_canister::check_username::Response::Success => {},
                        user_index_canister::check_username::Response::UsernameTaken => return UsernameTaken,
                        user_index_canister::check_username::Response::UsernameInvalid => return UsernameInvalid,
                        user_index_canister::check_username::Response::UsernameTooShort(val) => return UsernameTooShort(val),
                        user_index_canister::check_username::Response::UsernameTooLong(val) => return UsernameTooLong(val),
                    }
                },
                Err(error) => return InternalError(format!("{:?}", error)),
            }
        }
    
        mutate_state(|state| set_username_impl(jwt.noble_id, args, state))
    } else {
        PermissionDenied
    }
}

fn prepare(noble_id: NobleId, args: &Args, state: &RuntimeState) -> Result<(bool, Principal), Response> {
    let username = args.username.clone();

    match validate_username(&username) {
        Ok(_) => {}
        Err(UsernameValidationError::TooShort(min_length)) => return Err(UsernameTooShort(min_length)),
        Err(UsernameValidationError::TooLong(max_length)) => return Err(UsernameTooLong(max_length)),
        Err(UsernameValidationError::Invalid) => return Err(UsernameInvalid),
    }

    if let Some(user) = state.data.users.get(noble_id) {
        if username == user.username {
            return Err(Success);
        }

        let username_case_insensitive_changed = username.to_uppercase() != user.username.to_uppercase();

        Ok((username_case_insensitive_changed, state.data.user_index_canister_id))
    } else {
        Err(UserNotFound)
    }
}

fn set_username_impl(noble_id: NobleId, args: Args, state: &mut RuntimeState) -> Response {
    let username = args.username;
   
    if let Some(user) = state.data.users.get(noble_id) {
        let mut user_to_update = user.clone();
        user_to_update.username = username.clone();
        let now = state.env.now();

        let noble_id = user.noble_id;
        match state.data.users.update(user_to_update, now) {
            UpdateUserResult::Success => {
                state.push_event_to_user_index(UserIndexEvent::UsernameChanged(Box::new(
                    UsernameChanged { noble_id, username }
                )));
                Success
            },
            UpdateUserResult::UserNotFound => UserNotFound,
        }

    } else {
        UserNotFound
    }
}
