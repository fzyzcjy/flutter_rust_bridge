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

## Web stream initialization

- A Web worker can run before the browser has registered a newly created stream's `BroadcastChannel` receiver.
- Before dispatching work that follows channel creation, the Web thread pool confirms registration with a same-context `BroadcastChannel` round trip. Work without pending channel registrations dispatches immediately.
- Only registration probes are retried. Stream data and close messages are sent once, so applications can emit their first values immediately without adding sleeps.
