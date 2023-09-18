use crate::{read_state, RuntimeState};
use ic_cdk_macros::query;
use local_user_index_canister::get_user_data::{Response::*, *};
use types::check_jwt;

#[query]
fn get_user_data(args: Args) -> Response {
    read_state(|state| get_user_data_impl(&args, state))
}

fn get_user_data_impl(args: &Args, state: &RuntimeState) -> Response {
    let now = state.env.now();

    if let Some(jwt) = check_jwt(&args.jwt, now) {
        let noble_id = args.noble_id.unwrap_or(jwt.noble_id);

        if let Some(user) = state.data.users.get(noble_id) {
            let mut block_me_users = vec![];
            if args.mask & 1 != 0 {
                block_me_users = user.block_me_users.clone();
            }
            let mut following_list = vec![];
            if args.mask & 2 != 0 {
                following_list = user.following_list.iter().map(|item| item.noble_id).collect();
            }
            let mut bookmarks = vec![];
            if args.mask & 4 != 0 {
                bookmarks = user.bookmarks.clone();
            }
            let mut liked_posts = vec![];
            if args.mask & 8 != 0 {
                liked_posts = user.liked_posts.iter().map(|(post_id, _)| *post_id).collect();
            }

            Success(SuccessResult{
                block_me_users,
                following_list,
                bookmarks,
                liked_posts,
            })
        } else {
            UserNotFound
        }
    } else {
        PermissionDenied
    }
}