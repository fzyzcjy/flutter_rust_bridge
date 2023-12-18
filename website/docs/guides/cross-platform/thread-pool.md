# Thread pools

Thread pools also often do not run directly on the web,
due to limitations of WASM.
Here, we provide cross-platform thread pools.
For example, you can execute in a thread by:

```rust
FLUTTER_RUST_BRIDGE_HANDLER.thread_pool().execute(transfer!(|| {
    // your code executed in another thread
}));
```

The `transfer!` macro is there in case you need to move data to that thread
(which needs a bit of trick in WASM, encapsulated inside the macro).
We may improve the API in the future.