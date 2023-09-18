use crate::{mutate_state, RuntimeState};
use ic_cdk_macros::update;
use types::check_jwt;
use user_index_canister::set_username::{Response::*, *};
use utils::username_validation::{validate_username, UsernameValidationError};
use local_user_index_canister::{Event as LocalUserIndexEvent, UsernameChanged};

#[update]
fn set_username(args: Args) -> Response {
    mutate_state(|state| set_username_impl(&args, state))
}

fn set_username_impl(args: &Args, state: &mut RuntimeState) -> Response {
    if let Some(jwt) = check_jwt(&args.jwt, state.env.now()) {
        match prepare(args, state) {
            Ok(()) => {},
            Err(response) => return response,
        };

        if let Some(user) = state.data.users.get_mut(jwt.noble_id) {
            let noble_id = jwt.noble_id;

            let prev_username = user.username.clone();
            if prev_username != args.username {
                user.username = args.username.clone();
            }
            if prev_username.to_uppercase() != args.username.to_uppercase() {
                state.data.users.update_username(prev_username, args.username.clone(), noble_id);
            }

            state.push_event_to_local_user_index(noble_id, LocalUserIndexEvent::UsernameChanged(Box::new(
                UsernameChanged{noble_id, username: args.username.clone()}
            )));

            let user = state.data.users.get(jwt.noble_id).unwrap();

            match user.get_login_info(state.env.now()) {
                Ok(ok) => return Success(ok),
                Err(error) => return InternalError(error),
            }
        } else {
            UserNotFound
        }
    } else {
        PermissionDenied
    }
}

fn prepare(args: &Args, state: &mut RuntimeState) -> Result<(), Response> {
    let mut error = ErrorResult::new();

    match validate_username(&args.username) {
        Ok(_) => {}
        Err(UsernameValidationError::TooShort(min_length)) => error.username = format!("Username should be at least {} characters.", min_length),
        Err(UsernameValidationError::TooLong(max_length)) => error.username = format!("Username should be less than {} characters.", max_length),
        Err(UsernameValidationError::Invalid) => error.username = format!("Username is invalid."),
    };

    if state.data.users.does_username_exist(&args.username) {
        error.username = format!("Username already exists.");
    }

    if error.is_error() {
        return Err(Error(error));
    }
    Ok(())
}
