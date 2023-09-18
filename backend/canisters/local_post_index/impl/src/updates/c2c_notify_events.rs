use crate::guards::caller_is_post_index_canister;
use crate::{mutate_state, RuntimeState};
use canister_api_macros::update_msgpack;
use local_post_index_canister::c2c_notify_events::{Response::*, *};
use local_post_index_canister::Event;

#[update_msgpack(guard = "caller_is_post_index_canister")]
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
        Event::LocalUserIndexCanisterAdded(ev) => {
            state.data.local_user_index_canister_ids.insert(ev.canister_id);
        },
    }
}
