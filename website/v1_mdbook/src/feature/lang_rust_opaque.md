# Arbitrary Rust types (opaque)

On one hand, any Rust type, even if it is not supported using features of this library, can be used in Dart. This is done by wrapping it with `RustOpaque`.

The Rust opaque objects in Dart should be disposed *manually*, though it will also be disposed when it is GCed, that is discouraged, due to [suggestions by Dart team](https://github.com/fzyzcjy/flutter_rust_bridge/issues/775#issuecomment-1274635037). Think of it just like a lot of Flutter objects that we are familiar with, such as `ui.Image` - we have to manually dispose them as well.

Different from non-opaque types, opaque types are not copied/moved/reconstructed at all. For example, if you pass around `RwLock<Mutex<ArbitraryData>` in arguments and return values, you will get the exact *same* `RwLock<ArbitraryData>` object.

## Example `RustOpaque` 

Rust:

```rust,noplayground
struct ArbitraryData { ... }
pub fn use_opaque(a: RustOpaque<ArbitraryData>) { ... }
pub fn even_use_locks(b: RustOpaque<Mutex<ArbitraryData>) -> RustOpaque<RwLock<ArbitraryData>> { ... }
enum AnEnumContainingOpaque { Hello(RustOpaque<ArbitraryData>), World(i32) }
...
```

And use it in Dart:

```dart
var opaque = await api.functionThatCreatesSomeOpaqueData();
await api.functionThatUsesSomeOpaqueData(opaque);
opaque.dispose();
```

## Implementation details

As for how it is implemented as well as the design towards safety, please refer to [this doc](../contributing/rust_opaque_type_safety.md)