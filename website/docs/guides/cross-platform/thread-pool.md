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
- Each Rust worker has a direct `MessageChannel` connection to the Dart context, including workers created by other workers. Delivery remains possible while a parent worker executes synchronous Rust code.
- The Dart context delivers named messages to the matching local `MessagePort`, which queues messages until its listener starts. This routing adds one structured clone on the Dart context.
- The last stream sink reports the total number of successfully sent values when it closes. Dart delivers the close only after receiving that many values, even when sink clones run in different workers. Values retain each worker's sending order; there is no cross-worker data reordering.
- Delivery does not rely on `BroadcastChannel` registration, sleeps, or retries.
- Named ports remain in the Dart context; ordinary transferable ports and buffers retain their existing transfer semantics. Update the Dart and Rust runtimes together because named port delivery is a paired runtime protocol.
