# `justfile`

This file defines the recipes for the [just] command runner, in a similar vein to `make` and Makefile. [just] is built using Rust and improves upon the traditional Makefile syntax with better support for
conditionals, arguments, cross-platform compatibility and more.

One non-trivial feature of [just] utilized by this template is the
conditional LLVM flag for MacOS. On certain setups, a `brew install llvm` does not make the LLVM libraries visible to other executables, which causes problems for `ffigen`, a C-to-Dart codegen that `flutter_rust_bridge_codegen` uses under the hood.

Running `just` by default runs the `gen` and `lint` tasks.

## `just gen`

Generates the Rust bindings and puts them into the correct folders.
The [Generating new code](generate.md) section goes into detail how to modify
this task to perform side jobs as well.

## `just lint`

Runs the default linters for Dart and Rust.

## `just clean`

Runs the default clean commands for Flutter and Rust.
Useful when you want to debug build-related issues.

[just]: https://github.com/casey/just

