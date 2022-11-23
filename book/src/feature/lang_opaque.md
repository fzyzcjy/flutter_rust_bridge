# Arbitrary types (opaque)

Any Rust type, even if it is not supported using features of this library, can be used in Dart. This is done by wrapping it with `Opaque`.

The Dart opaque objects should be disposed *manually*, though it will also be disposed when it is GCed, that is discouraged, due to [suggestions by Dart team](https://github.com/fzyzcjy/flutter_rust_bridge/issues/775#issuecomment-1274635037). Think of it just like a lot of Flutter objects that we are familiar with, such as `ui.Image` - we have to manually dispose them as well.

## Example

Rust:

```rust,noplayground
struct ArbitraryData { ... }
pub fn use_opaque(a: Opaque<ArbitraryData>) { ... }
pub fn even_use_locks(b: Opaque<Mutex<ArbitraryData>) -> Opaque<RwLock<ArbitraryData>> { ... }
enum AnEnumContainingOpaque { Hello(Opaque<ArbitraryData>), World(i32) }
...
```

And use it in Dart:

```dart
var opaque = await api.functionThatCreatesSomeOpaqueData();
await api.functionThatUsesSomeOpaqueData(opaque);
opaque.dispose();
```

## Implementation details

As for how it is implemented as well as the design towards safety, please refer to [this doc](../contributing/opaque_type_safety.md).