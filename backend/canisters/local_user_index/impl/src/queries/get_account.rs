use crate::{read_state, RuntimeState};
use ic_cdk_macros::query;
use local_user_index_canister::get_account::{Response::*, *};
use types::check_jwt;

#[query]
fn get_account(args: Args) -> Response {
    read_state(|state| get_account_impl(args, state))
}

fn get_account_impl(args: Args, state: &RuntimeState) -> Response {
    if let Some(jwt) = check_jwt(&args.jwt, state.env.now()) {
        if let Some(user) = state.data.users.get(jwt.noble_id) {
            Success(SuccessResult{
                username: user.username.clone(),
                email: user.email.clone(),
                search_by_email: user.search_by_email,
                account_privacy: user.account_privacy,
            })
        } else {
            UserNotFound
        }
    } else {
        PermissionDenied
    }
}
