echo "Checking and building wasm-game-of-life crate from source"
cd rust
cargo check --verbose
cargo build --release --verbose

echo "Testing wasm-game-of-life crate"
cargo test --release --verbose

echo "Building module from wasm-game-of-life crate as web target at pkg/"
wasm-pack build --target web