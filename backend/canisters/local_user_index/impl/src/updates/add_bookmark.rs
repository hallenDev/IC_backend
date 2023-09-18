use crate::{mutate_state, RuntimeState, read_state};
use ic_cdk_macros::update;
use local_user_index_canister::add_bookmark::{Response::*, *};
use types::{check_jwt, NobleId};

#[update]
async fn add_bookmark(args: Args) -> Response {
    if let Some(jwt) = check_jwt(&args.jwt, read_state(|state| state.env.now())) {
        let post_index_canister_id = read_state(|state| state.data.post_index_canister_id);

        match post_index_canister_c2c_client::c2c_is_nobleblocks_post(
            post_index_canister_id,
            &post_index_canister::c2c_is_nobleblocks_post::Args{post_id: args.post_id}
        ).await {
            Ok(response) => {
                match response {
                    post_index_canister::c2c_is_nobleblocks_post::Response::Yes => {},
                    post_index_canister::c2c_is_nobleblocks_post::Response::No => return PostNotFound,
                }
            },
            Err(error) => return InternalError(format!("{:?}", error)),
        }
    
        mutate_state(|state| add_bookmark_impl(jwt.noble_id, args, state))
    } else {
        PermissionDenied
    }
}

fn add_bookmark_impl(noble_id: NobleId, args: Args, state: &mut RuntimeState) -> Response {
    if let Some(user) = state.data.users.get_mut(noble_id) {
        if user.is_bookmarked(args.post_id) {
            AlreadyBookmarked
        } else {
            user.add_bookmark(args.post_id);
            Success
        }
    } else {
        UserNotFound
    }
}
