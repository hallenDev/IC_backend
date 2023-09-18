use crate::{read_state, RuntimeState};
use ic_cdk_macros::query;
use local_post_index_canister::get_like_users::{Response::*, *};
use types::check_jwt;

#[query]
fn get_like_users(args: Args) -> Response {
    read_state(|state| get_like_users_impl(&args, state))
}

fn get_like_users_impl(args: &Args, state: &RuntimeState) -> Response {
    let now = state.env.now();

    if let Some(_jwt) = check_jwt(&args.jwt, now) {
        if let Some(post) = state.data.posts.get(args.post_id) {
            if args.comment_id == 0 {
                Success(post.liked_users.clone().into_iter().collect())
            } else {
                if let Some(comment) = post.comments.get(args.comment_id as usize) {
                    Success(comment.liked_users.clone().into_iter().collect())
                } else {
                    CommentNotFound
                }
            }
        } else {
            PostNotFound
        }
    } else {
        PermissionDenied
    }
}