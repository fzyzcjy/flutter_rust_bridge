# Synchronous in Dart

If you really need to generate synchronous functions in Dart, you can use the `SyncReturn<Vec<u8>>` as the return type.

We suggest only do this for very quick Rust functions, or the Dart UI will be blocked.

Currently, due to the lack of need, the only type supported is `Vec<u8>`, and the workaround of using other types is by using a serialization approach such as JSON or Protobuf. Notice that this is *only needed* in *this* very tiny part, and 99% of `flutter_rust_bridge` does not need this bare-matel approach. Moreover, please open an issue if you need other types.