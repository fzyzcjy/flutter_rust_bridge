# Synchronous in Dart

If you really need to generate synchronous functions in Dart, you can ues the `SyncReturn<Vec<u8>>` as the return type.

We suggest only do this for very quick Rust functions, or the Dart UI will be blocked.