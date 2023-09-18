use crate::{mutate_state, RuntimeState, model::temp::{TempData, TempDataType}, read_state, INFO_EMAIL};
use argon2::Config;
use candid::Principal;
use ic_cdk::api::management_canister::provisional::CanisterId;
use ic_cdk_macros::update;
use rand::Rng;
use types::NobleId;
use user_index_canister::{verify_code::{Response::*, *}, ResetPassword};
use user_index_canister::register_user::Args as RegisterUserArgs;

#[update]
async fn verify_code(args: Args) -> Response {
    match read_state(|state| get_type(&args, state)) {
        Some(_type) => {
            match _type {
                TempDataType::ResetPassword => mutate_state(|state| reset_password(&args, state)),
                TempDataType::RegisterUser => register_user(args).await,
            }
        },
        None => TempNotExist
    }
}

fn get_type(args: &Args, state: &RuntimeState) -> Option<TempDataType> {
    match state.data.temps.get(args.id) {
        Some(temp) => {
            match temp.temp_data {
                TempData::RegisterUser(_) => Some(TempDataType::RegisterUser),
                TempData::ResetPassword(_) => Some(TempDataType::ResetPassword),
            }
        },
        None => None,
    }
}

fn reset_password(args: &Args, state: &mut RuntimeState) -> Response {
    let now = state.env.now();
    state.data.temps.remove_expired_temp(now);

    if let Some(temp) = state.data.temps.get_mut(args.id) {
        if temp.is_used {
            return InvalidPasskey;
        }
        temp.is_used = true;
        if temp.passkey == args.passkey {
            match &temp.temp_data {
                TempData::ResetPassword(data) => {
                    if let Some(user) = state.data.users.get_mut_by_email(&temp.email) {
                        let salt: [u8; 32] = state.env.rng().gen();
                        let config = Config::default();
                        let password = data.password.clone();

                        let password_hash = match argon2::hash_encoded(data.password.as_bytes(), &salt, &config) {
                            Ok(password) => password,
                            Err(_) => return InternalError(format!("Unexpected error.")),
                        };

                        user.password = password_hash;

                        match user.get_login_info(now) {
                            Ok(ok) => {
                                let email = user.email.clone();
                                let name = user.username.clone();
                                state.data.temps.remove(args.id);
                                state.push_event_to_send_email(INFO_EMAIL, user_index_canister::EmailEvent::ResetPassword(Box::new(ResetPassword{
                                    email,
                                    name,
                                    password,
                                })));
        
                                return Success(ok);
                            },
                            Err(error) => return InternalError(error),
                        }
                    }
                },
                _ => return InternalError(format!("Unexpected error.")),
            }
        }
    }
    InvalidPasskey
}

async fn register_user(args: Args) -> Response {
    let RegisterOk {
        principal,
        canister_id,
        noble_id,
        password_hash,
        register_user_args,
    } = match mutate_state(|state| register_user_impl(&args, state)) {
        Ok(ok) => ok,
        Err(response) => return response,
    };
    match commit_local_index(
        principal,
        canister_id,
        noble_id,
        register_user_args.email.clone(),
        register_user_args.username.clone(),
    ).await {
        Ok(()) => {
            mutate_state(|state| {
                state.data.users.register(
                    principal,
                    noble_id,
                    register_user_args.email,
                    register_user_args.username,
                    password_hash,
                    canister_id,
                    state.env.now()
                );
                state.data.local_index_map.add_user(canister_id, noble_id);

                if let Some(user) = state.data.users.get(noble_id) {
                    match user.get_login_info(state.env.now()) {
                        Ok(ok) => {
                            state.data.temps.remove(args.id);
                            Success(ok)
                        },
                        Err(error) => InternalError(error),
                    }
                } else {
                    InternalError(format!("Unexpected error."))
                }
            })
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
    password_hash: String,
    register_user_args: RegisterUserArgs,
}

fn register_user_impl(args: &Args, state: &mut RuntimeState) -> Result<RegisterOk, Response> {
    state.data.temps.remove_expired_temp(state.env.now());

    if let Some(temp) = state.data.temps.get_mut(args.id) {
        if temp.is_used {
            return Err(InvalidPasskey);
        }
        temp.is_used = true;
        if temp.passkey == args.passkey {
            let caller = state.env.caller();

            let canister_id = match state.data.local_index_map.index_for_new_user() {
                Some(index) => index,
                None => return Err(InternalError(format!("User limit reached."))),
            };
            let register_user_args = match &temp.temp_data {
                TempData::RegisterUser(args) => args.clone(),
                _ => return Err(InternalError(format!("Unexpected error."))),
            };

            let salt: [u8; 32] = state.env.rng().gen();
            let config = Config::default();
            let password_hash = match argon2::hash_encoded(register_user_args.password.as_bytes(), &salt, &config) {
                Ok(password) => password,
                Err(_) => return Err(InternalError(format!("Password hash error."))),
            };
        
            let noble_id = state.data.users.new_noble_id(state.env.rng());

            return Ok(RegisterOk{
                principal: caller,
                canister_id,
                noble_id,
                password_hash,
                register_user_args,
            })
        }
    }
    Err(TempNotExist)
}

async fn commit_local_index(
    principal: Principal,
    canister_id: CanisterId,
    noble_id: NobleId,
    email: String,
    username: String,
) -> Result<(), String> {

    match local_user_index_canister_c2c_client::register_user(
        canister_id,
        &local_user_index_canister::register_user::Args {
            caller: principal,
            noble_id,
            canister_id,
            email,
            username,
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
