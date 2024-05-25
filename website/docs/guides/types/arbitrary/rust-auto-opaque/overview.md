# Overview

This feature, sometimes called `RustAutoOpaque` throughout the documentation,
allows arbitrary Rust type to be used without manual intervention,
by representing arbitrary Rust object as (smart) pointers in Dart.

Different from non-opaque types, opaque types are not copied/moved/reconstructed at all.
For example, if you pass around `RwLock<Mutex<ArbitraryData>` in arguments and return values,
you will get the exact *same* `RwLock<ArbitraryData>` object.

## Example

Suppose you have a type that is not encodable:

```rust
pub struct MyNonEncodableType {
    // e.g., a temporary directory, a file descriptor, a native resource, a lock, a channel, ...
    sample_non_encodable_field: tempdir::TempDir,
}
```

Then you can have Rust functions and methods on it.
Most, if not all, features of flutter_rust_bridge are supported,
and here are a few examples:

```rust
pub fn create() -> MyNonEncodableType { ... }

pub fn consume(obj: MyNonEncodableType) { ... }

pub fn borrow(obj: &MyNonEncodableType) { ... }

pub fn mutable_borrow(obj: &mut MyNonEncodableType) { ... }

impl MyNonEncodableType {
    // Or `self`, `&mut self`
    pub fn methods_on_it(&self) { ... }
}
```

They can be called in Dart:

```dart
var object = await create();
await borrow(object);
await mutable_borrow(object);
await consume(object);
```

P.S. As for how it is implemented as well as the design towards safety,
please refer to [this doc](../../../contributing/submodules/rust-opaque)
