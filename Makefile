export PATH := $(HOME)/.cargo/bin:$(PATH)

target/wasm32-unknown-unknown/debug/wasm_canvas.wasm: src/lib.rs
	cargo +nightly build --target wasm32-unknown-unknown

static/debug.js: target/wasm32-unknown-unknown/debug/wasm_canvas.wasm
	wasm-bindgen $< --out-dir static --out-name debug --no-modules --no-typescript

target/wasm32-unknown-unknown/release/wasm_canvas.wasm: src/lib.rs
	cargo +nightly build --release --target wasm32-unknown-unknown

static/release.js: target/wasm32-unknown-unknown/release/wasm_canvas.wasm
	wasm-bindgen $< --out-dir static --out-name release --no-modules --no-typescript
