# native-browser

A dependency-free, native Rust browser architecture. It is not a WebView and intentionally does not embed HTML/CSS/JavaScript parsers or engines. Modules expose stable boundaries for a future platform backend.

## Linux prerequisites

Install Rust (stable) and a C toolchain (`build-essential` on Debian/Ubuntu). The baseline crate has no system libraries or Cargo dependencies.

## Reproducible use

```sh
cargo build --locked
cargo test --lib
cargo run -- https://example.invalid
cargo run -- --headless file:///tmp/page.html --output output.png --width 800 --height 600
```

The default transport and raster output are conservative stubs; application/platform backends implement the documented traits.# swarm-github-demo
PRD 146 swarm, GitHub operating mode: commits and pull requests from ten agents
