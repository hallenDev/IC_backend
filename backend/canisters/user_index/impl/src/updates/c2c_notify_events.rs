use crate::guards::caller_is_local_user_index_canister;
use crate::model::follow_request_map::FollowRequest;
use crate::{mutate_state, RuntimeState};
use canister_api_macros::update_msgpack;
use types::NobleId;
use local_user_index_canister::{Event as LocalUserIndexEvent, FollowUser, BlockUser};
use user_index_canister::c2c_notify_events::{Response::*, *};
use user_index_canister::Event;

#[update_msgpack(guard = "caller_is_local_user_index_canister")]
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
        Event::EmailChanged(ev) => set_email(ev.noble_id, ev.email, state),
        Event::AccountDeleted(ev) => remove_user(ev.noble_id, state),
        Event::SearchByEmailChanged(ev) => set_search_by_email(ev.noble_id, ev.search_by_email, state),
        Event::NameChanged(ev) => set_name(ev.noble_id, ev.first_name, ev.last_name, state),
        Event::LocationChanged(ev) => set_location(ev.noble_id, ev.country, ev.city, state),
        Event::BioChanged(ev) => set_bio(ev.noble_id, ev.bio, state),
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
    }
}

fn set_username(noble_id: NobleId, username: String, state: &mut RuntimeState) {
    if let Some(user) = state.data.users.get(noble_id) {
        let mut user_to_update = user.clone();
        user_to_update.username = username;
        state.data.users.update(user_to_update);
    }
}

fn set_email(noble_id: NobleId, email: String, state: &mut RuntimeState) {
    if let Some(user) = state.data.users.get(noble_id) {
        let mut user_to_update = user.clone();
        user_to_update.email = email;
        state.data.users.update(user_to_update);
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

fn set_search_by_email(noble_id: NobleId, search_by_email: bool, state: &mut RuntimeState) {
    if let Some(user) = state.data.users.get(noble_id) {
        let mut user_to_update = user.clone();
        user_to_update.search_by_email = search_by_email;
        state.data.users.update(user_to_update);
    }
}

fn set_name(noble_id: NobleId, first_name: String, last_name: String, state: &mut RuntimeState) {
    if let Some(user) = state.data.users.get(noble_id) {
        let mut user_to_update = user.clone();
        user_to_update.first_name = first_name;
        user_to_update.last_name = last_name;
        state.data.users.update(user_to_update);
    }
}

fn set_location(noble_id: NobleId, country: String, city: String, state: &mut RuntimeState) {
    if let Some(user) = state.data.users.get(noble_id) {
        let mut user_to_update = user.clone();
        user_to_update.country = country;
        user_to_update.city = city;
        state.data.users.update(user_to_update);
    }
}

fn set_bio(noble_id: NobleId, bio: String, state: &mut RuntimeState) {
    if let Some(user) = state.data.users.get(noble_id) {
        let mut user_to_update = user.clone();
        user_to_update.bio = bio;
        state.data.users.update(user_to_update);
    }
}