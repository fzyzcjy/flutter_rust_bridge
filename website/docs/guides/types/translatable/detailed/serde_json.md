# serde_json

Codegen optionally supports [serde_json crate](https://docs.rs/serde_json) with feature `serde_json`.

| :crab: Rust              | :dart: Dart   |
| -----------              | -----------   |
| `serde_json::Value`      | `Object?`     |

The Rust `serde_json::Value` type maps to Dart's `Object?`, which can be any JSON-compatible value: `null`, `bool`, `num`, `String`, `List<dynamic>`, or `Map<String, dynamic>`.

## Setup

Add the feature to your Rust dependencies:

```toml
[dependencies]
flutter_rust_bridge = { features = ["serde_json"] }
serde_json = "1.0"
```

## Example

```rust
pub fn process_json(data: serde_json::Value) -> serde_json::Value {
    data
}
```

```dart
final result = await processJson(data: {'key': 'value', 'count': 42});
print(result); // {key: value, count: 42}
```

:bulb: Implementation detail: `serde_json::Value` is serialized as a JSON string across the FFI boundary, then decoded on the Dart side using `jsonDecode`/`jsonEncode`.
