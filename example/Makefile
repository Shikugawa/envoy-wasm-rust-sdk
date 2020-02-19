build:
	cargo build --lib --release --target=wasm32-unknown-unknown

nm:
	wasm-objdump -d target/wasm32-unknown-unknown/release/example.wasm | grep func

run:
	envoy-static --config-path ./envoy.yaml --concurrency 1
