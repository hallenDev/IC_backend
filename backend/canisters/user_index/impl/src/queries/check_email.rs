use crate::{read_state, RuntimeState};
use ic_cdk_macros::query;
use user_index_canister::check_email::{Response::*, *};

#[query]
fn check_email(args: Args) -> Response {
    read_state(|state| check_email_impl(args, state))
}

fn check_email_impl(args: Args, state: &RuntimeState) -> Response {
    if state.data.users.get_by_email(&args.email).is_some() {
        EmailAlreadyExist
    } else {
        Success
    }
}
