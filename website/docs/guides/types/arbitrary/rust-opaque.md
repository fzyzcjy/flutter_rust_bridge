# Manual arbitrary Rust type

Usually the [automatic arbitrary Rust type](rust-auto-opaque) is sufficient,
and thus there is no need to manually write down anything.
However, if you somehow want lower-level control, here is the type for you.

In addition, there is `RustAutoOpaque<YourArbitraryType>`,
which is a simple type alias of `RustOpaque<RwLock<YourArbitraryType>>`.
This is convenient when you want to sometimes have [everything automatically done](rust-auto-opaque),
and sometimes have the manual low-level control.

## Example

Rust:

```rust
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

As for how it is implemented as well as the design towards safety,
please refer to [this doc](../../contributing/submodules/rust-opaque)