default: dev

# Start both client and server concurrently
dev:
    #!/usr/bin/env bash
    just server &
    just client

# Start the Vite dev server
client:
    cd apps/client && pnpm dev

# Start the Axum server
server:
    cargo run -p equate-server

# Compile the game engine to WASM for the client
build-wasm:
    wasm-pack build packages/game-engine --target web --out-dir ../../apps/client/src/wasm

# Run all Rust tests
test:
    cargo test

# Clean build artifacts
clean:
    cargo clean
    rm -rf apps/client/node_modules apps/client/dist apps/client/src/wasm
