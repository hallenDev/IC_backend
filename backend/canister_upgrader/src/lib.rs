use candid::CandidType;
use canister_agent_utils::{build_ic_agent, get_canister_wasm, CanisterName};
use ic_agent::identity::Secp256k1Identity;
use ic_utils::call::AsyncCall;
use ic_utils::interfaces::management_canister::builders::InstallMode;
use ic_utils::interfaces::management_canister::CanisterStatus;
use ic_utils::interfaces::ManagementCanister;
use types::{CanisterId, CanisterWasm, UpgradeCanisterWasmArgs, Version};

pub async fn upgrade_user_index_canister(
    identity: Secp256k1Identity,
    url: String,
    user_index_canister_id: CanisterId,
    version: Version,
) {
    upgrade_top_level_canister(
        identity,
        url,
        user_index_canister_id,
        version,
        user_index_canister::post_upgrade::Args { wasm_version: version },
        CanisterName::UserIndex,
    )
    .await;

    println!("User index canister upgraded");
}

pub async fn upgrade_post_index_canister(
    identity: Secp256k1Identity,
    url: String,
    post_index_canister_id: CanisterId,
    version: Version,
) {
    upgrade_top_level_canister(
        identity,
        url,
        post_index_canister_id,
        version,
        post_index_canister::post_upgrade::Args { wasm_version: version },
        CanisterName::PostIndex,
    )
    .await;

    println!("Post index canister upgraded");
}

pub async fn upgrade_local_user_index_canister(
    identity: Secp256k1Identity,
    url: String,
    user_index_canister_id: CanisterId,
    version: Version,
) {
    let agent = build_ic_agent(url, identity).await;
    let canister_wasm = get_canister_wasm(CanisterName::LocalUserIndex, version);
    let args = UpgradeCanisterWasmArgs {
        wasm: CanisterWasm {
            version,
            module: canister_wasm.module,
        },
        filter: None,
        use_for_new_canisters: None,
    };

    let response = user_index_canister_client::upgrade_local_user_index_canister_wasm(&agent, &user_index_canister_id, &args)
        .await
        .unwrap();

    if !matches!(
        response,
        user_index_canister::upgrade_local_user_index_canister_wasm::Response::Success
    ) {
        panic!("{response:?}");
    }
    println!("Local user index canister wasm upgraded to version {version}");
}

pub async fn upgrade_local_post_index_canister(
    identity: Secp256k1Identity,
    url: String,
    post_index_canister_id: CanisterId,
    version: Version,
) {
    let agent = build_ic_agent(url, identity).await;
    let canister_wasm = get_canister_wasm(CanisterName::LocalPostIndex, version);
    let args = UpgradeCanisterWasmArgs {
        wasm: CanisterWasm {
            version,
            module: canister_wasm.module,
        },
        filter: None,
        use_for_new_canisters: None,
    };

    let response = post_index_canister_client::upgrade_local_post_index_canister_wasm(&agent, &post_index_canister_id, &args)
        .await
        .unwrap();

    if !matches!(
        response,
        post_index_canister::upgrade_local_post_index_canister_wasm::Response::Success
    ) {
        panic!("{response:?}");
    }
    println!("Local post index canister wasm upgraded to version {version}");
}

async fn upgrade_top_level_canister<A: CandidType + Send + Sync>(
    identity: Secp256k1Identity,
    url: String,
    canister_id: CanisterId,
    version: Version,
    args: A,
    canister_name: CanisterName,
) {
    let agent = build_ic_agent(url, identity).await;
    let management_canister = ManagementCanister::create(&agent);
    let canister_wasm = get_canister_wasm(canister_name, version);

    upgrade_wasm(&management_canister, &canister_id, &canister_wasm.module, args).await;
}

async fn upgrade_wasm<A: CandidType + Send + Sync>(
    management_canister: &ManagementCanister<'_>,
    canister_id: &CanisterId,
    wasm_bytes: &[u8],
    args: A,
) {
    println!("Stopping canister {canister_id}");
    management_canister
        .stop_canister(canister_id)
        .call_and_wait()
        .await
        .expect("Failed to stop canister");

    loop {
        let (canister_status,) = management_canister
            .canister_status(canister_id)
            .call_and_wait()
            .await
            .expect("Failed to call 'canister_status'");

        if canister_status.status == CanisterStatus::Stopped {
            break;
        }
        println!("Waiting for canister to stop");
    }
    println!("Canister stopped");

    println!("Upgrading wasm for canister {canister_id}");
    match management_canister
        .install_code(canister_id, wasm_bytes)
        .with_mode(InstallMode::Upgrade)
        .with_arg(args)
        .call_and_wait()
        .await
    {
        Ok(_) => println!("Wasm upgraded"),
        Err(error) => println!("Upgrade failed: {error:?}"),
    };

    println!("Starting canister {canister_id}");
    management_canister
        .start_canister(canister_id)
        .call_and_wait()
        .await
        .expect("Failed to start canister");
    println!("Canister started");
}
