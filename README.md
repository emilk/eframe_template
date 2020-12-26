# Egui Template

[![dependency status](https://deps.rs/repo/github/emilk/egui_template/status.svg)](https://deps.rs/repo/github/emilk/egui_template)
[![Build Status](https://github.com/emilk/egui_template/workflows/CI/badge.svg)](https://github.com/emilk/egui_template/actions?workflow=CI)

This is a template repo for [Egui](https://github.com/emilk/egui/).

The goal is for this to be the simplest way to get started writing a GUI app in Rust.

You can compile your app natively or for the web, and share it using Github Pages.


## Getting started

Start by clicking "Use this template" at https://github.com/emilk/egui_template/ or follow [these instructions](https://docs.github.com/en/free-pro-team@latest/github/creating-cloning-and-archiving-repositories/creating-a-repository-from-a-template).

`src/app.rs` contains a simple example app. This is just to give some inspiration - most of it can be removed if you like.

### Testing locally

`cargo run --release`

### Compiling for the web

You can compile your app to [WASM](https://en.wikipedia.org/wiki/WebAssembly) and publish it as a web page. For this you need to set up some tools. There are a few simple scripts that help you with this:

``` sh
./setup_web.sh
./build_web.sh
./start_server.sh
open http://127.0.0.1:8080/
```

* `setup_web.sh` installs the tools required to build for web
* `build_web.sh` compiles your code to wasm and puts it in the `docs/` folder (see below)
* `start_server.sh` starts a local HTTP server so you can test before you publish
* Open http://127.0.0.1:8080/ in a web browser to view

The finished we app is found in the `docs/` folder (this is so that you can easily share it with [GitHub Pages](https://docs.github.com/en/free-pro-team@latest/github/working-with-github-pages/configuring-a-publishing-source-for-your-github-pages-site)). It consists of three files:

* `index.html`: A few lines of HTML, CSS and JS that loads your app. **You need to edit this** (once) to replace `egui_template` with the name of your crate!
* `your_crate_bg.wasm`: What the Rust code compiles to.
* `your_crate.js`: Auto-generated binding between Rust and JS.

You can test the template app at <https://emilk.github.io/egui_template/>.

## Updating Egui

As of 2020, Egui is in active development with frequent releases with breaking changes. When updating Egui, update the version string in `Cargo.toml` of `egui`, `egui_glium` and `egui_web` (they should match). You can also check out the [egui_template](https://github.com/emilk/egui_template/) repository to see what changes has happened to it.
