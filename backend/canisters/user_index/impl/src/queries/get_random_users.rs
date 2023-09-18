use crate::model::user::User;
use crate::{mutate_state, RuntimeState};
use ic_cdk_macros::query;
use rand::seq::IteratorRandom;
use user_index_canister::get_random_users::{Response::*, *};

#[query]
fn get_random_users(_args: Args) -> Response {
    mutate_state(|state| get_random_users_impl(state))
}

fn get_random_users_impl(
    state: &mut RuntimeState
) -> Response {
    let results: Vec<&User> = state.data.users.iter().filter(|item| item.avatar_id != 0).choose_multiple(state.env.rng(), 5);
    Success(results.iter().map(|item| item.get_user_info()).collect())
}
