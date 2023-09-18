use crate::{read_state, RuntimeState};
use ic_cdk_macros::query;
use local_user_index_canister::get_liked_posts::{Response::*, *};
use types::check_jwt;

#[query]
fn get_liked_posts(args: Args) -> Response {
    read_state(|state| get_liked_posts_impl(&args, state))
}

fn get_liked_posts_impl(args: &Args, state: &RuntimeState) -> Response {
    let now = state.env.now();

    if let Some(jwt) = check_jwt(&args.jwt, now) {
        let noble_id = args.noble_id.unwrap_or(jwt.noble_id);

        if let Some(user) = state.data.users.get(noble_id) {
            Success(user.liked_posts.iter().map(|(post_id, _)| *post_id).collect())
        } else {
            UserNotFound
        }
    } else {
        PermissionDenied
    }
}