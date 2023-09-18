use crate::{read_state, RuntimeState};
use ic_cdk_macros::query;
use user_index_canister::get_user_infos::{Response::*, *};

#[query]
fn get_user_infos(args: Args) -> Response {
    read_state(|state| get_user_infos_impl(args, state))
}

fn get_user_infos_impl(args: Args, state: &RuntimeState) -> Response {
    // if check_jwt(&args.jwt, state.env.now()).is_some() {
        let mut result = Vec::with_capacity(args.noble_ids.len());

        args.noble_ids.iter().for_each(|noble_id| {
            if let Some(user) = state.data.users.get(*noble_id) {
                result.push(user.get_user_info());
            }
        });
    
        Success(result)
    // } else {
    //     PermissionDenied
    // }
}

