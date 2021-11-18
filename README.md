RUST-WASM-POC

compile a simple string of data (uint8array) from rust - wasm - javascript 



rustup default nightly
rustup target add wasm32-unknown-unknown
cargo install wasm-pack

cargo install wasm-gc 
cargo intsall https

cargo build --target wasm32-unknown-unknown --release 

wasm-gc target/wasm32-unknown-unknown/release/utils.wasm -o utils.gc.wasm

load
