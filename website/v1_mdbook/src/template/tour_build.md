# `native/build.rs`

This is the entry point of build-script. It runs flutter_rust_bridge_codegen for you before every build automatically so you do not have to. This is especially useful when using rust-analyzer to remove errors from outdated generated file.

You can see [the example file here](https://github.com/fzyzcjy/flutter_rust_bridge/blob/master/frb_example/pure_dart/rust/build.rs).
