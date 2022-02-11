# Installing dependencies

Next, we need to install a few build-time and runtime dependencies.

## Build-time dependencies

These are the same dependencies listed in [Installing codegen](../generate/install.md), but to reiterate:

- [`flutter_rust_bridge_codegen`](https://lib.rs/crates/flutter_rust_bridge_codegen), the core codegen for Rust-Dart glue code
- [`cbindgen`](https://lib.rs/crates/cbindgen), to generate C headers from Rust FFI code
- [`ffigen`](https://pub.dev/packages/ffigen), to generate Dart code from C headers
- A working installation of LLVM, see [Installing LLVM](https://pub.dev/packages/ffigen#installing-llvm)

An easy way to install the first three dependencies is to run these commands:
```
cargo install flutter_rust_bridge_codegen cbindgen
dart pub global activate ffigen
```

## Dart dependencies

On the Dart side, `flutter_rust_bridge` is the required runtime component of
`flutter_rust_bridge_codegen`. If you plan to use enum structs in Rust, the
following dependencies are also needed:
- `build_runner` (dev)
- `freezed` (dev)
- `freezed_annotation`

Their usage is explained in [Using `build_runner`](../generate/build_runner.md).

[`just`]: https://github.com/casey/just
