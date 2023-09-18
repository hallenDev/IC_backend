use canister_agent_utils::{get_dfx_identity, CanisterName};
use canister_upgrader::*;
use clap::Parser;
use types::{CanisterId, Version};

#[tokio::main]
async fn main() {
    let opts = Opts::parse();

    let identity = get_dfx_identity(&opts.controller);

    match opts.canister_to_upgrade {
        CanisterName::UserIndex => upgrade_user_index_canister(identity, opts.url, opts.user_index, opts.version).await,
        CanisterName::LocalUserIndex => {
            upgrade_local_user_index_canister(identity, opts.url, opts.user_index, opts.version).await
        },
        CanisterName::PostIndex => upgrade_post_index_canister(identity, opts.url, opts.post_index, opts.version).await,
        CanisterName::LocalPostIndex => {
            upgrade_local_post_index_canister(identity, opts.url, opts.post_index, opts.version).await
        },
    };
}

#[derive(Parser)]
struct Opts {
    #[arg(long)]
    url: String,

    #[arg(long)]
    controller: String,

    #[arg(long)]
    user_index: CanisterId,

    #[arg(long)]
    post_index: CanisterId,

    #[arg(long)]
    canister_to_upgrade: CanisterName,

    #[arg(long)]
    version: Version,
}
