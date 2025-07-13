
run_crate_00_hello_world:
	cd crate_00_hello_world; \
	cargo clean --package crate_00_hello_world; \
	cargo check --package crate_00_hello_world; \
	cargo build --package crate_00_hello_world; \
	cargo run --package crate_00_hello_world --bin crate_00_hello_world \
