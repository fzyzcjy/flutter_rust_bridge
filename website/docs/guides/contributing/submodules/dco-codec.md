# DCO Codec

## More about function call example

Continue from the CST codec function call story,
the user's Rust function is already executed,
and our job here is to pass the return value back to Dart.

1. The return value, a `String`, is posted to the Dart side. It is done by the Dart-provided API, [`Dart_PostCObject`](https://github.com/dart-lang/sdk/blob/fd0d3b254690007d0ebc84175f30fa7d7491ec3e/runtime/include/dart_native_api.h#L124), which let us provide C structs and it will automatically become Dart data on the other side. We use the Rust-safe wrapper `allo-isolate` for it. We deliberately choose this, because this enables Dart code to be _async_ instead of sync.
2. On the Dart side, we now see some Dart objects (indeed "_Dart wire data_"). We use functions like `_wire2api_SomeType` to convert it to the final "_Dart api data_". Notice this "wire2api" is on _Dart_ side, so it means "_Dart_ wire data to _Dart_ api data", and is different from the one above which is for Rust. For example, since `Dart_PostCObject` does not provide a way to construct arbitrary structs(classes), we have to pass Rust structs as lists, and use the `wire2api` to convert them to corresponding Dart classes.
3. The final result value is provided as return value of the Dart function, `func`, that the user called just now. A function call finishes!

## Cross-scope communication in the browser

On Web platforms, for lack of a proper `SendPort` there exists replacements from `dart:html`.

**MessagePort** replaces `dart:ffi`'s `SendPort` and is created from `MessageChannel`. The Dart
thread creates a channel, keeps the receive port and transfers the send port to the workers.

```mermaid
sequenceDiagram
Dart ->> Rust: port2
Rust ->> Rust Worker: port2
Rust Worker ->> Dart: port2.postMessage
```

**BroadcastChannel** carries named `StreamSink` messages between workers, because wasm_bindgen keeps JavaScript ports in a scope that cannot be shared with other threads.

- Dart creates one persistent broadcast receiver per library and verifies a round trip before starting Rust. Its name includes a cryptographically generated 128-bit nonce.
- Each sink has a distinct logical name and a local `MessageChannel`. Rust receives a handle containing the shared receiver name and the logical name.
- Workers open senders for the shared receiver and post `[logicalName, payload]`. Dart routes each payload to its local message port. Worker senders are reused within the current microtask and closed afterward.
- Closing a sink removes its logical route and closes its local ports without closing the shared receiver.
- Stream values and close frames share a sequence counter across Rust sink clones.
- Dart buffers out-of-order frames and delivers only consecutive sequence numbers, so an early close cannot discard preceding values.
- A rejected payload is followed by an empty frame for its reserved sequence number. If that frame also fails, the final sink drop sends a transport-error frame instead of a normal close.
- This framing applies only to stream broadcast channels; ordinary message ports retain their original payloads.

- Stream values and close frames share a sequence counter across Rust sink clones.
- Dart buffers out-of-order frames and delivers only consecutive sequence numbers, so an early close cannot discard preceding values.
- A rejected payload is followed by an empty frame for its reserved sequence number. If that frame also fails, the final sink drop sends a transport-error frame instead of a normal close.
- This framing applies only to stream broadcast channels; ordinary message ports retain their original payloads.

```mermaid
sequenceDiagram
Dart ->> Dart: Verify shared receiver readiness
Dart ->> Rust: sharedReceiverName/logicalName
Rust ->> Rust Worker 1: sharedReceiverName/logicalName
Rust Worker 1 ->> Dart: postMessage([logicalName, frame])
Dart ->> Dart: Route frame to the sink's local MessagePort
Rust ->> Rust Worker 2: sharedReceiverName/logicalName
Rust Worker 2 ->> Dart: postMessage([logicalName, frame])
Dart ->> Dart: Route frame to the sink's local MessagePort
```

It is theoretically possible to have a one-to-one implementation of Isolate using only web primitives,
`BroadcastChannel`s and `Worker`s, but it remains to be seen how practical such an approach would be.
