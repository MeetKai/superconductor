RUSTFLAGS=--cfg=web_sys_unstable_apis cargo build --target wasm32-unknown-unknown --release --features wasm &&
wasm-bindgen ../target/wasm32-unknown-unknown/release/superconductor_demo.wasm --out-dir web/pkg --target web &&
cd web &&
caddy file-server --listen :8000 --access-log