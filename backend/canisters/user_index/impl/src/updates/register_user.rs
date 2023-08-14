use crate::{mutate_state, RuntimeState};
use argon2::Config;
use candid::Principal;
use ic_cdk_macros::update;
use rand::Rng;
use types::{CanisterId, NobleId};
use user_index_canister::register_user::{Response::*, *};
use utils::username_validation::{validate_username, UsernameValidationError};

#[update]
async fn register_user(args: Args) -> Response {
    let RegisterOk {
        principal,
        canister_id,
        noble_id,
        username,
        password_hash,
    } = match mutate_state(|state| register_user_impl(&args, state)) {
        Ok(ok) => ok,
        Err(response) => return response,
    };
    match commit_local_index(principal, canister_id, noble_id, args.email.clone(), username.clone(), password_hash).await {
        Ok(()) => {
            mutate_state(|state| {
                state.data.users.register(principal, noble_id, args.email, username, canister_id, state.env.now());
                state.data.local_index_map.add_user(canister_id, noble_id);
            });
            Success(SuccessResult { canister_id })
        },
        Err(err) => {
            InternalError(err)
        },
    }
}

struct RegisterOk {
    principal: Principal,
    canister_id: CanisterId,
    noble_id: NobleId,
    username: String,
    password_hash: String,
}

fn register_user_impl(args: &Args, state: &mut RuntimeState) -> Result<RegisterOk, Response> {
    // Check the principal is derived from Internet Identity + check the username is valid
    let PrepareOk {
        caller,
        canister_id,
    } = match prepare(args, state) {
        Ok(ok) => ok,
        Err(response) => return Err(response),
    };

    let salt: [u8; 32] = state.env.rng().gen();
    let config = Config::default();
    let password_hash = match argon2::hash_encoded(args.password.as_bytes(), &salt, &config) {
        Ok(password) => password,
        Err(_) => return Err(PasswordHashError),
    };

    let mut noble_id = state.env.rng().gen_range(1000000000u64..10000000000u64);
    while state.data.users.get(noble_id).is_some() {
        noble_id = state.env.rng().gen_range(1000000000u64..10000000000u64);
    }

    Ok(RegisterOk{
        principal: caller,
        canister_id,
        noble_id,
        username: args.username.clone(),
        password_hash,
    })
}

struct PrepareOk {
    caller: Principal,
    canister_id: CanisterId,
}

fn prepare(args: &Args, state: &mut RuntimeState) -> Result<PrepareOk, Response> {
    let caller = state.env.caller();

    if caller != Principal::anonymous() && state.data.users.get_by_principal(&caller).is_some() {
        return Err(AlreadyRegistered);
    }

    if state.data.users.get_by_email(&args.email).is_some() {
        return Err(AlreadyRegistered);
    }

    let canister_id = match state.data.local_index_map.index_for_new_user() {
        Some(index) => index,
        None => return Err(UserLimitReached),
    };

    match validate_username(&args.username) {
        Ok(_) => {}
        Err(UsernameValidationError::TooShort(min_length)) => return Err(UsernameTooShort(min_length)),
        Err(UsernameValidationError::TooLong(max_length)) => return Err(UsernameTooLong(max_length)),
        Err(UsernameValidationError::Invalid) => return Err(UsernameInvalid),
    };

    if state.data.users.does_username_exist(&args.username) {
        return Err(UsernameAlreayExist);
    }

    if !email_address::EmailAddress::is_valid(&args.email) {
        return Err(EmailIsInvalid);
    }

    if args.password.is_empty() {
        return Err(PasswordIsRequired);
    } else if args.password.len() < 5 || args.password.len() > 20 {
        return Err(PasswordLengthIsInvalid(5, 20));
    }

    if args.password_confirm.is_empty() || args.password != args.password_confirm {
        return Err(PasswordIsnotMatch);
    }

    Ok(PrepareOk {
        caller,
        canister_id,
    })
}

async fn commit_local_index(
    principal: Principal,
    canister_id: CanisterId,
    noble_id: NobleId,
    email: String,
    username: String,
    password_hash: String,
) -> Result<(), String> {

    match local_user_index_canister_c2c_client::register_user(
        canister_id,
        &local_user_index_canister::register_user::Args {
            caller: principal,
            noble_id,
            email,
            username,
            password_hash,
        }).await
    {
        Ok(response) => match response {
            local_user_index_canister::register_user::Response::Success => Ok(()),
            local_user_index_canister::register_user::Response::AlreadyRegistered => Err("AlreadyRegistered".to_string()),
            local_user_index_canister::register_user::Response::UserLimitReached => Err("UserLimitReached".to_string()),
        },
        Err(error) => Err(format!("{error:?}")),
    }
}
