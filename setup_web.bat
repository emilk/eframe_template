@REM Pre-requisites:
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli
cargo update -p wasm-bindgen

@REM For local tests with `./start_server`:
cargo install basic-http-server
