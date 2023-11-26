# To use this file, install Just: `cargo install just` (or other ways)

default:
    @# Make this the first recipe of justfile, such that when users type `just`, it will list all commands
    @just --list --unsorted --justfile {{justfile()}}

# ----------------------------------- linter -----------------------------------

lint: lint_rust lint_dart

lint_rust:
    TODO

lint_dart:
    TODO
