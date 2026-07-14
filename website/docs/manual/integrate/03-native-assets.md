# Native assets

Native Assets are the Dart/Flutter build hooks mechanism for building and bundling native code assets.
In flutter_rust_bridge, this is a compilation backend: it controls how the Rust library is built and packaged.
It does not replace the core bridge code generator.

:::info
The flutter_rust_bridge Native Assets backend is available through `--integration-backend native-assets`.
Use `flutter_rust_bridge_codegen >= 2.13.0-beta.2` and a Flutter/Dart SDK that supports build hooks and code assets.
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

For existing projects that already use Cargokit, see [Migrate from Cargokit to Native Assets](migrate-cargokit-to-native-assets).

## Rust crate requirements

The Rust crate must be buildable by Cargo for the requested target platform.
For `native_toolchain_rust`, the crate needs a library target that can produce both static and dynamic artifacts:

```text
[lib]
crate-type = ["staticlib", "cdylib"]
```

The crate also needs a `rust-toolchain.toml` that pins a concrete Rust toolchain and lists the supported targets.
Do not rely on a floating `stable`, `beta`, or `nightly` channel for reproducible build hooks.
