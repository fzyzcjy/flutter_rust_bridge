# Rust runtime

## Customize handler

By default, a `FLUTTER_RUST_BRIDGE_HANDLER` instance is automatically generated,
and you can see it at `frb_generated.rs`.
However, you can provide your own `FLUTTER_RUST_BRIDGE_HANDLER` lazy-static instance.
If your instance is detected, the generator will not generate one, but will use your version.

The handler is the central entrypoint to handle calls between Rust and Dart,
therefore please visit the API of the `Handler` trait for more details.

Some typical scenarios are:

* [Inspection](../../how-to/inspect)
* [Report errors](../../how-to/report-error)

### States in handler

If you are only using the provided handler and executor (with your own arguments),
you can ignore this section. Only read it when you want to create your brand new handler.

It is usually a good idea to avoid coupling the handler with the specific Dart side isolate.
For example, suppose whenever we call `RustLib.init()` on Dart side,
we create a Dart isolate port and let the Rust side save and use it.
Then, during `flutter test`s, multiple Dart test files will be executed concurrently,
but (from my experiments) they all share one single Rust side.
Then, the one Rust side will be initialized multiple times with different Dart ports,
and it causes confusion and bugs.
Similar things may happen if you are using multiple isolates in your Dart program,
or when you hot-restart the Dart side.

## Customize WASM initialization code

By default, this library injects its own initialization code to facilitate panic information recovery
using [`console_error_panic_hook`](https://lib.rs/crates/console_error_panic_hook).
If you would like to run some initialization code for WASM, e.g. to set up logging libraries,
specify `default-features = false` in Cargo.toml:

```toml
flutter_rust_bridge = { version = "..", default-features = false, features = [..] }
```

The `wasm-start` feature governs this behavior and is enabled by default.
