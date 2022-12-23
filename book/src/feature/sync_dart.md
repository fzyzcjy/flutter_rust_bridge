# Synchronous in Dart

If you need to generate synchronous functions in Dart, you can use `SyncReturn<T>` as the return type. It supports whatever types that we have described in other places, i.e. whatever types that async mode supports.

We suggest only do this for very quick Rust functions, or the Dart UI will be blocked.
