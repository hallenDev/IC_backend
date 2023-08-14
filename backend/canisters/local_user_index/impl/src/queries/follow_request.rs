use crate::{mutate_state, RuntimeState, read_state};
use ic_cdk_macros::update;
use local_user_index_canister::follow_request::{Response::*, *};
use types::{check_jwt, NobleId};
use user_index_canister::{Event as UserIndexEvent, FollowRequest};

#[update]
async fn follow_request(args: Args) -> Response {
    if let Some(jwt) = check_jwt(&args.jwt, read_state(|state| state.env.now())) {
        let user_index_canister_id = read_state(|state| state.data.user_index_canister_id);

        match user_index_canister_c2c_client::c2c_is_nobleblocks_user(
            user_index_canister_id,
            &user_index_canister::c2c_is_nobleblocks_user::Args{noble_id: args.noble_id}
        ).await {
            Ok(response) => {
                match response {
                    user_index_canister::c2c_is_nobleblocks_user::Response::Yes => {},
                    user_index_canister::c2c_is_nobleblocks_user::Response::No => return UserNotFound,
                }
            },
            Err(error) => return InternalError(format!("{:?}", error)),
        }
    
        mutate_state(|state| follow_request_impl(jwt.noble_id, args, state))
    } else {
        PermissionDenied
    }
}

fn follow_request_impl(noble_id: NobleId, args: Args, state: &mut RuntimeState) -> Response {
    if let Some(sender) = state.data.users.get(noble_id) {
        let sender_id = sender.noble_id;
        let receiver_id = args.noble_id;

        if sender_id == receiver_id {
            return UserNotFound;
        }

        if !sender.is_following(receiver_id) {
            return UnfollowState;
        }

        state.push_event_to_user_index(UserIndexEvent::FollowRequest(Box::new(
            FollowRequest { sender_id, receiver_id }
        )));
        Success
    } else {
        UserNotFound
    }
}
