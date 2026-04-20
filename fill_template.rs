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
        "{{ crate }}",
    )?;
    do_switch(&["Cargo.toml"], "Emil Ernerfeldt", "{{ author }}")?;
    do_switch(&["Cargo.toml"], "emil.ernerfeldt@gmail.com", "{{ email }}")?;
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
