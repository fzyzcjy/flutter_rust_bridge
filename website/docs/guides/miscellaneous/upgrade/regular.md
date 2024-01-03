# Regular upgrades

Since the bridge consists of three components, we need to upgrade them together:

1. **Code generator**: Run `cargo install 'flutter_rust_bridge_codegen@^2.0.0-dev.0'`, or use other installation methods mentioned in [the quickstart](../../../quickstart).
2. **Rust runtime**: Edit `Cargo.toml`, changing `flutter_rust_bridge = "=x.x.x"`.
3. **Dart runtime**: Edit `pubspec.yaml`, changing `flutter_rust_bridge: x.x.x`.
