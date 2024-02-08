# Run in build.rs

There are basically two approaches to execute the code generator.
The first and most evident approach is to directly execute the `flutter_rust_bridge_codegen` in command line.

The second approach is to integrate it into `build.rs` of your project.
With this approach, the code generator is automatically triggered whenever you build your Rust project.
For example configuration, have a look at this [build.rs](https://github.com/fzyzcjy/flutter_rust_bridge/blob/master/frb_example/dart_build_rs/rust/build.rs) file.
Don't forget to configure the 'build-dependency' in our [cargo.toml](https://github.com/fzyzcjy/flutter_rust_bridge/blob/master/frb_example/dart_build_rs/rust/Cargo.toml) to depend on `flutter_rust_bridge_codegen = <version you use>`.
