# flutter_package_native_assets

A Flutter package using `flutter_rust_bridge` with native assets.

## Project Structure

* `rust`: Contains the Rust crate.
* `hook`: Contains the native assets build hook.
* `lib`: Contains the Dart API and generated bridge code.
* `example`: Contains a Flutter app that depends on this package.

## Building Native Code

Flutter invokes `hook/build.dart` during builds. The hook uses `flutter_rust_bridge_hooks` to build and bundle the Rust crate as native assets.

## Regenerating Bridge Code

After changing Rust APIs, run `flutter_rust_bridge_codegen generate` from the package root.
