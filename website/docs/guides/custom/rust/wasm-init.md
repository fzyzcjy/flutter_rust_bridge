# Customize WASM initialization code

By default, this library injects its own initialization code to facilitate panic information recovery
using [`console_error_panic_hook`](https://lib.rs/crates/console_error_panic_hook).
If you would like to run some initialization code for WASM, e.g. to set up logging libraries,
specify `default-features = false` in Cargo.toml:

```toml
flutter_rust_bridge = { version = "..", default-features = false, features = [..] }
```

The `wasm-start` feature governs this behavior and is enabled by default.

