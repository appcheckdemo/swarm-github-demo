# Build and run

These are static fixtures. Dependencies are a POSIX shell, Python 3, and a browser or headless renderer; no package install or network is required.

```sh
cargo build
python3 -m http.server 8000 --directory fixtures
# Desktop: open fixtures/rendering/index.html
# Local task app: http://127.0.0.1:8000/task-list/index.html
chromium --headless --no-sandbox --disable-gpu --screenshot=/tmp/rendering.png --window-size=1280,900 http://127.0.0.1:8000/rendering/index.html
```

The executable may be `google-chrome` or `chromium-browser`. Stop the server with Ctrl-C. Run infinite-loop.html only with an external process timeout.
