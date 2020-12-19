#!/bin/bash
set -eu

# ./setup_web.sh # <- call this first!

FOLDER_NAME=${PWD##*/}
CRATE_NAME=$FOLDER_NAME # assume crate name is the same as the folder name

export RUSTFLAGS=--cfg=web_sys_unstable_apis # required for the clipboard API

# Clear output from old stuff:
rm -rf docs/$CRATE_NAME.wasm

echo "Building rust…"
BUILD=release
cargo build --release -p $CRATE_NAME --lib --target wasm32-unknown-unknown

echo "Generating JS bindings for wasm…"
TARGET_NAME="$CRATE_NAME.wasm"
wasm-bindgen "target/wasm32-unknown-unknown/$BUILD/$TARGET_NAME" \
  --out-dir docs --no-modules --no-typescript

echo "Finished: docs/$CRATE_NAME.wasm"
