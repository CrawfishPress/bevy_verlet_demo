
### Docs

    https://bevy-cheatbook.github.io/platforms/wasm.html
    https://rustwasm.github.io/docs/book/game-of-life/setup.html
    https://rustwasm.github.io/wasm-pack/installer/ # needed?
    https://rustwasm.github.io/wasm-bindgen/
    https://rustwasm.github.io/wasm-bindgen/reference/cli.html

### Setup
Added this to `~/.cargo/config.toml`

    [target.wasm32-unknown-unknown]
    runner = "wasm-server-runner"

Building:

    cargo build --release --target wasm32-unknown-unknown

### Webserving

    # cargo run --release --target wasm32-unknown-unknown # Worked in v0.7, see Issues

    wasm-bindgen  --out-dir ./web/ --target web target/wasm32-unknown-unknown/release/bevy-verlet.wasm
    - add an `index.html` file for serving
    python -m http.server --directory web

### Issues

Note: I had to remove bevy-feature "dynamic", as `dynlib` doesn't support WASM.

Had lots of problems getting the `wasm-server-runner` to work with v0.8 (it worked
in v0.7). Apparently, "under the hood", it's building some binary, and there were
versioning issues:

    it looks like the Rust project used to create this wasm file was linked against
    version of wasm-bindgen that uses a different bindgen format than this binary:
    
        rust wasm file schema version: 0.2.82
        this binary schema version: 0.2.81

So have to use the `wasm-bindgen` tool, to build a full WASM app, then use some
actual web-server (I use the built-in Python one) for testing.

Note: the "window" turns out to be larger than my browser's default - gotta use
< CTRL>- a few times, to shrink it down.
