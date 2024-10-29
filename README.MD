# About
This is a simple Catch the Falling Items web game written in Rust using WebAssembly

# Install dependencies
rustup target add wasm32-unknown-unknown

cargo install wasm-bindgen-cli

curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

cargo build

# Build and Run
wasm-pack build --target web

# Serve the game from a local server
python3 -m http.server
