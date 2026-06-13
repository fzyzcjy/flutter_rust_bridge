# Native assets

Native Assets are the Dart/Flutter build hooks mechanism for building and bundling native code assets.
In flutter_rust_bridge, this is a compilation backend: it controls how the Rust library is built and packaged.
It does not replace the core bridge code generator.

:::info
The flutter_rust_bridge Native Assets backend is available through `--integration-backend native-assets`.
:::

## Usage

Generate a project with:

```bash
flutter_rust_bridge_codegen create my_app --integration-backend native-assets
```

Or integrate into an existing project with:

```bash
flutter_rust_bridge_codegen integrate --integration-backend native-assets
```

The generated `hook/build.dart` uses `flutter_rust_bridge_hooks`.
That package currently wraps [`native_toolchain_rust`](https://pub.dev/packages/native_toolchain_rust), which compiles the Rust crate with Cargo and registers the result as a Dart/Flutter code asset.

## Current model

The first implementation keeps the existing flutter_rust_bridge generated IO bindings.
The Rust side is still built as a native library, such as `.so`, `.dylib`, or `.dll`.
The Dart side still uses the existing flutter_rust_bridge runtime and generated Dart API.

In other words, the Native Assets backend changes how the native library is compiled and bundled, not how Rust APIs are exposed to Dart.

Future versions may use `@Native`, `ffigen`, or Dart generate hooks for a more native-assets-oriented low-level binding model.
That is not required for the first backend.

## Rust crate requirements

The Rust crate must be buildable by Cargo for the requested target platform.
For `native_toolchain_rust`, the crate needs a library target that can produce both static and dynamic artifacts:

```toml
[lib]
crate-type = ["staticlib", "cdylib"]
```

The crate also needs a `rust-toolchain.toml` that pins a concrete Rust toolchain and lists the supported targets.
Do not rely on a floating `stable`, `beta`, or `nightly` channel for reproducible build hooks.
