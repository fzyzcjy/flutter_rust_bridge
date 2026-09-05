# DCO Codec

## More about function call example

Continue from the CST codec function call story,
the user's Rust function is already executed,
and our job here is to pass the return value back to Dart.

1. The return value, a `String`, is posted to the Dart side. It is done by the Dart-provided API, [`Dart_PostCObject`](https://github.com/dart-lang/sdk/blob/fd0d3b254690007d0ebc84175f30fa7d7491ec3e/runtime/include/dart_native_api.h#L124), which let us provide C structs and it will automatically become Dart data on the other side. We use the Rust-safe wrapper `allo-isolate` for it. We deliberately choose this, because this enables Dart code to be _async_ instead of sync.
2. On the Dart side, we now see some Dart objects (indeed "_Dart wire data_"). We use functions like `_wire2api_SomeType` to convert it to the final "_Dart api data_". Notice this "wire2api" is on _Dart_ side, so it means "_Dart_ wire data to _Dart_ api data", and is different from the one above which is for Rust. For example, since `Dart_PostCObject` does not provide a way to construct arbitrary structs(classes), we have to pass Rust structs as lists, and use the `wire2api` to convert them to corresponding Dart classes.
3. The final result value is provided as return value of the Dart function, `func`, that the user called just now. A function call finishes!

## Cross-scope communication in the browser

On Web platforms, browser messaging APIs replace native `SendPort`s.

**MessagePort** replaces `dart:ffi`'s `SendPort` and is created from `MessageChannel`. The Dart
thread creates a channel, keeps the receive port and transfers the send port to the workers.

```mermaid
sequenceDiagram
Dart ->> Rust: port2
Rust ->> Rust Worker: port2
Rust Worker ->> Dart: port2.postMessage
```

**BroadcastChannel** carries `StreamSink` values and Dart callbacks because wasm_bindgen keeps JS ports local to their context. Rust shares a string handle across threads instead of sharing a JS object. This also supports sinks stored in static variables and workers supplied by custom thread pools.

- Dart creates one physical BroadcastChannel receiver and confirms readiness during initialization.
- Each invocation receives a logical port name and a local MessagePort queue. Its string handle contains both the physical channel name and logical port name.
- Rust creates a sender for each message, posts `[logicalPortName, payload]`, and closes the sender before returning, including when posting fails.
- Dart dispatches the payload to the matching local queue. Closing a logical port removes it without closing the physical channel.
- Stream values and the final close carry a shared sequence number. Dart restores their sending order, including across sink clones. Failed sends return errors to Rust and settle their reserved sequence without delivering a value.

Keeping the physical receiver alive avoids racing receiver registration against every new stream. See [Web stream delivery](../../cross-platform/thread-pool.md#web-stream-delivery) for lifecycle and runtime compatibility requirements.

```mermaid
sequenceDiagram
Dart ->> Rust: physical channel name + logical port name
Rust ->> Rust Worker 1: string handle
Rust Worker 1 ->> Dart: BroadcastChannel.postMessage([port, value])
Rust ->> Rust Worker 2: string handle
Rust Worker 2 ->> Dart: BroadcastChannel.postMessage([port, value])
```

It is theoretically possible to have a one-to-one implementation of Isolate using only web primitives,
`BroadcastChannel`s and `Worker`s, but it remains to be seen how practical such an approach would be.
