# Installing dependencies

Next, we need to install a few build-time and runtime dependencies.

## Build-time dependencies

These dependencies are required only in build-time:

- [`flutter_rust_bridge_codegen`](https://lib.rs/crates/flutter_rust_bridge_codegen), the core codegen for Rust-Dart glue code
- [`ffigen`](https://pub.dev/packages/ffigen), to generate Dart code from C headers
- A working installation of LLVM, see [Installing LLVM](https://pub.dev/packages/ffigen#installing-llvm), used by `ffigen`
- (Optional) [`cargo-xcode`](https://lib.rs/crates/cargo-xcode), if you want to generate Xcode projects for iOS and MacOS

An easy way to install most of these dependencies is to run:

- dart project
  
  ```bash
  cargo install flutter_rust_bridge_codegen
  dart pub add --dev ffigen && dart pub add ffi
  # if building for iOS or MacOS
  cargo install cargo-xcode
  ```

- flutter project

  ```bash
  cargo install flutter_rust_bridge_codegen
  flutter pub add --dev ffigen && flutter pub add ffi
  # if building for iOS or MacOS
  cargo install cargo-xcode
  ```

Alternatively, each of these dependencies may provide prebuilt binaries. Check with
your package manager and review them individually.

## Dart dependencies

On the Dart side, `flutter_rust_bridge` is the required runtime component of
`flutter_rust_bridge_codegen`. If you plan to use enum structs in Rust, the
following dependencies are also needed:

- `build_runner` (dev)
- `freezed` (dev)
- `freezed_annotation`

Their usage is explained in [Using `build_runner`](../template/generate/build-runner).

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

## System dependencies

### Non-Debian Linux

For non-debian based Linux distributions, there are a few prerequisites:

Firstly, ensure that packages are up to date (or install by demand).

- clang
- llvm-libs
- glibc

Restarting system may be required.

Secondly, set the environment variable in your shell profile (`.bashrc`, `.zshrc`, etc):

```bash
export CPATH="$(clang -v 2>&1 | grep "Selected GCC installation" | rev | cut -d' ' -f1 | rev)/include"
```
