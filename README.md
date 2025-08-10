# About 

Tictactoe built with svelte and rust using wasm.
Hosted using GitHub pages.

# Building
## Prerequisites

Build prerequisites are a working rust toolchain,
wasm-pack, npm and nodejs.

## Actual building

To build the project:

    wasm-pack build --target web ./tictactoe-rs
    npm install ./tictactoe-rs/pkg
    npm run build
