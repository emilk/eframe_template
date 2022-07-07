# eframe template

[![dependency status](https://deps.rs/repo/github/emilk/eframe_template/status.svg)](https://deps.rs/repo/github/emilk/eframe_template)
[![Build Status](https://github.com/emilk/eframe_template/workflows/CI/badge.svg)](https://github.com/emilk/eframe_template/actions?workflow=CI)

This is a template repo for [eframe](https://github.com/emilk/egui/tree/master/eframe), a framework for writing apps using [egui](https://github.com/emilk/egui/).

The goal is for this to be the simplest way to get started writing a GUI app in Rust.

You can compile your app natively or for the web, and share it using Github Pages.

## Getting started

Start by clicking "Use this template" at https://github.com/emilk/eframe_template/ or follow [these instructions](https://docs.github.com/en/free-pro-team@latest/github/creating-cloning-and-archiving-repositories/creating-a-repository-from-a-template).

Change the name of the crate: Chose a good name for your project, and change the name to it in:
* `Cargo.toml`
    * Change the `package.name` from `eframe_template` to `your_crate`
    * Change the `package.authors`
    * Change the `package.default-run` from `eframe_template_bin` to `your_crate_bin` (note the `_bin`!)
    * Change the `bin.name` from `eframe_template_bin` to `your_crate_bin` (note the `_bin`!)
* `main.rs`
    * Change `eframe_template::TemplateApp` to `your_crate::TemplateApp`
* `docs/index.html`
    * Change the `<title>`
    * Change the `<script src=…` line from `eframe_template.js` to `your_crate.js`
    * Change the `wasm_bindgen(…` line from `eframe_template_bg.wasm` to `your_crate_bg.wasm` (note the `_bg`!)
* `docs/sw.js`
    * Change the `'./eframe_template.js'` to `./your_crate.js` (in `filesToCache` array)
    * Change the `'./eframe_template_bg.wasm'` to `./your_crate_bg.wasm` (in `filesToCache` array)
* Remove the web build of the old name: `rm docs/eframe_template*`

### Learning about egui

`src/app.rs` contains a simple example app. This is just to give some inspiration - most of it can be removed if you like.

The official egui docs are at <https://docs.rs/egui>. If you prefer watching a video introduction, check out <https://www.youtube.com/watch?v=NtUkr_z7l84>. For inspiration, check out the [the egui web demo](https://emilk.github.io/egui/index.html) and follow the links in it to its source code.

### Testing locally

Make sure you are using the latest version of stable rust by running `rustup update`.

`cargo run --release`

On Linux you need to first run:

`sudo apt-get install libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libspeechd-dev libxkbcommon-dev libssl-dev`

On Fedora Rawhide you need to run:

`dnf install clang clang-devel clang-tools-extra speech-dispatcher-devel libxkbcommon-devel pkg-config openssl-devel libxcb-devel`

For running the `build_web.sh` script you also need to install `jq` and `binaryen` with your packet manager of choice.

### Compiling for the web

Install [jq](https://stedolan.github.io/jq/download/).

Make sure you are using the latest version of stable rust by running `rustup update`.

You can compile your app to [WASM](https://en.wikipedia.org/wiki/WebAssembly) and publish it as a web page. For this you need to set up some tools. There are a few simple scripts that help you with this:

```sh
./setup_web.sh
./start_server.sh
./build_web.sh --optimize --open
```

* `setup_web.sh` installs the tools required to build for web
* `start_server.sh` starts a local HTTP server so you can test before you publish
* `build_web.sh` compiles your code to WASM and puts it in the `docs/` folder (see below) and `--open` opens the result in your default browser.

The finished web app is found in the `docs/` folder (this is so that you can easily share it with [GitHub Pages](https://docs.github.com/en/free-pro-team@latest/github/working-with-github-pages/configuring-a-publishing-source-for-your-github-pages-site)). It consists of three files:

* `index.html`: A few lines of HTML, CSS and JS that loads your app. **You need to edit this** (once) to replace `eframe_template` with the name of your crate!
* `your_crate_bg.wasm`: What the Rust code compiles to.
* `your_crate.js`: Auto-generated binding between Rust and JS.

You can test the template app at <https://emilk.github.io/eframe_template/>.

### Web testing/development

Open `index.html#dev` to disable caching, which makes development easier.

## Updating egui

As of 2022, egui is in active development with frequent releases with breaking changes. [eframe_template](https://github.com/emilk/eframe_template/) will be updated in lock-step to always use the latest version of egui.

When updating `egui` and `eframe` it is recommended you do so one version at the time, and read about the changes in [the egui changelog](https://github.com/emilk/egui/blob/master/CHANGELOG.md) and [eframe changelog](https://github.com/emilk/egui/blob/master/eframe/CHANGELOG.md).
