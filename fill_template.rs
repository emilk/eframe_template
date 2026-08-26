#!/usr/bin/env -S cargo +nightly -Zscript
---cargo
[dependencies]
anyhow = "1.0.102"
---


use std::{io::Write as _, path::Path};

use anyhow::Context as _;

pub fn main() -> anyhow::Result<()> {
    do_switch(
        &["Cargo.toml", "src/main.rs", "index.html", "assets/sw.js"],
        "eframe_template",
        "{{ crate_name }}",
    )?;
    do_switch(
        &["Cargo.toml"],
        "Emil Ernerfeldt <emil.ernerfeldt@gmail.com>",
        "{{ authors }}",
    )?;
    do_switch(
        &["src/main.rs", "src/app.rs", "index.html"],
        "eframe template",
        "{{ crate_display_name }}",
    )?;
    do_switch(
        &["assets/sw.js"],
        "egui-template-pwa",
        "{{ pwa_short_name }}",
    )?;
    do_switch(
        &["assets/manifest.json"],
        "egui-template-pwa",
        "{{ pwa_short_name }}",
    )?;
    do_switch(
        &["assets/manifest.json"],
        "egui Template PWA",
        "{{ pwa_name }}",
    )?;
    do_switch(
        &["src/app.rs", "src/lib.rs", "src/main.rs"],
        "TemplateApp",
        "{{ app_struct_identifier }}",
    )?;
    do_switch(
        &["README.md"],
        "# eframe template",
        "# {{ crate_display_name }}",
    )?;
    do_switch(
        &["README.md"],
        "repo/github/emilk/eframe_template",
        "repo/github/{{ github_repository_owner_and_name }}",
    )?;
    do_switch(
        &["README.md"],
        "emilk/eframe_template/workflows",
        "{{ github_repository_owner_and_name }}/workflows",
    )?;
    do_switch(
        &["README.md"],
        "emilk/eframe_template/actions",
        "{{ github_repository_owner_and_name }}/actions",
    )?;
    println!("Completed");
    Ok(())
}

fn do_switch<P: std::fmt::Debug + AsRef<Path>>(
    paths: &[P],
    from: &str,
    to: &str,
) -> anyhow::Result<()> {
    for path in paths {
        let contents = std::fs::read_to_string(path)
            .with_context(|| format!("failed to read file contents of: {path:?}"))?;
        let output = contents.replace(from, to);
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)
            .with_context(|| format!("failed to open file for writing: {path:?}"))?;
        file.write_all(output.as_bytes())
            .with_context(|| format!("failed to write changes to: {path:?}"))?;
    }
    Ok(())
}
