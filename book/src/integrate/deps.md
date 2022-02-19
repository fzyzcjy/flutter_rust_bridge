# Installing dependencies

Next, we need to install a few build-time and runtime dependencies.

## Build-time dependencies

These depdencies are required only in build-time:

- [`flutter_rust_bridge_codegen`](https://lib.rs/crates/flutter_rust_bridge_codegen), the core codegen for Rust-Dart glue code
- [`cbindgen`](https://lib.rs/crates/cbindgen), to generate C headers from Rust FFI code
- [`ffigen`](https://pub.dev/packages/ffigen), to generate Dart code from C headers
- [`cargo-xcode`](https://lib.rs/crates/cargo-xcode), to generate Xcode projects for iOS and MacOS
- A working installation of LLVM, see [Installing LLVM](https://pub.dev/packages/ffigen#installing-llvm)

An easy way to install most of these dependencies is to run:
```bash
cargo install flutter_rust_bridge_codegen cbindgen
dart pub global activate ffigen
# if building for iOS or MacOS
cargo install cargo-xcode
```

## Dart dependencies

On the Dart side, `flutter_rust_bridge` is the required runtime component of
`flutter_rust_bridge_codegen`. If you plan to use enum structs in Rust, the
following dependencies are also needed:
- `build_runner` (dev)
- `freezed` (dev)
- `freezed_annotation`

Their usage is explained in [Using `build_runner`](../generate/build_runner.md).

```bash
flutter pub add flutter_rust_bridge
# if using Dart codegen
flutter pub add -d build_runner
flutter pub add -d freezed
flutter pub add freezed_annotation
```

## Rust dependencies

Similar to Dart, Rust requires the `flutter_rust_bridge` runtime component for support.

Add these lines to `Cargo.toml`:

```diff
+[dependencies]
+flutter_rust_bridge = "1"
```

> **Note** \
> If you wish to return a `Result`, keep in mind that this library can only run codegen
> for [`anyhow::Result`](https://docs.rs/anyhow/latest/anyhow/type.Result.html).
