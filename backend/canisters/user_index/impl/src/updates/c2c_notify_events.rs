use crate::guards::caller_is_known_canister;
use crate::model::follow_request_map::FollowRequest;
use crate::{mutate_state, RuntimeState};
use canister_api_macros::update_msgpack;
use types::{NobleId, Country, AcademicDegree, AvatarId, CanisterId};
use local_user_index_canister::{Event as LocalUserIndexEvent, FollowUser, BlockUser, CommentLiked, CommentUnliked, LocalPostIndexCanisterAdded};
use user_index_canister::c2c_notify_events::{Response::*, *};
use user_index_canister::Event;

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
        Event::UsernameChanged(ev) => set_username(ev.noble_id, ev.username, state),
        Event::AccountDeleted(ev) => remove_user(ev.noble_id, state),
        Event::UserFollowed(ev) => {
            state.push_event_to_local_user_index(ev.receiver_id, LocalUserIndexEvent::UserFollowed(Box::new(
                FollowUser{sender_id: ev.sender_id, receiver_id: ev.receiver_id}
            )));
        },
        Event::UserUnfollowed(ev) => {
            state.push_event_to_local_user_index(ev.receiver_id, LocalUserIndexEvent::UserUnfollowed(Box::new(
                FollowUser{sender_id: ev.sender_id, receiver_id: ev.receiver_id}
            )));
        },
        Event::UserBlocked(ev) => {
            state.push_event_to_local_user_index(ev.receiver_id, LocalUserIndexEvent::UserBlocked(Box::new(
                BlockUser{sender_id: ev.sender_id, receiver_id: ev.receiver_id}
            )));
        },
        Event::UserUnblocked(ev) => {
            state.push_event_to_local_user_index(ev.receiver_id, LocalUserIndexEvent::UserUnblocked(Box::new(
                BlockUser{sender_id: ev.sender_id, receiver_id: ev.receiver_id}
            )));
        },
        Event::FollowRequest(ev) => follow_request(ev.sender_id, ev.receiver_id, state),
        Event::ProfileChanged(ev) => set_profile(ev.noble_id, ev.first_name, ev.last_name, ev.degree, ev.country, ev.city, ev.bio, ev.avatar_id, state),
        Event::AccountChanged(ev) => set_account(ev.noble_id, ev.username, ev.email, ev.search_by_email, state),
        Event::PhotoChanged(ev) => set_photo(ev.noble_id, ev.avatar_id, state),
        Event::CommentLiked(ev) => {
            state.push_event_to_local_user_index(ev.noble_id, LocalUserIndexEvent::CommentLiked(Box::new(
                CommentLiked { noble_id: ev.noble_id, post_id: ev.post_id, comment_id: ev.comment_id }
            )));
        },
        Event::CommentUnliked(ev) => {
            state.push_event_to_local_user_index(ev.noble_id, LocalUserIndexEvent::CommentUnliked(Box::new(
                CommentUnliked { noble_id: ev.noble_id, post_id: ev.post_id, comment_id: ev.comment_id }
            )));
        },
        Event::LocalPostIndexAdded(ev) => add_local_post_index_canister(ev.canister_id, state),
    }
}

fn set_username(noble_id: NobleId, username: String, state: &mut RuntimeState) {
    if let Some(user) = state.data.users.get_mut(noble_id) {
        let prev_username = user.username.clone();
        if prev_username != username {
            user.username = username.clone();
        }
        if prev_username.to_uppercase() != username.to_uppercase() {
            state.data.users.update_username(prev_username, username, noble_id);
        }
    }
}

fn remove_user(noble_id: NobleId, state: &mut RuntimeState) {
    if let Some(user) = state.data.users.get(noble_id) {
        state.data.local_index_map.remove_user(user.canister_id, noble_id);
        state.data.users.remove(noble_id);
    }
}

fn follow_request(sender_id: NobleId, receiver_id: NobleId, state: &mut RuntimeState) {
    if state.data.users.get(receiver_id).is_some() {
        let request = FollowRequest {
            sender: sender_id,
            receiver: receiver_id,
            timestamp: state.env.now(),
        };

        state.data.follow_requests.add_request(&request);
    }
}

fn set_profile(
    noble_id: NobleId,
    first_name: String,
    last_name: String,
    degree: Option<AcademicDegree>,
    country: Option<Country>,
    city: String,
    bio: String,
    avatar_id: AvatarId,
    state: &mut RuntimeState,
) {
    if let Some(user) = state.data.users.get_mut(noble_id) {
        user.noble_id = noble_id;
        user.first_name = first_name;
        user.last_name = last_name;
        user.degree = degree;
        user.country = country;
        user.city = city;
        user.bio = bio;
        user.avatar_id = avatar_id;
    }
}

fn set_account(
    noble_id: NobleId,
    username: String,
    email: String,
    search_by_email: bool,
    state: &mut RuntimeState,
) {
    if let Some(user) = state.data.users.get_mut(noble_id) {
        let prev_username = user.username.clone();
        let prev_email = user.email.clone();
        user.username = username.clone();
        user.email = email.clone();
        user.search_by_email = search_by_email;

        if prev_username.to_uppercase() != username.to_uppercase() {
            state.data.users.update_username(prev_username, username, noble_id);
        }
        if prev_email != email{
            state.data.users.update_email(prev_email, email, noble_id);
        }
    }
}

fn set_photo(
    noble_id: NobleId,
    avatar_id: AvatarId,
    state: &mut RuntimeState,
) {
    if let Some(user) = state.data.users.get_mut(noble_id) {
        user.noble_id = noble_id;
        user.avatar_id = avatar_id;
    }
}

fn add_local_post_index_canister(canister_id: CanisterId, state: &mut RuntimeState) {
    state.data.local_post_index_canister_ids.insert(canister_id);
    state.push_event_to_all_local_user_index(LocalUserIndexEvent::LocalPostIndexCanisterAdded(Box::new(LocalPostIndexCanisterAdded{
        canister_id,
    })));
}