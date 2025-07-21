
run_crate_00:
	cd crate_00_hello_world; \
	cargo clean --package crate_00_hello_world; \
	cargo check --package crate_00_hello_world; \
	cargo build --package crate_00_hello_world; \
	cargo build --release; \
	cargo run --package crate_00_hello_world --bin crate_00_hello_world

run_crate_01:
	cd crate_01_variables_and_mutability; \
	cargo clean; \
	cargo run --bin intro_to_variables; \
	cargo run --bin interpolation_with_curly_braces; \
	cargo run --bin mutable_vs_immutable; \
	cargo run --bin variable_shadowing; \
	cargo run --bin constants; \
	cargo run --bin scopes; \
	cargo run --bin type_aliases

# cargo check; \
# cargo build --release; \
# cargo run

# constants
# scopes
# type_aliases
# variable_shadowing


format_file:
	rustfmt ./crate_*/src/*.rs

format_projects:
	cargo fmt --verbose --manifest-path ./crate_00_hello_world/Cargo.toml

run_all_executables:
	./crate_*/target/debug/crate_*

