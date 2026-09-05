# Build and Release App

## Android/iOS/Linux/MacOS/Windows

In order to build and release your app,
simply follow [Flutter's documentation](https://docs.flutter.dev/deployment)
as if the Flutter+Rust app is a Flutter-only one.

## Web

Build the Rust WebAssembly module before the Flutter application:

```shell
flutter_rust_bridge_codegen build-web --release
```

Choose the Dart compiler for the Flutter application:

| Dart compiler | Command |
| --- | --- |
| WebAssembly | `flutter build web --wasm` |
| JavaScript | `flutter build web` |

Deploy the complete `build/web` directory and add the required [cross-origin headers](../../manual/miscellaneous/web-cross-origin) to your web server configuration.

See [Build for Web](web) for pure Dart commands, generated artifacts, browser compatibility, and fallback behavior.
