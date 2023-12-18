# Run in `build.rs`

There are basically two approaches to execute the code generator. The first and most evident approach is to directly execute the `flutter_rust_bridge` in command line.

The second approach is to integrate it into `build.rs` of your project. With this approach, the code generator is automatically triggered whenever you build your Rust project. For example configuration, have a look at this [build.rs](https://github.com/fzyzcjy/flutter_rust_bridge/blob/master/frb_example/pure_dart/rust/build.rs) file.

If the `build.rs` in the example projects is making it difficult to modify and test flutter_rust_bridge_codegen, you can rename it to disable it, and instead use the pure_dart and pure_dart_multi tests to debug any issues. Please refer to `frb_codegen/src/main.rs`'s tests for more details.
