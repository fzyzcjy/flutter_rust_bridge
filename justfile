# To use this file, install Just: `cargo install just` (or other ways)

default:
    @# Make this the first recipe of justfile, such that when users type `just`, it will list all commands
    @just --list --unsorted --justfile {{justfile()}}

# ----------------------------------- linter -----------------------------------

lint: lint_rust lint_dart

# TODO frb_example packages are separated, are they ok?
lint_rust: _lint_rust_main _lint_rust_wasm

_lint_rust_main:
    cargo fmt
    cargo clippy -- -D warnings

_lint_rust_wasm:
    rustup target add wasm32-unknown-unknown
    cd frb_rust && cargo clippy --target wasm32-unknown-unknown -- -D warnings

lint_dart:
    TODO
