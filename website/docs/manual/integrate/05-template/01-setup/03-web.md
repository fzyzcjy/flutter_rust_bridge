# Web setup

Building on web requires nightly Rust, the `wasm32-unknown-unknown` target
and [wasm-pack], which can be installed using these commands:

```bash
rustup toolchain install nightly
rustup +nightly component add rust-src
rustup +nightly target add wasm32-unknown-unknown
# either of these
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
cargo install wasm-pack
```

Optionally (but highly recommended), install `flutter_rust_bridge_serve`
to expedite the process of building the WASM binary and setting up HTTP headers:

```bash
# in your Flutter/Dart package
flutter pub add flutter_rust_bridge
# then run this instead of "flutter web -d chrome"
dart run flutter_rust_bridge:serve
# or install globally
dart pub global activate flutter_rust_bridge
flutter_rust_bridge_serve
```

## Limitations of WASM

Running code on the Web entails several restrictions on the kinds of code that
can be executed. Please refer to [Limitations of WASM](../../../miscellaneous/wasm-limitations)
to see if your code is compatible with WASM.

[wasm-pack]: https://rustwasm.github.io/wasm-pack/
