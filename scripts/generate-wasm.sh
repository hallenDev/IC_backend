#!/bin/sh

SCRIPT=$(readlink -f "$0")
SCRIPT_DIR=$(dirname "$SCRIPT")
cd $SCRIPT_DIR/..

CANISTER_NAME=$1
PACKAGE="${CANISTER_NAME}_canister_impl"

echo Building package $PACKAGE
cargo build --locked --target wasm32-unknown-unknown --release --package $PACKAGE

echo Optimising wasm
if ! cargo install --list | grep -Fxq "ic-wasm v0.3.7:"
then
  cargo install --version 0.3.7 ic-wasm
fi
ic-wasm ./target/wasm32-unknown-unknown/release/$PACKAGE.wasm -o ./target/wasm32-unknown-unknown/release/$PACKAGE-opt.wasm shrink

echo Compressing wasm
mkdir -p wasms
gzip -fckn target/wasm32-unknown-unknown/release/$PACKAGE-opt.wasm > ./wasms/$CANISTER_NAME.wasm.gz
