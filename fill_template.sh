#!/usr/bin/env sh

set -e

echo "To fill the template tell me your egui project crate name: "
read crate

echo "To fill the template tell me your name (for author in Cargo.toml): "
read name

echo "To fill the template tell me your e-mail address (also for Cargo.toml): "
read email

echo "Patching files..."

# Determine the correct sed syntax based on OS
if [ "$(uname)" = "Darwin" ]; then
  # macOS sed
  sed_inplace() {
    sed -i '' "$@"
  }
else
  # GNU sed (Linux)
  sed_inplace() {
    sed -i "$@"
  }
fi

sed_inplace "s/eframe_template/$crate/g" Cargo.toml
sed_inplace "s/eframe_template/$crate/g" src/main.rs
sed_inplace "s/eframe template/$crate/g" index.html
sed_inplace "s/eframe_template/$crate/g" assets/sw.js
sed_inplace "s/Emil Ernerfeldt/$name/g" Cargo.toml
sed_inplace "s/emil.ernerfeldt@gmail.com/$email/g" Cargo.toml

echo "Done."
