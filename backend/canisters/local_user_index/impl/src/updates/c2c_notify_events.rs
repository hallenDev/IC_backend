use crate::guards::caller_is_user_index_canister;
use crate::{mutate_state, RuntimeState};
use canister_api_macros::update_msgpack;
use local_user_index_canister::c2c_notify_events::{Response::*, *};
use local_user_index_canister::Event;
use types::{NobleId, PostId, CommentId};

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
    match event {
        Event::UserFollowed(ev) => follow_user(ev.sender_id, ev.receiver_id, state),
        Event::UserUnfollowed(ev) => unfollow_user(ev.sender_id, ev.receiver_id, state),
        Event::UserBlocked(ev) => block_user(ev.sender_id, ev.receiver_id, state),
        Event::UserUnblocked(ev) => unblock_user(ev.sender_id, ev.receiver_id, state),
        Event::UsernameChanged(ev) => username_changed(ev.noble_id, ev.username, state),
        Event::CommentLiked(ev) => comment_liked(ev.noble_id, ev.post_id, ev.comment_id, state),
        Event::CommentUnliked(ev) => comment_unliked(ev.noble_id, ev.post_id, ev.comment_id, state),
        Event::LocalPostIndexCanisterAdded(ev) => {
            state.data.local_post_index_canister_ids.insert(ev.canister_id);
        }
    }
}

fn follow_user(sender_id: NobleId, receiver_id: NobleId, state: &mut RuntimeState) {
    if let Some(receiver) = state.data.users.get_mut(receiver_id) {
        receiver.add_follower(sender_id);
    }
}

fn unfollow_user(sender_id: NobleId, receiver_id: NobleId, state: &mut RuntimeState) {
    if let Some(receiver) = state.data.users.get_mut(receiver_id) {
        receiver.remove_follower(sender_id);
    }
}

fn block_user(sender_id: NobleId, receiver_id: NobleId, state: &mut RuntimeState) {
    if let Some(receiver) = state.data.users.get_mut(receiver_id) {
        receiver.add_block_me_user(sender_id);
    }
}

fn unblock_user(sender_id: NobleId, receiver_id: NobleId, state: &mut RuntimeState) {
    if let Some(receiver) = state.data.users.get_mut(receiver_id) {
        receiver.remove_block_me_user(sender_id);
    }
}

fn username_changed(noble_id: NobleId, username: String, state: &mut RuntimeState) {
    if let Some(user) = state.data.users.get_mut(noble_id) {
        user.username = username;
    }
}

fn comment_liked(noble_id: NobleId, post_id: PostId, comment_id: CommentId, state: &mut RuntimeState) {
    if let Some(user) = state.data.users.get_mut(noble_id) {
        user.like_post(post_id, comment_id);
    }
}

fn comment_unliked(noble_id: NobleId, post_id: PostId, comment_id: CommentId, state: &mut RuntimeState) {
    if let Some(user) = state.data.users.get_mut(noble_id) {
        user.unlike_post(post_id, comment_id);
    }
}
