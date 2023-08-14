use crate::{read_state, RuntimeState};
use candid::Principal;
use ic_cdk_macros::update;
use user_index_canister::login_user::{Response::*, *};
use types::{CanisterId, NobleId, JWT};

#[update]
async fn login_user(args: Args) -> Response {
    let caller = read_state(get_data);

    if caller == Principal::anonymous() {
        let (noble_id, canister_id) = match read_state(|state| prepare(&args.email, state)) {
            Ok(ok) => ok,
            Err(respose) => return respose,
        };

        match local_user_index_canister_c2c_client::check_password(canister_id,
            &local_user_index_canister::check_password::Args{
                noble_id,
                password: args.password
            }
        ).await {
            Ok(res) => {
                println!("{:?}", res);
                match res {
                    local_user_index_canister::check_password::Response::Success => {
                        read_state(|state| login_with_email(&args.email, state))
                    },
                    _ => EmailOrPasswordIncorrect,
                }
            },
            Err(error) => InternalError(format!("{:?}", error))
        }
    } else {
        read_state(|state| login_with_principal(&caller, state))
    }
}

fn login_with_principal(caller: &Principal, state: &RuntimeState) -> Response {
    if let Some(user) = state.data.users.get_by_principal(caller) {
        let jwt = JWT::new(user.noble_id, user.email.clone(), user.username.clone(), state.env.now());

        let jwt = match jwt.to_string() {
            Some(j) => j,
            None => return InternalError("".to_string()),
        };
        Success(SuccessResult{
            canister_id: user.canister_id,
            jwt,
        })
    } else {
        UnregisteredUser
    }
}

fn prepare(email: &str, state: &RuntimeState) -> Result<(NobleId, CanisterId), Response> {
    if let Some(user) = state.data.users.get_by_email(email) {
        Ok((user.noble_id, user.canister_id))
    } else {
        Err(EmailOrPasswordIncorrect)
    }
}

fn login_with_email(email: &str, state: &RuntimeState) -> Response {
    if let Some(user) = state.data.users.get_by_email(email) {
        let jwt = JWT::new(user.noble_id, user.email.clone(), user.username.clone(), state.env.now());

        let jwt = match jwt.to_string() {
            Some(j) => j,
            None => return InternalError("".to_string()),
        };
        Success(SuccessResult{
            canister_id: user.canister_id,
            jwt,
        })
    } else {
        UnregisteredUser
    }
}

fn get_data(state: &RuntimeState) -> Principal {
    state.env.caller()
}