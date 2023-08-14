use crate::{mutate_state, RuntimeState, POST_LIMIT};
use crate::guards::caller_is_post_index_canister;
use ic_cdk_macros::update;
use local_post_index_canister::new_post::{Response::*, *};

#[update(guard = "caller_is_post_index_canister")]
fn new_post(args: Args) -> Response {
    mutate_state(|state| new_post_impl(args, state))
}

fn new_post_impl(args: Args, state: &mut RuntimeState) -> Response {
    if state.data.posts.len() >= POST_LIMIT {
        return PostLimitReached;
    }

    state
        .data
        .posts
        .add_post(
            args.post_id,
            args.owner,
            args.title,
            args.description,
            args.category,
            args.link_url,
            args.video_url,
            args.attached_file_id,
            args.post_privacy,
            args.invited_users,
            args.date_created,
        );

    Success
}
