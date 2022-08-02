#!/usr/bin/env bash
set -eu

CRATE_NAME=${PWD##*/} # assume crate name is the same as the folder name

script_path=$( cd "$(dirname "${BASH_SOURCE[0]}")" ; pwd -P )
cd "$script_path"

OPEN=false
OPTIMIZE=false

while test $# -gt 0; do
  case "$1" in
    -h|--help)
      echo "build_web.sh [--optimize] [--open]"
      echo "  --optimize: enable optimization step"
      echo "  --open: open the result in a browser"
      exit 0
      ;;
    -O|--optimize)
      shift
      OPTIMIZE=true
      ;;
    --open)
      shift
      OPEN=true
      ;;
    *)
      break
      ;;
  esac
done

./setup_web.sh

CRATE_NAME_SNAKE_CASE="${CRATE_NAME//-/_}" # for those who name crates with-kebab-case

# This is required to enable the web_sys clipboard API which egui_web uses
# https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Clipboard.html
# https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html
export RUSTFLAGS=--cfg=web_sys_unstable_apis

# Clear output from old stuff:
rm -f "docs/${CRATE_NAME_SNAKE_CASE}_bg.wasm"

echo "Building rust…"
BUILD=release
cargo build -p "${CRATE_NAME}" --release --lib --target wasm32-unknown-unknown

# Get the output directory (in the workspace it is in another location)
TARGET=$(cargo metadata --format-version=1 | jq --raw-output .target_directory)

echo "Generating JS bindings for wasm…"
TARGET_NAME="${CRATE_NAME_SNAKE_CASE}.wasm"
WASM_PATH="${TARGET}/wasm32-unknown-unknown/${BUILD}/${TARGET_NAME}"
wasm-bindgen "${WASM_PATH}" --out-dir docs --no-modules --no-typescript

if [[ "${OPTIMIZE}" == true ]]; then
  echo "Optimizing wasm…"
  # to get wasm-opt:  apt/brew/dnf install binaryen
  wasm-opt "docs/${CRATE_NAME}_bg.wasm" -O2 --fast-math -o "docs/${CRATE_NAME}_bg.wasm" # add -g to get debug symbols
fi

echo "Finished: docs/${CRATE_NAME_SNAKE_CASE}.wasm"

if [[ "${OPEN}" == true ]]; then
  if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    # Linux, ex: Fedora
    xdg-open http://localhost:8080/index.html#dev
  elif [[ "$OSTYPE" == "msys" ]]; then
    # Windows
    start http://localhost:8080/index.html#dev
  else
    # Darwin/MacOS, or something else
    open http://localhost:8080/index.html#dev
  fi
fi
