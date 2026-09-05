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

- Web streams and Dart callbacks share one `BroadcastChannel` receiver per Dart context. Initialization confirms that this channel can receive messages before running Rust initializers.
- Each stream has a logical port name and a local `MessageChannel` queue. Closing a receive port removes its logical name and closes both local endpoints.
- Named and ordinary local ports share the same channel implementation. Each stream subscription independently tracks its received values and pending close.
- String handles contain the physical channel name and logical port name. Each Rust thread reuses its own BroadcastChannel sender, including custom thread pools and nested workers. Sending does not depend on the parent worker's event loop.
- The Dart context delivers named messages to the matching local `MessagePort`, which queues messages until its listener starts. This adds one structured clone on the Dart context.
- The last stream sink reports the total number of successfully sent values when it closes. Dart delivers the close only after receiving that many values, even when sink clones run in different workers. Values retain each worker's sending order; there is no cross-worker data reordering.
- Physical channels remain alive for the context's lifetime. Creating or closing a stream does not create or close a physical BroadcastChannel. Only initialization readiness probes are repeated; stream data is never retried.
- Named ports remain in the Dart context; ordinary transferable ports and buffers retain their existing transfer semantics. Update the Dart and Rust runtimes together because named port delivery is a paired runtime protocol.
