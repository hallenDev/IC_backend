# NobleBlocks

NobleBlocks is a decentralized platform that transforms the field of scientific publishing by empowering authors, incentivizing reviewers, and heightening transparency.


To learn more before you start working with NobleBlocks, see the following documentation available online:

- [Quick Start](https://internetcomputer.org/docs/quickstart/quickstart-intro)
- [SDK Developer Tools](https://internetcomputer.org/docs/developers-guide/sdk-guide)
- [Rust Canister Devlopment Guide](https://internetcomputer.org/docs/rust-guide/rust-intro)
- [ic-cdk](https://docs.rs/ic-cdk)
- [ic-cdk-macros](https://docs.rs/ic-cdk-macros)
- [Candid Introduction](https://internetcomputer.org/docs/candid-guide/candid-intro)
- [JavaScript API Reference](https://erxue-5aaaa-aaaab-qaagq-cai.raw.icp0.io)

## Running the project locally

If you want to test your project locally, you can use the following commands:

```bash
# Clone from github
git clone git@github.com:NOBLBLOCKS/backend.git

cd backend

# Starts the replica, running in the background
dfx start --background

# Deploys your canisters to the replica and generates your candid interface
./scripts/deploy-local.sh
```

Once the job completes, your application will be available at `http://localhost:8080?canisterId={asset_canister_id}`.
