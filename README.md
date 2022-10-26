# About 

Tictactoe built with svelte and rust using wasm.
Hosted at [url].

[url]: https://toppa102.gitlab.io/tictactoe-wasm/

# Building
## Prerequisites

Build prerequisites are a working rust toolchain,
wasm-pack, npm and nodejs.

## Actual building

To build the project:

    wasm-pack build --target web ./tictactoe-rs
    npm install ./tictactoe-rs/pkg
    npm run build
