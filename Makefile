build:
	cargo build --lib --release --target=wasm32-unknown-unknown

nm:
	wasm-objdump -d target/wasm32-unknown-unknown/release/proxy_wasm.wasm | grep func

run:
	envoy-wasm --config-path ./envoy.yaml