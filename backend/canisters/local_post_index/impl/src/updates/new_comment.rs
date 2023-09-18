use crate::{mutate_state, RuntimeState, read_state, MAX_COMMENT_LENGTH};
use ic_cdk_macros::update;
use local_post_index_canister::new_comment::{Response::*, *};
use types::{check_jwt, NobleId, TimestampMillis};
use utils::field_validation::validate_field_value;
use post_index_canister::{Event as PostIndexEvent, NewComment};

#[update]
fn new_comment(args: Args) -> Response {
    let now = read_state(|state| state.env.now());
    if let Some(jwt) = check_jwt(&args.jwt, now) {
        mutate_state(|state| new_comment_impl(jwt.noble_id, args, state, now))
    } else {
        PermissionDenied
    }
}

fn new_comment_impl(noble_id: NobleId, args: Args, state: &mut RuntimeState, now: TimestampMillis) -> Response {
    if let Some(post) = state.data.posts.get_mut(args.post_id) {
        if let Err(err) = validate_field_value("Description", true, MAX_COMMENT_LENGTH, &args.description, utils::field_validation::FieldType::Text) {
            return Error(ErrorResult { description: err });
        }
        if post.comments.len() > args.comment_id as usize {
            if let Some(comment_id) = post.add_comment(noble_id, args.comment_id, args.description, now) {
                state.push_event_to_post_index(PostIndexEvent::NewComment(Box::new(NewComment{
                    noble_id,
                    post_id: args.post_id,
                    date_create: now
                })));
                Success(SuccessResult { comment_id, parent_comment_id: args.comment_id })
            } else {
                CommentNotFound
            }
        } else {
            CommentNotFound
        }
    } else {
        PostNotFound
    }
}
