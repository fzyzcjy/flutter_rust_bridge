# Synchronous in Dart

If you need to generate synchronous functions in Dart, you can use `SyncReturn<T>` as the return type.

We suggest only do this for very quick Rust functions, or the Dart UI will be blocked.