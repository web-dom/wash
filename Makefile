build:
	@cargo build --target wasm32-unknown-unknown --release
	@cp target/wasm32-unknown-unknown/release/wash.wasm .
serve:
	http-server -p 8080
