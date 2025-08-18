# Colors
RED     := \033[0;31m
GREEN   := \033[0;32m
BLUE    := \033[0;34m
YELLOW  := \033[0;33m
NC      := \033[0m   # No Color / Reset

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
	cargo run --bin type_aliases; \
	cargo run --bin project

run_crate_02:
	cd crate_02_data_types; \
	cargo clean; \
	cargo run --bin integers; \
	cargo run --bin strings_and_raw_strings; \
	cargo run --bin methods; \
	cargo run --bin floats; \
	cargo run --bin type_casting; \
	cargo run --bin arithmetic_operations; \
	cargo run --bin logical_operations; \
	cargo run --bin char_and_arrays; \
	cargo run --bin traits; \
	cargo run --bin tuples; \
	cargo run --bin ranges_and_iteration; \
	cargo run --bin project

run_crate_03:
	cd crate_03_functions; \
	cargo clean; \
	cargo run --bin functions; \
	cargo run --bin project

run_crate_04:
	cd crate_04_control_flow; \
	cargo clean; \
	cargo run --bin conditions; \
	cargo run --bin project

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

# FLAGS_LATEX := -pdf -shell-escape -verbose -file-line-error -interaction=nonstopmode \
# 				-synctex=1 -outdir=./docs/build
#
# build_latex:
# 	latexmk $(FLAGS_LATEX) ./docs/notes.tex
#
# FLAGS_TYPST := --format=pdf --open --root=./docs/
#
# build_typst:
# 	typst compile $(FLAGS_TYPST) ./docs/notes.typ ./docs/notes.pdf
# # build_typst:
# # 	typst watch $(FLAGS_TYPST) ./docs/notes.typ ./docs/notes.pdf
