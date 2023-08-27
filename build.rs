use regex::Regex;
use std::{
    fs::{read_to_string, File},
    io::Write,
};
use toml::Value;

#[cfg(target_arch = "wasm32")]
fn main() {
    let cargo_toml_raw = include_str!("Cargo.toml");
    let cargo_toml: Value = toml::from_str(cargo_toml_raw).unwrap();

    let version = cargo_toml["package"]["version"].as_str().unwrap();
    let old_version = read_to_string("assets/version").ok();
    if Some(version.to_string()) != old_version {
        let name = cargo_toml["package"]["name"].as_str().unwrap();
        let mut items = std::fs::read_dir("assets")
            .unwrap()
            .map(|v| v.unwrap().file_name().to_str().unwrap().to_string())
            .collect::<Vec<_>>();
        items.append(&mut vec![
            "".to_string(),
            "index.html".to_string(),
            format!("{}.js", name.replace("-", "_")),
            format!("{}_bg.wasm", name.replace("-", "_")),
        ]);
        let items = items
            .iter()
            .map(|v| format!("'./{}'", v))
            .collect::<Vec<_>>();
        let replacement_files = format!("var filesToCache = [ {} ];", items.join(", "));
        let replacement_cache_name = format!("var cacheName = '{}-pwa';", name.replace("_", "-"));

        let regex = Regex::new(r"var\s+filesToCache\s*=\s*\[[^\]]*\];").unwrap();
        let replaced = regex
            .replace(include_str!("assets/sw.js"), replacement_files)
            .to_string();
        let regex = Regex::new("(var cacheName = )(\"|').+?(\"|');").unwrap();
        let replaced = regex.replace(&replaced, replacement_cache_name).to_string();
        let mut sw = File::create("assets/sw.js").unwrap();
        sw.write_all(replaced.as_bytes()).unwrap();
        let mut ver = File::create("assets/version").unwrap();
        ver.write_all(version.as_bytes()).unwrap()
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {}
