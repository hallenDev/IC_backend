use crate::{mutate_state, RuntimeState};
use ic_cdk_macros::update;
use types::check_jwt;
use local_post_index_canister::delete_comment::{Response::*, *};
use post_index_canister::{Event as PostIndexEvent, PostDeleted, CommentDeleted};

#[update]
fn delete_comment(args: Args) -> Response {
    mutate_state(|state| delete_comment_impl(args, state))
}

fn delete_comment_impl(args: Args, state: &mut RuntimeState) -> Response {
    let super_admin = state.caller_is_super_admin();
    if let Some(jwt) = check_jwt(&args.jwt, state.env.now()) {
        if let Some(post) = state.data.posts.get_mut(args.post_id) {
            if args.comment_id == 0 {
                if post.noble_id == jwt.noble_id || super_admin {
                    state.data.posts.remove_post(args.post_id);
                    state.push_event_to_post_index(PostIndexEvent::PostDeleted(Box::new(PostDeleted {
                        post_id: args.post_id,
                    })));
    
                    Success(SuccessResult { post_id: args.post_id, comment_id: args.comment_id })
                } else {
                    PermissionDenied
                }
            } else {
                if let Some(comment) = post.comments.get(args.comment_id as usize) {
                    if comment.noble_id == jwt.noble_id || super_admin {
                        if post.remove_comment(args.comment_id) {
                            let comments_count = post.comments.get(0).unwrap().comments_count;
                            state.push_event_to_post_index(PostIndexEvent::CommentDeleted(Box::new(CommentDeleted {
                                post_id: args.post_id,
                                comments_count,
                            })));
                            Success(SuccessResult { post_id: args.post_id, comment_id: args.comment_id })
                        } else {
                            CommentNotFound
                        }
                    } else {
                        PermissionDenied
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
