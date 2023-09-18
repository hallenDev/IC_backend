IDENTITY=$1

# Create the NobleBlocks canisters
dfx --identity $IDENTITY canister create --no-wallet --with-cycles 100000000000000 user_index
dfx --identity $IDENTITY canister create --no-wallet --with-cycles 100000000000000 local_user_index
dfx --identity $IDENTITY canister create --no-wallet --with-cycles 100000000000000 post_index
dfx --identity $IDENTITY canister create --no-wallet --with-cycles 100000000000000 local_post_index

export SUPER_ADMIN="jyd6z-43tsj-pmjnt-dz6ir-b4qw3-bgksc-jzmq2-xafh3-vcfb4-eckz6-hqe"

./scripts/deploy.sh local $IDENTITY
