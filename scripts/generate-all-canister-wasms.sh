#!/bin/sh

SCRIPT=$(readlink -f "$0")
SCRIPT_DIR=$(dirname "$SCRIPT")
cd $SCRIPT_DIR/..

./scripts/generate-wasm.sh user_index
./scripts/generate-wasm.sh local_user_index
./scripts/generate-wasm.sh post_index
./scripts/generate-wasm.sh local_post_index
