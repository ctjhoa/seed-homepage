cargo build --target wasm32-unknown-unknown
wasm-bindgen target/wasm32-unknown-unknown/debug/seed_homepage.wasm --no-modules --out-dir ./pkg --out-name package