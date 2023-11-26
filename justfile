# To use this file, install Just: `cargo install just`

default:
    @# Make this the first recipe of justfile, such that when users type `just`, it will list all commands
    @just --list --unsorted --justfile {{justfile()}}
