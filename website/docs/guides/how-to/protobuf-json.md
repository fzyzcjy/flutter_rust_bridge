# Protobuf / JSON / etc

If you want to use Protobuf, JSON, or whatever serialization methods, it is also quite easy:
Since `flutter_rust_bridge` supports complicated types, it surely supports the simple `Vec<u8>`/`String`.

For example, the code below uses JSON to serialize the arguments/results:

```dart
// Dart
var result = jsonDecode(await api.f(a: jsonEncode({"x": [100, 200, "what"], "y": "hello"})));
```

```rust
// Rust
pub fn f(a: String) -> Result<String> {
    let arg = serde_json::from_str(&a)?;
    Ok(json!({"some": "result", "is": [42]}).to_string())
}
```

If you want to even automatically generate those serialization calls (`jsonEncode`, `jsonDecode`, etc),
feel free to create an issue to tell me! (I usually reply quickly within hours)

## Drawbacks

However, it is generally preferred to avoid Protobuf/JSON,
and use the flutter_rust_bridge features directly.

On one hand, by doing so, we can use all features that flutter_rust_bridge provides,
which is a long list ;)

On the other hand, for example,
please refer to https://github.com/flutter/flutter/issues/60758
to see why the official Flutter chooses to write a custom serialization protocol,
instead of using Protobuf.
(P.S. In flutter_rust_bridge, the SSE codec is serialization-based which is like the Flutter protocol in that text,
and the CST/DCO codecs mimic how humans write C glue code in the standard way.
Both can be freely configured and used.)
