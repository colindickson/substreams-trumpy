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
	substreams run ./trumpy-v3-v0.0.0.spkg map_mint_several -e polygon.streamingfast.io:443 -s 36860752 -t 36860852