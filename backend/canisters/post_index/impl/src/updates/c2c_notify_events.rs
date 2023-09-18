use std::collections::HashSet;

use crate::guards::caller_is_known_canister;
use crate::{mutate_state, RuntimeState};
use canister_api_macros::update_msgpack;
use local_post_index_canister::{Event as LocalPostIndexEvent, LocalUserIndexCanisterAdded};
use types::{NobleId, PostId, TimestampMillis, PostPrivacy, CanisterId};
use post_index_canister::c2c_notify_events::{Response::*, *};
use post_index_canister::Event;

#[update_msgpack(guard = "caller_is_known_canister")]
fn c2c_notify_events(args: Args) -> Response {
    mutate_state(|state| c2c_notify_events_impl(args, state))
}

fn c2c_notify_events_impl(args: Args, state: &mut RuntimeState) -> Response {
    for event in args.events {
        handle_event(event, state);
    }

    Success
}

fn handle_event(event: Event, state: &mut RuntimeState) {
    match event {
        Event::NewComment(ev) => new_comment(ev.noble_id, ev.post_id, ev.date_create, state),
        Event::CommentDeleted(ev) => delete_comment(ev.post_id, ev.comments_count, state),
        Event::PostLiked(ev) => post_liked(ev.post_id, state),
        Event::PostUnliked(ev) => post_unliked(ev.post_id, state),
        Event::PostEdited(ev) => post_edited(ev.post_id, ev.title, ev.description, ev.post_privacy, ev.invited_users, state),
        Event::PostDeleted(ev) => post_deleted(ev.post_id, state),
        Event::LocalUserIndexAdded(ev) => add_local_user_index_canister_id(ev.canister_id, state),
    }
}

fn new_comment(
    noble_id: NobleId,
    post_id: PostId,
    now: TimestampMillis,
    state: &mut RuntimeState,
) {
    if let Some(post) = state.data.posts.get_mut(post_id) {
        post.date_last_commented = now;
        post.comments_count += 1;
        if !post.contributed_users.contains(&noble_id) {
            if post.contributed_users.len() < 2 {
                post.contributed_users.push(noble_id);
            } else {
                post.contributed_users.remove(2);
                post.contributed_users.push(noble_id);
            }
        } else {
            if let Some(index) = post.contributed_users.iter().position(|item| *item == noble_id) {
                if index >= 2 {
                    post.contributed_users.remove(index);
                    post.contributed_users.push(noble_id);
                }
            }
        }
    }
}

fn delete_comment(
    post_id: PostId,
    comments_count: u32,
    state: &mut RuntimeState,
) {
    if let Some(post) = state.data.posts.get_mut(post_id) {
        post.comments_count = comments_count;
    }
}

fn post_liked(post_id: PostId, state: &mut RuntimeState) {
    if let Some(post) = state.data.posts.get_mut(post_id) {
        post.liked_users_count += 1;
    }
}

fn post_unliked(post_id: PostId, state: &mut RuntimeState) {
    if let Some(post) = state.data.posts.get_mut(post_id) {
        post.liked_users_count -= 1;
    }
}

fn post_edited(
    post_id: PostId,
    title: String,
    description: String,
    post_privacy: PostPrivacy,
    invited_users: HashSet<NobleId>,
    state: &mut RuntimeState,
) {
    if let Some(post) = state.data.posts.get_mut(post_id) {
        post.title = title;
        post.description = description;
        post.post_privacy = post_privacy;
        post.invited_users = invited_users;
    }
}

fn post_deleted(post_id: PostId, state: &mut RuntimeState) {
    if let Some(post) = state.data.posts.get(post_id) {
        state.data.local_index_map.remove_post(post.canister_id, post_id);
        state.data.posts.remove_post(post_id);
    }
}

fn add_local_user_index_canister_id(canister_id: CanisterId, state: &mut RuntimeState) {
    state.data.local_user_index_canister_ids.insert(canister_id);
    state.push_event_to_all_local_post_index(LocalPostIndexEvent::LocalUserIndexCanisterAdded(Box::new(LocalUserIndexCanisterAdded{
        canister_id
    })));
}