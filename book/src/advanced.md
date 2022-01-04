### Customize handler's behavior

By default, the `DefaultHandler` is used. You can implement your own `Handler` doing whatever you want. In order to do this, create a variable named `FLUTTER_RUST_BRIDGE_HANDLER` in the Rust input file (probably using `lazy_static`). You may not need to create a brand new struct implementing `Handler`, but instead, use the `SimpleHandler` and customize its generic arguments such as its `Executor`.

### Setup/init FFI call

If you want that feature, have a look at `FlutterRustBridgeSetupMixin` in the Dart side.

### Async in Rust

If you want to use async/await or return a Future type from your Rust functions, please refer [this documentation](https://github.com/fzyzcjy/flutter_rust_bridge/blob/master/docs/async_in_rust.md) for a detailed guide.

