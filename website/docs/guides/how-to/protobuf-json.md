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
