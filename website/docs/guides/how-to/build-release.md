# Build and Release App

## Android/iOS/Linux/MacOS/Windows

In order to build and release your app,
simply follow [Flutter's documentation](https://docs.flutter.dev/deployment)
as if the Flutter+Rust app is a Flutter-only one.

## Web

For the Web platform only, besides following Flutter's documentation, there are two extra things to do:

1. Just like what is already done when developing the app, execute `flutter_rust_bridge_codegen build-web --release`
2. Add [cross-origin headers](../../manual/miscellaneous/web-cross-origin) to your web server configuration.
