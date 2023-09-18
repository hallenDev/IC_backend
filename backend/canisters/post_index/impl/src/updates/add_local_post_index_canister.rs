use crate::guards::caller_is_governance_principal;
use crate::{mutate_state, RuntimeState, LOCAL_POST_INDEX_CANISTER_INITIAL_CYCLES_BALANCE};
use canister_api_macros::proposal;
use local_post_index_canister::init::Args as InitLocalPostIndexCanisterArgs;
use types::{CanisterId, CanisterWasm, Cycles, Version};
use post_index_canister::add_local_post_index_canister::{Response::*, *};
use utils::canister;
use utils::consts::{CREATE_CANISTER_CYCLES_FEE, MIN_CYCLES_BALANCE};
use user_index_canister::{Event as UserIndexEvent, LocalPostIndexAdded};

#[proposal(guard = "caller_is_governance_principal")]
async fn add_local_post_index_canister(args: Args) -> Response {
    let PrepareOk {
        canister_id,
        canister_wasm,
        cycles_to_use,
        init_canister_args,
    } = match mutate_state(|state| prepare(&args, state)) {
        Ok(ok) => ok,
        Err(response) => return response,
    };

    let wasm_version = canister_wasm.version;

    match canister::create_and_install(
        canister_id,
        canister_wasm,
        init_canister_args,
        cycles_to_use,
        on_canister_created,
    )
    .await
    {
        Ok(canister_id) => {
            mutate_state(|state| commit(canister_id, wasm_version, state));
            Success
        }
        Err(error) => InternalError(format!("{error:?}"))
    }
}

struct PrepareOk {
    canister_id: Option<CanisterId>,
    canister_wasm: CanisterWasm,
    cycles_to_use: Cycles,
    init_canister_args: InitLocalPostIndexCanisterArgs,
}


fn prepare(args: &Args, state: &mut RuntimeState) -> Result<PrepareOk, Response> {
    let cycles_to_use = if args.canister_id.is_none() {
        let cycles_required = LOCAL_POST_INDEX_CANISTER_INITIAL_CYCLES_BALANCE + CREATE_CANISTER_CYCLES_FEE;
        if !utils::cycles::can_spend_cycles(cycles_required, MIN_CYCLES_BALANCE) {
            return Err(CyclesBalanceTooLow);
        }
        cycles_required
    } else {
        if state.data.local_index_map.contains_key(&args.canister_id.unwrap()) {
            return Err(AlreadyAdded);
        }
        0
    };

    let canister_id = args.canister_id;
    let canister_wasm = state.data.local_post_index_canister_wasm_for_new_canisters.clone();
    let init_canister_args = InitLocalPostIndexCanisterArgs {
        post_index_canister_id: state.env.canister_id(),
        user_index_canister_id: state.data.user_index_canister_id,
        local_user_index_canister_ids: state.data.local_user_index_canister_ids.clone(),
        super_admin: state.data.super_admin,
        wasm_version: canister_wasm.version,
    };

    Ok(PrepareOk {
        canister_id,
        canister_wasm,
        cycles_to_use,
        init_canister_args,
    })
}

fn on_canister_created(cycles: Cycles) {
    mutate_state(|state| state.data.total_cycles_spent_on_canisters += cycles);
}

fn commit(canister_id: CanisterId, wasm_version: Version, state: &mut RuntimeState) {
    state.data.local_index_map.add_index(canister_id, wasm_version);
    state.push_event_to_user_index(UserIndexEvent::LocalPostIndexAdded(Box::new(LocalPostIndexAdded{
        canister_id,
    })));
}