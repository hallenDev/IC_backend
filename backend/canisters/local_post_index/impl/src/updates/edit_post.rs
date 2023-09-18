use crate::{mutate_state, RuntimeState, MAX_TITLE_LENGTH, MAX_DESCRIPTION_LENGTH};
use ic_cdk_macros::update;
use types::check_jwt;
use local_post_index_canister::edit_post::{Response::*, *};
use utils::{field_validation::validate_field_value, truncate_string::truncate_string};
use post_index_canister::{Event as PostIndexEvent, PostEdited};

#[update]
fn edit_post(args: Args) -> Response {
    mutate_state(|state| edit_post_impl(args, state))
}

fn edit_post_impl(args: Args, state: &mut RuntimeState) -> Response {
    if let Some(jwt) = check_jwt(&args.jwt, state.env.now()) {
        match prepare(&args) {
            Ok(()) => {},
            Err(response) => return response,
        };
        if let Some(post) = state.data.posts.get_mut(args.post_id) {
            if post.noble_id == jwt.noble_id {
                post.title = args.title.clone();
                post.description = args.description.clone();
                post.post_privacy = args.post_privacy;
                post.invited_users = args.invited_users.clone();

                state.push_event_to_post_index(PostIndexEvent::PostEdited(Box::new(PostEdited {
                    post_id: args.post_id,
                    title: args.title,
                    description: truncate_string(args.description, 100),
                    post_privacy: args.post_privacy,
                    invited_users: args.invited_users,
                })));

                Success
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

fn prepare(args: &Args) -> Result<(), Response> {
    let mut error = ErrorResult::default();

    if let Err(err) = validate_field_value("Title", true, MAX_TITLE_LENGTH, &args.title, utils::field_validation::FieldType::Text) {
        error.title = err;
    }

    if let Err(err) = validate_field_value("Description", true, MAX_DESCRIPTION_LENGTH, &args.description, utils::field_validation::FieldType::Text) {
        error.description = err;
    }

    if error.is_error() {
        return Err(Error(error));
    }

    Ok(())
}
