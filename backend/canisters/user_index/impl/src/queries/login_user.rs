use crate::{read_state, RuntimeState, model::user::User};
use ic_cdk_macros::query;
use user_index_canister::login_user::{Response::*, *};
use types::TimestampMillis;

#[query]
fn login_user(args: Args) -> Response {
    read_state(|state| login_user_impl(args, state))
}

fn login_user_impl(args: Args, state: &RuntimeState) -> Response {
    if let Some(user) = state.data.users.get_by_email(&args.email) {
        verify_user(args, user, state.env.now())
    } else if let Some(user) = state.data.users.get_by_username(&args.email) {
        verify_user(args, user, state.env.now())
    } else {
        EmailOrPasswordIncorrect
    }
}

fn verify_user(args: Args, user: &User, now: TimestampMillis) -> Response {
    match user.verify_password(&args.password) {
        true => {
            match user.get_login_info(now) {
                Ok(ok) => Success(ok),
                Err(error) => InternalError(error),
            }
        },
        false => EmailOrPasswordIncorrect,
    }
}
