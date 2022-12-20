.PHONY: protogen
protogen:
	substreams protogen ./substreams.yaml

.PHONY: build
build: protogen
	cargo build --target wasm32-unknown-unknown --release

.PHONY: pack
pack: build
	substreams pack ./substreams.yaml

.PHONY: run
run: pack
	substreams run ./trumpy-v3-v0.0.0.spkg db_out -e polygon.streamingfast.io:443 -s 36856629 -t +10000