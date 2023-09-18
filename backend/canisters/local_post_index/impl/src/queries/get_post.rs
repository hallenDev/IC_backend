use crate::{read_state, RuntimeState};
use ic_cdk_macros::query;
use local_post_index_canister::get_post::{Response::*, *};
use types::check_jwt;

#[query]
fn get_post(args: Args) -> Response {
    read_state(|state| get_post_impl(args, state))
}

fn get_post_impl(
    args: Args,
    state: &RuntimeState
) -> Response {
    let now = state.env.now();

    if let Some(jwt) = check_jwt(&args.jwt, now) {
        if let Some(post) = state.data.posts.get(args.post_id) {
            if post.can_show(jwt.noble_id, &args.following_list, &args.block_me_users) {
                let (comments, more_exist) = post.get_sub_comments(0, jwt.noble_id, 0, args.limit, &args.block_me_users);
                Success(SuccessResult {
                    post: post.to_detail(post.liked_users.contains(&jwt.noble_id), args.bookmarks.contains(&post.post_id)),
                    comments, more_exist
                })
            } else {
                PermissionDenied
            }
        } else {
            PostNotFound
        }
    } else {
        PermissionDenied
    }
}
