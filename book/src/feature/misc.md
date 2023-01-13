# Miscellaneous

## Separate generated definitions from implementations

The generated `bridge_generated.dart` by default contains definitions of the APIs as well as the implementations. With the flag `--dart-decl-output`, the two can be separated, and the definitions will not contain anything like `dart:ffi`.

A command example as follow:

```shell
flutter_rust_bridge_codegen .. --dart-decl-output <DECL>
```

where `DECL` is the path to the common class/function declarations file.
For example, if you emit your Dart bridge to `lib/bridge_generated.dart`,
you can put the declarations file at `lib/bridge_definitions.dart`


By default this will create new file:

```
├── lib
│   ├── bridge_definitions.dart
```

More information: [#298](https://github.com/fzyzcjy/flutter_rust_bridge/issues/298).

## Injecting WASM initialization code

By default, this library injects its own initialization code to facilitate panic information recovery
using [`console_error_panic_hook`](https://lib.rs/crates/console_error_panic_hook).
If you would like to run some initialization code for WASM, e.g. to set up logging libraries,
specify `default-features = false` in Cargo.toml:

```toml
flutter_rust_bridge = { version = "..", default-features = false, features = [..] }
```

The `wasm-start` feature governs this behavior and is enabled by default.
