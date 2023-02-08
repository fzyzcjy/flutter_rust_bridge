# Worker pool

When you call a Rust function with generated code from Dart side, it is executed inside a separate worker pool handled by flutter_rust_bridge. Flutter_rust_bridge internally creates a pool of 4 threads in non-WASM configuration and 4 workers in WASM configuration. Thanks to the pool, type of the return values in Dart is async `Future` which means heavy calculation in Rust does not block the user interface from responding.

However, if you think that this number of 4 is inappropriate for your project, you can choose a different option. By specifying features in `Cargo.toml`, you can optimize the number of threads or workers in the pool. Just include `features` key when adding flutter_rust_bridge as a dependency.

```toml
// Cargo.toml

[dependencies]
flutter_rust_bridge = { workspace = true, features = ["worker-count-max"] }
```

Currently available options related to worker pool are:

- `worker-count-single`: Uses 1 worker in the pool.
- `worker-count-max`: Uses all available logical cores.

Note that threadpool in non-WASM configuration provides true multithreaded parallelism while workerpool in WASM configuration is actually a singlethreaded JavaScript object.
