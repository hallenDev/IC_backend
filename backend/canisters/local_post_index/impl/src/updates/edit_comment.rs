use crate::{mutate_state, RuntimeState, read_state, MAX_COMMENT_LENGTH};
use ic_cdk_macros::update;
use local_post_index_canister::edit_comment::{Response::*, *};
use types::{check_jwt, NobleId};
use utils::field_validation::validate_field_value;

#[update]
fn edit_comment(args: Args) -> Response {
    let now = read_state(|state| state.env.now());
    if let Some(jwt) = check_jwt(&args.jwt, now) {
        mutate_state(|state| edit_comment_impl(jwt.noble_id, args, state))
    } else {
        PermissionDenied
    }
}

fn edit_comment_impl(noble_id: NobleId, args: Args, state: &mut RuntimeState) -> Response {
    if let Some(post) = state.data.posts.get_mut(args.post_id) {
        if let Err(err) = validate_field_value("Description", true, MAX_COMMENT_LENGTH, &args.description, utils::field_validation::FieldType::Text) {
            return Error(ErrorResult { description: err });
        }
        if post.comments.len() > args.comment_id as usize {
            if post.edit_comment(noble_id, args.comment_id, args.description) {
                Success
            } else {
                PermissionDenied
            }
        } else {
            CommentNotFound
        }
    } else {
        PostNotFound
    }
}
