.PHONY: protogen
protogen:
	substreams protogen ./substreams.yaml

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release
