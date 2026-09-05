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

## Web stream delivery

- Web streams and Dart callbacks use named `MessageChannel` endpoints owned by the Dart context. Closing a receive port removes its name and closes both endpoints.
- Rust workers forward named messages through their parent worker connection. The Dart context delivers them to the matching `MessagePort`, which queues messages until its listener starts.
- Stream values and close messages follow the same path. Delivery does not rely on `BroadcastChannel` registration, sleeps, retries, or message reordering.
- Named ports remain in the Dart context; ordinary transferable ports and buffers retain their existing transfer semantics. Update the Dart and Rust runtimes together because named port delivery is a paired runtime protocol.
