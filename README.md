# native-browser

A small Rust-native browser foundation. It includes no WebView, browser engine, JavaScript engine, HTML/CSS parser, or layout-engine dependency: parsing, vertical layout, and software painting are intentionally minimal and replaceable.

## Linux dependencies

For optional native window support, install `libx11-dev libxi-dev libxrandr-dev libxcursor-dev libxinerama-dev` on Debian/Ubuntu. Rust and Cargo are required.

## Commands

`cargo build` builds the project. Load a local file or inline HTML with `cargo run -- page.html`. Produce deterministic PNG output with `cargo run -- page.html --headless output.png`. Public interfaces are in `model`, `dom`, `style`, `layout`, `paint`, `browser`, `resources`, `navigation`, `js`, and `events`.
