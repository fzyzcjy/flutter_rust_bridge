# Synchronous in Dart

If you need to generate synchronous functions in Dart, you can use `SyncReturn<T>` as the return type. It supports whatever types that we have described in other places, i.e. whatever types that async mode supports.

We suggest only do this for very quick Rust functions, or the Dart UI will be blocked.

If you are using the default handler, the behavior about threading is different from normal Rust functions. Normal function calls are executed in the [worker pool](worker_pool.md). However, Rust functions with return type of `SyncReturn<T>` are executed on the _main_ thread. This means that if a `SyncReturn<T>` function takes time, Dart UI will not be able to respond.
