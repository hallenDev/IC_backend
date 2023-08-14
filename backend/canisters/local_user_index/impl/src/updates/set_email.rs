use crate::model::user_map::UpdateUserResult;
use crate::{mutate_state, RuntimeState, read_state};
use candid::Principal;
use ic_cdk_macros::update;
use local_user_index_canister::set_email::{Response::*, *};
use types::{check_jwt, NobleId};
use user_index_canister::{Event as UserIndexEvent, EmailChanged};

#[update]
async fn set_email(args: Args) -> Response {
    if let Some(jwt) = check_jwt(&args.jwt, read_state(|state| state.env.now())) {
        let user_index_canister_id = match read_state(|state| prepare(jwt.noble_id, &args, state)) {
            Ok(ok) => ok,
            Err(error) => return error,
        };

        match user_index_canister_c2c_client::check_email(
            user_index_canister_id,
            &user_index_canister::check_email::Args{email: args.email.clone()}
        ).await {
            Ok(response) => {
                match response {
                    user_index_canister::check_email::Response::Success => {},
                    user_index_canister::check_email::Response::EmailAlreadyExist => return EmailAlreadyExist,
                }
            },
            Err(error) => return InternalError(format!("{:?}", error)),
        }
    
        mutate_state(|state| set_email_impl(jwt.noble_id, args, state))
    } else {
        PermissionDenied
    }
}

fn prepare(noble_id: NobleId, args: &Args, state: &RuntimeState) -> Result<Principal, Response>{
    if !email_address::EmailAddress::is_valid(&args.email) {
        return Err(EmailIsInvalid);
    }
    if let Some(user) = state.data.users.get(noble_id) {
        if user.email == args.email {
            return Err(Success);
        }
    }
    Ok(state.data.user_index_canister_id)
}

fn set_email_impl(noble_id: NobleId, args: Args, state: &mut RuntimeState) -> Response {
    let email = args.email;

    if let Some(user) = state.data.users.get(noble_id) {
        let mut user_to_update = user.clone();
        user_to_update.email = email.clone();
        let now = state.env.now();

        let noble_id = user.noble_id;
        match state.data.users.update(user_to_update, now) {
            UpdateUserResult::Success => {
                state.push_event_to_user_index(UserIndexEvent::EmailChanged(Box::new(
                    EmailChanged { noble_id, email }
                )));
                Success
            },
            UpdateUserResult::UserNotFound => UserNotFound,
        }

    } else {
        UserNotFound
    }
}