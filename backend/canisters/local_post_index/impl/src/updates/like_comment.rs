use crate::{mutate_state, RuntimeState};
use ic_cdk_macros::update;
use types::check_jwt;
use local_post_index_canister::like_comment::{Response::*, *};
use post_index_canister::{Event as PostIndexEvent, PostLiked};
use user_index_canister::{Event as UserIndexEvent, CommentLiked};

#[update]
fn like_comment(args: Args) -> Response {
    mutate_state(|state| like_comment_impl(args, state))
}

fn like_comment_impl(args: Args, state: &mut RuntimeState) -> Response {
    if let Some(jwt) = check_jwt(&args.jwt, state.env.now()) {
        if let Some(post) = state.data.posts.get_mut(args.post_id) {
            if args.comment_id == 0 {
                if post.liked_users.insert(jwt.noble_id) {
                    state.push_event_to_post_index(PostIndexEvent::PostLiked(Box::new(PostLiked { noble_id: jwt.noble_id, post_id: args.post_id })));
                    state.push_event_to_user_index(UserIndexEvent::CommentLiked(Box::new(CommentLiked { noble_id: jwt.noble_id, post_id: args.post_id, comment_id: 0 })));
                    Success
                } else {
                    AlreadyLiked
                }
            } else {
                if let Some(comment) = post.comments.get_mut(args.comment_id as usize) {
                    if comment.liked_users.insert(jwt.noble_id) {
                        state.push_event_to_user_index(UserIndexEvent::CommentLiked(Box::new(CommentLiked { noble_id: jwt.noble_id, post_id: args.post_id, comment_id: args.comment_id })));
                        Success
                    } else {
                        AlreadyLiked
                    }
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
