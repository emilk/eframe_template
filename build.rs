use regex::Regex;
use std::{
    fs::{read_to_string, File},
    io::Write,
};
use toml::Value;

fn main() {
    let cargo_toml_raw = include_str!("Cargo.toml");
    let cargo_toml: Value = toml::from_str(cargo_toml_raw).unwrap();
    let name = cargo_toml["package"]["name"].as_str().unwrap();
    let version = cargo_toml["package"]["version"].as_str().unwrap();

    let old_version = read_to_string("assets/version").ok();

    if Some(version.to_string()) != old_version {
        write_version(version).unwrap();

        let mut assets = get_assets();

        let index = insert_files_into_index(&assets);
        write_index(&index).unwrap();

        assets.append(&mut vec![
            "".to_string(),
            "index.html".to_string(),
            format!("{}.js", name.replace('-', "_")),
            format!("{}_bg.wasm", name.replace('-', "_")),
        ]);

        let cache_files = assets
            .iter()
            .map(|v| format!("'./{}'", v))
            .collect::<Vec<String>>();

        let sw = replace_sw_cache_files(cache_files);
        let sw = replace_cache_name(&sw, name);
        write_sw(&sw).unwrap();
    }
}

fn get_assets() -> Vec<String> {
    std::fs::read_dir("assets")
        .unwrap()
        .map(|v| v.unwrap().file_name().to_str().unwrap().to_string())
        .collect()
}

fn insert_files_into_index(files: &[String]) -> String {
    let indexhtml = include_str!("index.html");
    let insert_pos = "<!-- Insert assets here -->";
    let mut lines: Vec<_> = files
        .iter()
        .filter(|v| !is_in_index(indexhtml, v))
        .map(|v| {
            format!(
                "    <link data-trunk rel=\"copy-file\" href=\"assets/{}\" />",
                v
            )
        })
        .collect();
    lines.insert(0, insert_pos.to_string());
    indexhtml.replace(insert_pos, &lines.join("\n"))
}

fn is_in_index(text: &str, file: &str) -> bool {
    let pattern = format!("<link data-trunk rel=\".+?\" href=\"assets/{}\"", file);
    Regex::new(&pattern).unwrap().is_match(text)
}

fn replace_sw_cache_files(items: Vec<String>) -> String {
    let regex = Regex::new(r"\s+filesToCache\s*=\s*\[[^\]]*\];").unwrap();
    let replacement_files = format!(" filesToCache = [ {} ];", items.join(", "));
    regex
        .replace(include_str!("assets/sw.js"), replacement_files)
        .to_string()
}

fn replace_cache_name(text: &str, name: &str) -> String {
    let replacement_cache_name = format!(" cacheName = '{}-pwa';", name.replace('_', "-"));
    let regex = Regex::new("( cacheName = )(\"|').+?(\"|');").unwrap();
    regex.replace(text, replacement_cache_name).to_string()
}

fn write_version(text: &str) -> std::io::Result<()> {
    let mut ver = File::create("assets/version").unwrap();
    ver.write_all(text.as_bytes())
}

fn write_sw(text: &str) -> std::io::Result<()> {
    let mut sw = File::create("assets/sw.js").unwrap();
    sw.write_all(text.as_bytes())
}

fn write_index(text: &str) -> std::io::Result<()> {
    let mut index = File::create("index.html").unwrap();
    index.write_all(text.as_bytes())
}
