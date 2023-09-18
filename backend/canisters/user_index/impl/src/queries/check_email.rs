use crate::{read_state, RuntimeState};
use ic_cdk_macros::query;
use user_index_canister::check_email::{Response::*, *};

#[query]
fn check_email(args: Args) -> Response {
    read_state(|state| check_email_impl(args, state))
}

fn check_email_impl(args: Args, state: &RuntimeState) -> Response {
    if !email_address::EmailAddress::is_valid(&args.email) {
        return EmailIsInvalid;
    }

    if state.data.users.does_email_exist(&args.email) {
        return EmailTaken;
    }

    if state.data.temps.does_email_exist(&args.email) {
        return EmailTaken;
    }

    Success
}
