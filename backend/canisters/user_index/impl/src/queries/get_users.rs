use crate::model::user::User;
use crate::{read_state, RuntimeState};
use ic_cdk_macros::query;
use user_index_canister::get_users::{Response::*, *};
use types::{NobleId, check_jwt};

#[query]
fn get_users(args: Args) -> Response {
    read_state(|state| get_users_impl(args, state))
}

fn get_users_impl(
    args: Args,
    state: &RuntimeState
) -> Response {
    let now = state.env.now();

    let noble_id = check_jwt(&args.jwt, now).unwrap_or_default().noble_id;

    let mut users: Vec<&User> = state.data.users.iter().filter(|item| is_filtered(item, noble_id, &args.block_me_users)).collect();

    users.sort_unstable_by(|lhs, rhs| {
        rhs.date_created.cmp(&lhs.date_created)
    });

    let total_users_count = users.len() as u32;

    let results = users.iter()
        .skip(((args.page - 1) * args.limit ) as usize)
        .take(args.limit as usize)
        .map(|item| item.to_summary(args.following_list.contains(&item.noble_id)))
        .collect();

    Success(SuccessResult {
        total_users_count,
        users: results,
        timestamp: now,
    })
}

fn is_filtered(user: &User, _noble_id: NobleId, block_me_users: &Vec<NobleId>) -> bool {
    if block_me_users.contains(&user.noble_id) {
        return false;
    }
    if user.username.is_empty() {
        return false;
    }
    true
}
