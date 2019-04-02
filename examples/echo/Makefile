build:
	@cd .. && cd .. && make
	@cargo build --target wasm32-unknown-unknown --release
	@cp target/wasm32-unknown-unknown/release/echo.wasm .
