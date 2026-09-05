# Build and Release App

## Android/iOS/Linux/MacOS/Windows

In order to build and release your app,
simply follow [Flutter's documentation](https://docs.flutter.dev/deployment)
as if the Flutter+Rust app is a Flutter-only one.

## Web

1. Run `flutter_rust_bridge_codegen build-web --release`.
2. Run `flutter build web`, adding `--wasm` for Dart WebAssembly.
3. Deploy `build/web` with [cross-origin headers](../../manual/miscellaneous/web-cross-origin).

See [Build for Web](web) for details.
