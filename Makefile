updest:
	RUSTFLAGS='-C link-arg=-s' cargo build --release --target wasm32-unknown-unknown
	mkdir -p ./artifacts
	cp ./target/wasm32-unknown-unknown/release/*.wasm ./artifacts
	rm -f ./configs/co*.json
	npm test


compress:
	gzip -f9  ./artifacts/*.wasm


schema:
	cargo run --example schema
