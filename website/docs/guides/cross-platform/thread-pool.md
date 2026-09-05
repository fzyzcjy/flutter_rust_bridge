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
- Named and ordinary local ports share the same channel implementation. Each stream subscription independently buffers out-of-order frames until their predecessors arrive.
- String handles contain the physical channel name and logical port name. Rust creates a BroadcastChannel sender for each message, including in custom thread pools and nested workers. Sending does not depend on the parent worker's event loop.
- The Dart context delivers named messages to the matching local `MessagePort`, which queues messages until its listener starts. This adds one structured clone on the Dart context.
- Sink clones share a sequence counter. Values and the final close carry sequence numbers, and Dart delivers them in that order, including when BroadcastChannel delivers frames out of order.
- A failed payload send still returns an error to Rust. An empty frame settles its reserved sequence number without delivering a value. If that frame also fails, further sends are rejected and the last sink reports transport failure instead of a normal close.
- The physical receiver remains alive for the Dart context's lifetime. Rust closes each sender before returning from the send, including on failure, so idle workers do not remain broadcast recipients. Creating or closing a stream does not register another physical receiver. Only initialization readiness probes are repeated; stream data is never retried.
- Named ports remain in the Dart context; ordinary transferable ports and buffers retain their existing transfer semantics. Update the Dart and Rust runtimes together because named port delivery is a paired runtime protocol.
