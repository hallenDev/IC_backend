use crate::{mutate_state, RuntimeState};
use ic_cdk_macros::update;
use user_index_canister::set_password::{Response::*, *};
use types::{check_jwt, NobleId};
use argon2::Config;
use rand::Rng;

#[update]
fn set_password(args: Args) -> Response {
    mutate_state(|state| set_password_impl(args, state))
}

fn set_password_impl(args: Args, state: &mut RuntimeState) -> Response {
    if let Some(jwt) = check_jwt(&args.jwt, state.env.now()) {
        match prepare(jwt.noble_id, &args, state) {
            Ok(hash) => {
                if let Some(user) = state.data.users.get_mut(jwt.noble_id) {
                    user.password = hash;
                    Success
                } else {
                    UserNotFound
                }
            },
            Err(error) => return error,
        }
    } else {
        PermissionDenied
    }
}

fn prepare(noble_id: NobleId, args: &Args, state: &mut RuntimeState) -> Result<String, Response> {
    match state.data.users.get(noble_id) {
        Some(user) => {
            match user.verify_password(&args.password) {
                true => {
                    let mut error = ErrorResult::new();

                    if args.new_password.is_empty() {
                        error.new_password = format!("New password is required.");
                    } else if args.new_password.len() < 5 || args.new_password.len() > 20 {
                        error.new_password = format!("Password should be between 5 and 20 characters.");
                    }
                
                    if args.password_confirm.is_empty() || args.new_password != args.password_confirm {
                        error.password_confirm = format!("Password isn't matched.");
                    }
                
                    let salt: [u8; 32] = state.env.rng().gen();
                    let config = Config::default();
                    let password_hash = match argon2::hash_encoded(args.new_password.as_bytes(), &salt, &config) {
                        Ok(hash) => hash,
                        Err(_) => {
                            error.new_password = "Password hash error".to_string();
                            return Err(Error(error));
                        },
                    };

                    if error.is_error() {
                        Err(Error(error))
                    } else {
                        Ok(password_hash)
                    }
                },
                false => Err(Error(ErrorResult { password: format!("Password is incorrect."), ..Default::default() })),
            }
        },
        None => Err(UserNotFound),
    }
}
