# Native Browser

A deliberately small, native Rust browser vertical slice. The public modules form
replaceable contracts for DOM/HTML, CSS/style, layout/paint, resources, script,
events, networking, and navigation. The baseline parses simple markup and renders
a deterministic framebuffer; later implementations can replace modules without
changing the browser entry points.

## Build and run

Rust stable and Cargo are required. Desktop builds use the system window support
provided by `minifb`: Linux needs X11 (or Wayland plus the platform compatibility
libraries), macOS needs Cocoa, and Windows needs the normal desktop SDK. A
headless build needs no display server.

```sh
cargo build --all-targets
cargo test --lib
cargo run -- index.html
cargo run -- --headless index.html --output page.png
```

`--headless` uses the same document, style/layout, and paint interfaces as the
desktop path. Local files are supported now; network and interactive desktop
frontends have bounded baseline errors and are intentionally isolated behind
their public interfaces.
