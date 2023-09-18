use crate::{RuntimeState, mutate_state, read_state};
use candid::Principal;
use ic_cdk_macros::update;
use user_index_canister::login_user_with_internet_identity::{Response::*, *};
use types::{CanisterId, NobleId, TimestampMillis};

#[update]
async fn login_user_with_internet_identity(args: Args) -> Response {
    let caller = ic_cdk::caller();
    if caller == Principal::anonymous() {
        return InvalidInternetIdentity;
    }

    if read_state(|state| is_new_user(&caller, state)) {
        let (canister_id, noble_id, username, now) = match mutate_state(prepare) {
            Ok(ok) => ok,
            Err(err) => return err,
        };
        match register_user(caller, canister_id, noble_id, username.clone()).await {
            Ok(()) => {
                mutate_state(|state| {
                    state.data.users.register(caller, noble_id, String::new(), username, String::new(), canister_id, now);
                    state.data.local_index_map.add_user(canister_id, noble_id);
                });
            },
            Err(err) => return err,
        }
    }

    mutate_state(|state| login_user_with_internet_identity_impl(&caller, args, state))
}

fn is_new_user(caller: &Principal, state: &RuntimeState) -> bool {
    state.data.users.get_by_principal(caller).is_none()
}

fn prepare(state: &mut RuntimeState) -> Result<(Principal, NobleId, String, TimestampMillis), Response> {
    let canister_id = match state.data.local_index_map.index_for_new_user() {
        Some(index) => index,
        None => return Err(UserLimitReached),
    };

    let noble_id = state.data.users.new_noble_id(state.env.rng());

    Ok((canister_id, noble_id, state.data.get_anonymous_username(), state.env.now()))
}

async fn register_user(caller: Principal, canister_id: CanisterId, noble_id: NobleId, username: String) -> Result<(), Response> {
    match local_user_index_canister_c2c_client::register_user_with_internet_identity(
        canister_id,
        &local_user_index_canister::register_user_with_internet_identity::Args {
            caller,
            noble_id,
            canister_id,
            username,
        }).await
    {
        Ok(response) => match response {
            local_user_index_canister::register_user_with_internet_identity::Response::Success => return Ok(()),
            local_user_index_canister::register_user_with_internet_identity::Response::UserLimitReached => return Err(UserLimitReached),
        },
        Err(error) => return Err(InternalError(format!("{:?}", error))),
    }
}

fn login_user_with_internet_identity_impl(caller: &Principal, _: Args, state: &mut RuntimeState) -> Response {
    if let Some(user) = state.data.users.get_by_principal(&caller) {
        match user.get_login_info(state.env.now()) {
            Ok(ok) => Success(ok),
            Err(error) => InternalError(error),
        }
    } else {
        InternalError(format!("Something went wrong"))
    }
}
