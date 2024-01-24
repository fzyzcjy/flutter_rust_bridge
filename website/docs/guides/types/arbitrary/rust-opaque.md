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

## Trait objects

Trait objects can be put behind opaque pointers. For example, this declaration can
be used across the FFI border:

```rust
pub struct DebugWrapper(pub RustOpaque<Box<dyn Debug>>);

// creating a DebugWrapper using the opaque_dyn macro
let wrap = DebugWrapper(opaque_dyn!("foobar"));
// it's possible to name it directly
pub struct DebugWrapper2(pub RustOpaque<Box<dyn Debug + Send + Sync + UnwindSafe + RefUnwindSafe>>);
```

## Naming the inner type

When an `RustOpaque<T>` is transformed into a Dart type, T's string
representation undergoes some transformations to become a valid Dart type:
- Rust keywords (dyn, 'static, etc.) are automatically removed.
- ASCII alphanumerics are kept, all other characters are ignored.

## Implementation details

As for how it is implemented as well as the design towards safety,
please refer to [this doc](../../contributing/submodules/rust-opaque)