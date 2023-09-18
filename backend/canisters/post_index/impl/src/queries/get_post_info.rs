use crate::{read_state, RuntimeState};
use ic_cdk_macros::query;
use post_index_canister::get_post_info::{Response::*, *};

#[query]
fn get_post_info(args: Args) -> Response {
    // if check_jwt(&args.jwt, read_state(|state| state.env.now())).is_some() {
        read_state(|state| get_post_info_impl(args, state))
    // } else {
    //     PermissionDenied
    // }
}

fn get_post_info_impl(
    args: Args,
    state: &RuntimeState
) -> Response {
    if let Some(post) = state.data.posts.get(args.post_id) {
        Success(SuccessResult { post_id: post.post_id, local_post_canister_id: post.canister_id })
    } else {
        PermissionDenied
    }
}
