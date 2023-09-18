use crate::{read_state, RuntimeState};
use ic_cdk_macros::query;
use local_post_index_canister::get_comments::{Response::*, *};
use types::check_jwt;

#[query]
fn get_comments(args: Args) -> Response {
    read_state(|state| get_comments_impl(args, state))
}

fn get_comments_impl(
    args: Args,
    state: &RuntimeState
) -> Response {
    let now = state.env.now();

    if let Some(jwt) = check_jwt(&args.jwt, now) {
        if let Some(post) = state.data.posts.get(args.post_id) {
            if post.can_show(jwt.noble_id, &args.following_list, &args.block_me_users) {
                let (comments, more_exist) = post.get_sub_comments(args.comment_id, jwt.noble_id, args.from - 1, args.limit, &args.block_me_users);
                Success(ScucessResult { comments, more_exist })
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
