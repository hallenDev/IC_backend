use crate::{read_state, RuntimeState};
use ic_cdk_macros::query;
use types::{CanisterId, check_jwt};
use local_user_index_canister::user::{Response::*, *};

#[query]
async fn user(args: Args) -> Response {
    if check_jwt(&args.jwt, read_state(|state| state.env.now())).is_none() {
        return PermissionDenied;
    }

    let PrepareOk{
        canister_id
    } = match read_state(|state| prepare(&args, state)) {
        Ok(result) => result,
        Err(err) => return err,
    };

    match local_user_index_canister_c2c_client::user(canister_id, &args).await {
        Ok(result) => return result,
        Err(error) => InternalError(format!("{:?}", error)),
    }
}

struct PrepareOk{
    canister_id: CanisterId,
}

fn prepare(args: &Args, state: &RuntimeState) -> Result<PrepareOk, Response> {
    let canister_id = match state.data.users.get(args.noble_id) {
        Some(user) => user.canister_id,
        None => return Err(UserNotFound),
    };

    Ok(PrepareOk { canister_id })
}


use candid::CandidType;
use serde::Deserialize;

#[derive(CandidType, Deserialize, Debug)]
struct TestArgs {
    user_index_canister_id: CanisterId,
}

#[derive(CandidType, Deserialize, Debug)]
enum TestResponse {
    Success
}

#[query]
fn test(args: TestArgs) -> TestResponse {
    ic_cdk::println!("{:?}", args);
    TestResponse::Success
}