WASM_DIR=$(shell pwd)/wasm

build:
	cd $(WASM_DIR) && wasm-pack build --target web
