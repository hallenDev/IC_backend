use crate::{mutate_state, RuntimeState, INFO_EMAIL};
use crate::model::temp::TempData;
use ic_cdk_macros::update;
use user_index_canister::{EmailEvent, RegisterUser};
use user_index_canister::register_user::{Response::*, *};
use utils::username_validation::{validate_username, UsernameValidationError};

#[update]
fn register_user(args: Args) -> Response {
    mutate_state(|state| register_user_impl(args, state))
}

fn register_user_impl(args: Args, state: &mut RuntimeState) -> Response {
    let now = state.env.now();
    state.data.temps.remove_expired_temp(now);

    if state.data.local_index_map.index_for_new_user().is_none() {
        return UserLimitReached;
    }

    if let Some((temp_id, passkey)) = state.data.temps.does_exist(&args.email, &args.username, state.env.rng(), now) {
        state.push_event_to_send_email(
            INFO_EMAIL,
            EmailEvent::RegisterUser(Box::new(RegisterUser { email: args.email, name: args.username, passkey }))
        );
        return Success(temp_id);
    }

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

    if state.data.temps.does_username_exist(&args.username) {
        error.username = format!("Username already exists.");
    }

    if !email_address::EmailAddress::is_valid(&args.email) {
        error.email = format!("Email is invalid.");
    }

    if state.data.users.does_email_exist(&args.email) {
        error.email = format!("Email already exists.");
    }

    if state.data.temps.does_email_exist(&args.email) {
        error.email = format!("Email already exists.");
    }

    if args.password.is_empty() {
        error.password = format!("Password is required.");
    } else if args.password.len() < 5 || args.password.len() > 20 {
        error.password = format!("Password should be between 5 and 20 characters.");
    }

    if args.password_confirm.is_empty() || args.password != args.password_confirm {
        error.password_confirm = format!("Password isn't matched.");
    }

    if error.is_error() {
        return Error(error);
    }

    let email = args.email.clone();
    let name = args.username.clone();

    let (temp_id, passkey) = state.data.temps.add_new_temp(
        args.email.clone(),
        TempData::RegisterUser(args),
        state.env.rng(),
        now
    );
    state.push_event_to_send_email(
        INFO_EMAIL,
        EmailEvent::RegisterUser(Box::new(RegisterUser { email, name, passkey }))
    );
    Success(temp_id)
}

