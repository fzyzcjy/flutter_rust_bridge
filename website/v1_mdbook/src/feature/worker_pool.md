# Worker pool

Note: You can customize [handlers](handler.md) and even completely get rid of the worker pool. The following doc only works for those who wants to use the default handler and thus pool.

When you call a Rust function with generated code from Dart side, it is executed inside a separate worker pool handled by flutter_rust_bridge. Thanks to the pool, type of the return values in Dart is async `Future` which means heavy calculation in Rust does not block the user interface from responding.

In non-WASM configuration, flutter_rust_bridge internally creates a pool of 4 threads. In WASM configuration, a pool with 4 web workers is used.

However, if you think that this number of 4 is inappropriate for your project, you can choose a different option. By specifying features in `Cargo.toml`, you can optimize the number of threads or workers in the pool. Just include `features` key when adding flutter_rust_bridge as a dependency.

```toml
// Cargo.toml

[dependencies]
flutter_rust_bridge = { workspace = true, features = ["worker-max"] }
```

Currently available options related to worker pool are:

- `worker-single`: Uses 1 worker in the pool.
- `worker-max`: Uses all available logical cores.

Note that both non-WASM and WASM configurations provide true multithreaded parallelism. They both utilize actual threads in logical cores. However, you do need to be aware of the limitations of WASM, such as inability to use shared memory, etc.

The options are experimental, and may change later. However, it should be trivial to migrate even if it changes.
