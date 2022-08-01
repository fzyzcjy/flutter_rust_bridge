# Installing codegen

First, you will need the following programs, preferably visible from your path:

- [`flutter_rust_bridge_codegen`](https://lib.rs/crates/flutter_rust_bridge_codegen), the core codegen for Rust-Dart glue code
- [`ffigen`](https://pub.dev/packages/ffigen), to generate Dart code from C headers
- [`just`](https://github.com/casey/just), a modern command runner alternative to Make
- A working installation of LLVM, see [Installing LLVM](https://pub.dev/packages/ffigen#installing-llvm)

An easy way to install the first three dependencies is to run these commands:

* dart project
  ```shell
  cargo install flutter_rust_bridge_codegen just
  dart pub add --dev ffigen
  dart pub add ffi
  ```
* flutter project
  ```shell
  cargo install flutter_rust_bridge_codegen just
  flutter pub add --dev ffigen
  flutter pub add ffi
  ```

Alternatively, each of these dependencies may provide prebuilt binaries. Check with
your package manager and review them individually.
