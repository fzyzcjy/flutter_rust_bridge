# `justfile`

This file defines the recipes for the [just] command runner, in a similar vein to `make` and Makefile. [just] is built using Rust and improves upon the traditional Makefile syntax with better support for
conditionals, arguments, cross-platform compatibility and more.

One non-trivial feature of [just] utilized by this template is the
conditional LLVM flag for MacOS. On certain setups, a `brew install llvm` does not make the LLVM libraries visible to other executables, which causes problems for `ffigen`, a C-to-Dart codegen that `flutter_rust_bridge_codegen` uses under the hood. It is certainly possible to rewrite this in a more C-friendly fashion, but the benefits of a functional, easy-to-grok command suite far outweights that of familiarity.

[just]: https://github.com/casey/just