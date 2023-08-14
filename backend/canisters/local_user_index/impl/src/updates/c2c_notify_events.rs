use crate::guards::caller_is_user_index_canister;
use crate::{mutate_state, RuntimeState};
use canister_api_macros::update_msgpack;
use local_user_index_canister::c2c_notify_events::{Response::*, *};
use local_user_index_canister::Event;
use types::NobleId;

#[update_msgpack(guard = "caller_is_user_index_canister")]
fn c2c_notify_events(args: Args) -> Response {
    mutate_state(|state| c2c_notify_user_index_events_impl(args, state))
}

fn c2c_notify_user_index_events_impl(args: Args, state: &mut RuntimeState) -> Response {
    for event in args.events {
        handle_event(event, state);
    }
    Success
}

fn handle_event(event: Event, state: &mut RuntimeState) {
    ic_cdk::println!("{:?}", event);
    match event {
        Event::UserFollowed(ev) => follow_user(ev.sender_id, ev.receiver_id, state),
        Event::UserUnfollowed(ev) => unfollow_user(ev.sender_id, ev.receiver_id, state),
        Event::UserBlocked(ev) => block_user(ev.sender_id, ev.receiver_id, state),
        Event::UserUnblocked(ev) => unblock_user(ev.sender_id, ev.receiver_id, state),
    }
}

fn follow_user(sender_id: NobleId, receiver_id: NobleId, state: &mut RuntimeState) {
    if let Some(receiver) = state.data.users.get(receiver_id) {
        let mut receiver_to_update = receiver.clone();
        receiver_to_update.add_follower(sender_id);
        let now = state.env.now();

        state.data.users.update(receiver_to_update, now);
    }
}

fn unfollow_user(sender_id: NobleId, receiver_id: NobleId, state: &mut RuntimeState) {
    if let Some(receiver) = state.data.users.get(receiver_id) {
        let mut receiver_to_update = receiver.clone();
        receiver_to_update.remove_follower(sender_id);
        let now = state.env.now();

        state.data.users.update(receiver_to_update, now);
    }
}

fn block_user(sender_id: NobleId, receiver_id: NobleId, state: &mut RuntimeState) {
    if let Some(receiver) = state.data.users.get(receiver_id) {
        let mut receiver_to_update = receiver.clone();
        receiver_to_update.add_block_me_user(sender_id);
        let now = state.env.now();

        state.data.users.update(receiver_to_update, now);
    }
}

fn unblock_user(sender_id: NobleId, receiver_id: NobleId, state: &mut RuntimeState) {
    if let Some(receiver) = state.data.users.get(receiver_id) {
        let mut receiver_to_update = receiver.clone();
        receiver_to_update.remove_block_me_user(sender_id);
        let now = state.env.now();

        state.data.users.update(receiver_to_update, now);
    }
}

