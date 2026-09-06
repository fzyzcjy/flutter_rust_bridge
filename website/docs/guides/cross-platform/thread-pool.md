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

## Web message delivery

- `RustLib.init()` waits until its shared broadcast receiver has received a readiness message before starting Rust.
- Each named port uses a local `MessageChannel`; worker messages are routed through the established receiver, so creating a stream does not require registering a new cross-worker receiver.
- Stream values and the final close message carry shared sequence numbers; Dart restores their sending order before decoding them.
- Initialization fails if readiness is not confirmed within ten seconds; callers can retry initialization after resolving the failure.
- The receiver is scoped to the Dart library instance and lives for that instance's lifetime. Closing a stream releases its local ports, not the shared receiver.
