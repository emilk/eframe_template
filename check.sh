#!/bin/bash
# This scripts runs various CI-like checks in a convenient way.
set -eu

cargo check --workspace --all-targets
cargo check --workspace --all-features --lib --target wasm32-unknown-unknown
cargo fmt --all -- --check
CARGO_INCREMENTAL=0 cargo clippy --workspace --all-targets --all-features --  -D warnings -W clippy::all
cargo test --workspace --all-targets --all-features
cargo test --workspace --doc
