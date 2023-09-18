use crate::{read_state, RuntimeState};
use canister_api_macros::query_msgpack;
use post_index_canister::c2c_is_nobleblocks_post::{Response::*, *};

#[query_msgpack]
fn c2c_is_nobleblocks_post(args: Args) -> Response {
    read_state(|state| c2c_is_nobleblocks_post_impl(args, state))
}

fn c2c_is_nobleblocks_post_impl(args: Args, state: &RuntimeState) -> Response {
    if state.data.posts.get(args.post_id).is_some() {
        return Yes;
    }
    No
}

