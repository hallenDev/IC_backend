use crate::{mutate_state, RuntimeState, read_state};
use candid::Principal;
use ic_cdk_macros::update;
use local_user_index_canister::set_account::{Response::*, *};
use user_index_canister::{Event as UserIndexEvent, AccountChanged};
use types::{check_jwt, NobleId};
use utils::username_validation::{validate_username, UsernameValidationError};

#[update]
async fn set_account(args: Args) -> Response {
    if let Some(jwt) = check_jwt(&args.jwt, read_state(|state| state.env.now())) {
        let (username_case_insensitive_changed, email_changed, user_index_canister_id) = match read_state(|state| prepare(jwt.noble_id, &args, state)) {
            Ok(ok) => ok,
            Err(error) => return error,
        };

        let mut error = ErrorResult::new();

        if username_case_insensitive_changed {
            match user_index_canister_c2c_client::check_username(
                user_index_canister_id,
                &user_index_canister::check_username::Args{username: args.username.clone()}
            ).await {
                Ok(response) => {
                    match response {
                        user_index_canister::check_username::Response::Success => {},
                        user_index_canister::check_username::Response::UsernameTaken => error.username = format!("Username already exists."),
                        user_index_canister::check_username::Response::UsernameInvalid => error.username = format!("Username is invalid."),
                        user_index_canister::check_username::Response::UsernameTooShort(val) => error.username = format!("Username should be at least {} characters.", val),
                        user_index_canister::check_username::Response::UsernameTooLong(val) => error.username = format!("Username should be less than {} characters.", val),
                    }
                },
                Err(error) => return InternalError(format!("{:?}", error)),
            }
        }

        if email_changed {
            match user_index_canister_c2c_client::check_email(
                user_index_canister_id,
                &user_index_canister::check_email::Args{email: args.email.clone()}
            ).await {
                Ok(response) => {
                    match response {
                        user_index_canister::check_email::Response::Success => {},
                        user_index_canister::check_email::Response::EmailTaken => error.email = format!("Email already exists."),
                        user_index_canister::check_email::Response::EmailIsInvalid => error.email = format!("Email is invalid."),
                    }
                },
                Err(error) => return InternalError(format!("{:?}", error)),
            }
        }

        if error.is_error() {
            return Error(error);
        }
    
        mutate_state(|state| set_account_impl(jwt.noble_id, args, state))
    } else {
        PermissionDenied
    }
}

fn prepare(noble_id: NobleId, args: &Args, state: &RuntimeState) -> Result<(bool, bool, Principal), Response> {
    let username = &args.username;

    let mut error = ErrorResult::new();

    match validate_username(username) {
        Ok(_) => {}
        Err(UsernameValidationError::TooShort(min_length)) => error.username = format!("Username should be at least {} characters.", min_length),
        Err(UsernameValidationError::TooLong(max_length)) => error.username = format!("Username should be less than {} characters.", max_length),
        Err(UsernameValidationError::Invalid) => error.username = format!("Username is invalid."),
    };

    if !email_address::EmailAddress::is_valid(&args.email) {
        error.email = format!("Email is invalid.");
    }

    if error.is_error() {
        return Err(Error(error));
    }

    if let Some(user) = state.data.users.get(noble_id) {
        let username_case_insensitive_changed = username.to_uppercase() != user.username.to_uppercase();

        let email_changed = args.email != user.email;

        Ok((username_case_insensitive_changed, email_changed, state.data.user_index_canister_id))
    } else {
        Err(UserNotFound)
    }
}

fn set_account_impl(noble_id: NobleId, args: Args, state: &mut RuntimeState) -> Response {
    if let Some(user) = state.data.users.get_mut(noble_id) {
        user.username = args.username.clone();
        user.email = args.email.clone();
        user.search_by_email = args.search_by_email;
        user.account_privacy = args.account_privacy;
        state.push_event_to_user_index(UserIndexEvent::AccountChanged(Box::new(
            AccountChanged {
                noble_id,
                username: args.username,
                email: args.email,
                search_by_email: args.search_by_email,
            }
        )));
        Success
    } else {
        UserNotFound
    }
}
