#!/usr/bin/env sh

set -e

echo "To fill the template tell me your egui project crate name: "

read crate

echo "To fill the template tell me your name (for author in Cargo.toml): "

read name

echo "To fill the template tell me your e-mail address (also for Cargo.toml): "

read email

echo "To fill the template tell me your android package name: "

read package

echo "Patching files..."

sed -i "s/eframe_template/$crate/g" Cargo.toml
sed -i "s/eframe_template/$crate/g" src/main.rs
sed -i "s/eframe template/$crate/g" index.html
sed -i "s/eframe_template/$crate/g" assets/sw.js
sed -i "s/Emil Ernerfeldt/$name/g" Cargo.toml
sed -i "s/emil.ernerfeldt@gmail.com/$email/g" Cargo.toml

sed -i "s/eframe_template/$crate/g" kotlin/MainActivity.kt
sed -i "s/rs\.egui\.$crate/$package/g" kotlin/MainActivity.kt
sed -i "s/rs\.egui\.eframe_template/$package/g" manifest.yaml

echo "Done."

