#!/bin/bash
# This scripts runs various CI-like checks in a convenient way.
set -eu

# This is required to enable the web_sys clipboard API
# https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Clipboard.html
# https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html
export RUSTFLAGS=--cfg=web_sys_unstable_apis

cargo check --workspace --all-targets --all-features --release
cargo fmt --all -- --check
CARGO_INCREMENTAL=0 cargo clippy --workspace --all-targets --all-features --  -D warnings -W clippy::all
cargo test --workspace --all-targets --all-features
cargo test --workspace --doc

cargo check --lib --target wasm32-unknown-unknown
