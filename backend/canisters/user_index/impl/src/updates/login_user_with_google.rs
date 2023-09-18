use crate::{RuntimeState, mutate_state, read_state};
use candid::Principal;
use ic_cdk_macros::update;
use user_index_canister::login_user_with_google::{Response::*, *};
use types::{CanisterId, NobleId, TimestampMillis};
use utils::username_validation::{validate_username, UsernameValidationError};

#[update]
async fn login_user_with_google(args: Args) -> Response {
    let caller = ic_cdk::caller();

    if read_state(|state| is_new_user(&args.email, state)) {
        let (canister_id, noble_id, username, now) = match mutate_state(|state| prepare(&args, state)) {
            Ok(ok) => ok,
            Err(err) => return err,
        };
        match register_user(caller, canister_id, noble_id, args.email.clone(), username.clone()).await {
            Ok(()) => {
                mutate_state(|state| {
                    state.data.users.register(caller, noble_id, args.email.clone(), username, String::new(), canister_id, now);
                    state.data.local_index_map.add_user(canister_id, noble_id);
                });
                return mutate_state(|state| login_user_with_google_impl(&args.email, state))
            },
            Err(err) => return err,
        }
    }

    mutate_state(|state| login_user_with_google_impl(&args.email, state))
}

fn is_new_user(email: &str, state: &RuntimeState) -> bool {
    state.data.users.get_by_email(email).is_none()
}

fn prepare(args: &Args, state: &mut RuntimeState) -> Result<(Principal, NobleId, String, TimestampMillis), Response> {
    let canister_id = match state.data.local_index_map.index_for_new_user() {
        Some(index) => index,
        None => return Err(UserLimitReached),
    };

    let noble_id = state.data.users.new_noble_id(state.env.rng());

    let mut username = format!("{}{}", args.first_name, args.last_name);

    match validate_username(&username) {
        Ok(_) => {}
        Err(UsernameValidationError::TooShort(_)) => username.clear(),
        Err(UsernameValidationError::TooLong(_)) => username.clear(),
        Err(UsernameValidationError::Invalid) => username.clear(),
    };

    if state.data.users.does_username_exist(&username) {
        username.clear();
    }

    if state.data.temps.does_username_exist(&username) {
        username.clear();
    }

    if username.is_empty() {
        username = state.data.get_anonymous_username();
    }

    Ok((canister_id, noble_id, username, state.env.now()))
}

async fn register_user(
    caller: Principal,
    canister_id: CanisterId,
    noble_id: NobleId,
    email: String,
    username: String,
) -> Result<(), Response> {
    match local_user_index_canister_c2c_client::register_user_with_google(
        canister_id,
        &local_user_index_canister::register_user_with_google::Args {
            caller,
            noble_id,
            canister_id,
            email: email.clone(),
            username: username.clone(),
        }).await
    {
        Ok(response) => match response {
            local_user_index_canister::register_user_with_google::Response::Success => Ok(()),
            local_user_index_canister::register_user_with_google::Response::UserLimitReached => return Err(UserLimitReached),
        },
        Err(error) => return Err(InternalError(format!("{:?}", error))),
    }
}

fn login_user_with_google_impl(email: &str, state: &mut RuntimeState) -> Response {
    if let Some(user) = state.data.users.get_by_email(email) {
        match user.get_login_info(state.env.now()) {
            Ok(ok) => {
                if user.username.is_empty() {
                    UsernameRequire(UsernameRequireResult { jwt: ok.jwt })
                } else {
                    Success(ok)
                }
            },
            Err(error) => InternalError(error),
        }
    } else {
        InternalError(format!("Something went wrong"))
    }
}
